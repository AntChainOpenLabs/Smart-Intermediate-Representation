// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::values::BasicValueEnum;
use inkwell::values::FunctionValue;

use crate::integration::hostapi::HostAPI;
use crate::ir::cfg::Type;
use crate::ir::interface_type::PartialFuncNameBehavior;
use crate::ir::interface_type::{get_all_intrinsic_func_names, IntrinsicFuncName};
use crate::ir_codegen::common::global::{get_extend_context, has_extend_context};
use crate::ir_codegen::IR2LLVMCodeGenContext;

pub const ENTRY_NAME: &str = "entry";
mod builtin;
mod map;
mod math;
mod mycov;
mod storage;
mod vector;

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    pub fn is_runtime_abort(&self, ir_func: &IntrinsicFuncName) -> bool {
        matches!(
            ir_func.key,
            IntrinsicFuncName::IR_VECTOR_GET
                | IntrinsicFuncName::IR_VECTOR_AT
                | IntrinsicFuncName::IR_VECTOR_SLICE
                | IntrinsicFuncName::IR_VECTOR_SET
                | IntrinsicFuncName::IR_VECTOR_POP
                | IntrinsicFuncName::IR_VECTOR_TO_STR
                | IntrinsicFuncName::IR_STR_SPLIT
                | IntrinsicFuncName::IR_STR_JOIN
                | IntrinsicFuncName::IR_STR_REPLACE
                | IntrinsicFuncName::IR_STR_INSERT
        )
    }

    pub fn get_intrinsic_function_name(
        ir_func_key: &str, // &IntrinsicFuncName,
        params: &[Type],
        ret: &Type,
    ) -> String {
        let all_intrinsic_func_names = get_all_intrinsic_func_names();
        let found = all_intrinsic_func_names.get(ir_func_key);
        if found.is_none() {
            unreachable!("not supported intrinsic func key {ir_func_key}");
        }
        let ir_func = found.unwrap();
        let mut intrinsic_function_name = ir_func.apply_name();
        match ir_func.key {
            // No need for overloading
            IntrinsicFuncName::IR_VECTOR_CREATE_ITER
            | IntrinsicFuncName::IR_VECTOR_GET_NEXT
            | IntrinsicFuncName::IR_VECTOR_OBJ_KEY
            | IntrinsicFuncName::IR_VECTOR_DELETE
            | IntrinsicFuncName::IR_VECTOR_SLICE
            | IntrinsicFuncName::IR_MAP_CREATE_ITER
            | IntrinsicFuncName::IR_MAP_GET_NEXT => intrinsic_function_name,

            // redirect function name
            IntrinsicFuncName::IR_VECTOR_AT => IR2LLVMCodeGenContext::get_intrinsic_function_name(
                IntrinsicFuncName::IR_VECTOR_GET,
                params,
                ret,
            ),
            IntrinsicFuncName::IR_MAP_INSERT => IR2LLVMCodeGenContext::get_intrinsic_function_name(
                IntrinsicFuncName::IR_VECTOR_SET,
                params,
                ret,
            ),

            _ => {
                for val in params {
                    intrinsic_function_name =
                        format!("{intrinsic_function_name}_{}", val.func_sign_ty_str());
                }
                format!("{intrinsic_function_name}_{}", ret.func_sign_ty_str())
            }
        }
    }

    pub fn build_intrinsic_function<F>(
        &self,
        name: &str,
        params: &[Type],
        ret: &Type,
        is_runtime_abort: bool,
        intrinsic_builder: F,
    ) -> FunctionValue<'ctx>
    where
        F: Fn(&Self, &[BasicValueEnum<'ctx>], &[Type], &Type) -> BasicValueEnum<'ctx>,
    {
        let pre_bb = self.builder.get_insert_block();
        let func_ty = self.build_llvm_function_type(params, ret, is_runtime_abort);
        let func = self.module.add_function(name, func_ty, None);

        let entry = self.llvm_context.append_basic_block(func, ENTRY_NAME);
        self.builder.position_at_end(entry);

        let ret_val = intrinsic_builder(self, func.get_params().as_slice(), params, ret);
        if ret.is_void() {
            self.builder.build_return(None);
        } else {
            self.builder.build_return(Some(&ret_val));
        }

        if let Some(pre_bb) = pre_bb {
            self.builder.position_at_end(pre_bb);
        }

        func
    }

    pub fn add_or_get_intrinsic_function(
        &self,
        ir_func: &IntrinsicFuncName,
        params: &[Type],
        ret: &Type,
    ) -> FunctionValue<'ctx> {
        let intrinsic_function_name =
            IR2LLVMCodeGenContext::get_intrinsic_function_name(ir_func.key, params, ret);
        let mut intrinsics = self.intrinsics.borrow_mut();
        match intrinsics.get(&intrinsic_function_name) {
            Some(func) => *func,
            None => {
                let func = match ir_func.key {
                    IntrinsicFuncName::IR_VECTOR_SET => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_set,
                    ),
                    IntrinsicFuncName::IR_VECTOR_GET => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_get,
                    ),
                    IntrinsicFuncName::IR_VECTOR_CREATE_ITER => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_create_iter,
                    ),
                    IntrinsicFuncName::IR_VECTOR_GET_NEXT => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_get_next,
                    ),
                    IntrinsicFuncName::IR_VECTOR_OBJ_KEY => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_obj_key,
                    ),
                    IntrinsicFuncName::IR_VECTOR_OBJ_VALUE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_obj_value,
                    ),
                    IntrinsicFuncName::IR_VECTOR_PUSH => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_push,
                    ),
                    IntrinsicFuncName::IR_VECTOR_POP => {
                        self.module.get_function("qvector_poplast").unwrap()
                    }
                    IntrinsicFuncName::IR_VECTOR_AT => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_get,
                    ),
                    IntrinsicFuncName::IR_VECTOR_INSERT => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_insert,
                    ),
                    IntrinsicFuncName::IR_VECTOR_DELETE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_delete,
                    ),
                    IntrinsicFuncName::IR_VECTOR_LEN => {
                        self.module.get_function("qvector_size").unwrap()
                    }
                    IntrinsicFuncName::IR_VECTOR_CLEAR => {
                        self.module.get_function("qvector_clear").unwrap()
                    }
                    IntrinsicFuncName::IR_VECTOR_REVERSE => {
                        self.module.get_function("qvector_reverse").unwrap()
                    }
                    IntrinsicFuncName::IR_VECTOR_SLICE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_vector_slice,
                    ),
                    IntrinsicFuncName::IR_VECTOR_TO_STR => {
                        self.module.get_function("qvector_to_str").unwrap()
                    }
                    IntrinsicFuncName::IR_MAP_SET => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_map_set,
                    ),
                    IntrinsicFuncName::IR_MAP_GET => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_map_get,
                    ),
                    IntrinsicFuncName::IR_MAP_CREATE_ITER => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_map_create_iter,
                    ),
                    IntrinsicFuncName::IR_MAP_GET_NEXT => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_map_get_next,
                    ),
                    IntrinsicFuncName::IR_MAP_OBJ_KEY => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_map_obj_key,
                    ),
                    IntrinsicFuncName::IR_MAP_OBJ_VALUE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_map_obj_value,
                    ),
                    IntrinsicFuncName::IR_MAP_CONTAINS_KEY => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_map_contains_key,
                    ),
                    IntrinsicFuncName::IR_MAP_INSERT => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_map_set,
                    ),
                    IntrinsicFuncName::IR_MAP_DELETE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_map_delete,
                    ),
                    IntrinsicFuncName::IR_MAP_LEN => {
                        self.module.get_function("qhashtbl_size").unwrap()
                    }
                    IntrinsicFuncName::IR_MAP_CLEAR => {
                        self.module.get_function("qhashtbl_clear").unwrap()
                    }
                    IntrinsicFuncName::IR_STORAGE_PUSH => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_push,
                    ),
                    IntrinsicFuncName::IR_STORAGE_PUSH_EMPTY => self
                        .module
                        .get_function("builtin_storage_one_dimension_array_push_empty")
                        .unwrap(),
                    IntrinsicFuncName::IR_STORAGE_MULTIARRAY_PUSH_EMPTY => self
                        .module
                        .get_function("builtin_storage_multiarray_push_empty")
                        .unwrap(),
                    IntrinsicFuncName::IR_STORAGE_POP => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_pop,
                    ),
                    IntrinsicFuncName::IR_STORAGE_LEN => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_len,
                    ),
                    IntrinsicFuncName::IR_STORAGE_VERIFY_INDEX => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_verify_index,
                    ),
                    IntrinsicFuncName::IR_STORAGE_CONTAINS_KEY => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_contains_key,
                    ),
                    IntrinsicFuncName::IR_STORAGE_CONTAINS_ASSET => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_contains_asset,
                    ),
                    IntrinsicFuncName::IR_STORAGE_DELETE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_delete,
                    ),

                    IntrinsicFuncName::IR_STORAGE_MINT => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_mint,
                    ),
                    IntrinsicFuncName::IR_STORAGE_BURN => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_burn,
                    ),
                    IntrinsicFuncName::IR_STORAGE_DESTROY => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_destroy,
                    ),
                    IntrinsicFuncName::IR_STORAGE_GET_BALANCE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_get_balance,
                    ),
                    IntrinsicFuncName::IR_STORAGE_GET_TAG => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_get_tag,
                    ),
                    IntrinsicFuncName::IR_STORAGE_TRANSFER => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_transfer,
                    ),
                    IntrinsicFuncName::IR_STORAGE_SET_BSS => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_set_bss,
                    ),
                    IntrinsicFuncName::IR_STORAGE_GET_BSS => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_storage_get_bss,
                    ),
                    IntrinsicFuncName::IR_STORAGE_PATH_JOIN => self
                        .module
                        .get_function("builtin_storage_path_join")
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_ABORT => {
                        self.module.get_function("ir_builtin_abort").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_ASSERT => {
                        self.module.get_function("ir_builtin_abort").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_REQUIRE => self.build_intrinsic_function(
                        "ir_builtin_require",
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_require,
                    ),
                    IntrinsicFuncName::IR_BUILTIN_PRINT => {
                        self.module.get_function("ir_builtin_print").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_PRINT_TYPE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_print_type,
                    ),
                    IntrinsicFuncName::IR_BUILTIN_ADDRESS => {
                        self.module.get_function("ir_builtin_address").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_ENCODE_BASE64 => self
                        .module
                        .get_function("ir_builtin_encode_base64")
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_DECODE_BASE64 => self
                        .module
                        .get_function("ir_builtin_decode_base64")
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_ENCODE_HEX => {
                        self.module.get_function("ir_builtin_encode_hex").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_DECODE_HEX => {
                        self.module.get_function("ir_builtin_decode_hex").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_ENCODE_PARAMS => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_encode_params,
                    ),
                    IntrinsicFuncName::IR_BUILTIN_CALL_LOG => {
                        self.module.get_function("ir_builtin_call_log").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_SHA256 => {
                        self.module.get_function("ir_builtin_sha256").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_SM3 => {
                        self.module.get_function("ir_builtin_sm3").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_KECCAK256 => {
                        self.module.get_function("ir_builtin_keccak256").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_REVERT => {
                        self.module.get_function("builtin_revert").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_CO_CALL_DIRECTLY => {
                        self.module.get_function("builtin_co_call").unwrap()
                    }
                    IntrinsicFuncName::IR_BUILTIN_VERIFY_MYCRYPTO_SIGNATURE => self
                        .module
                        .get_function("ir_builtin_verify_mycrypto_signature")
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_ETH_SECP256K1_RECOVERY => self
                        .module
                        .get_function("ir_builtin_eth_secp256k1_recovery")
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_GET_ACCOUT => unimplemented!(),
                    IntrinsicFuncName::IR_BUILTIN_GET_CONTRACT => unimplemented!(),
                    IntrinsicFuncName::IR_BUILTIN_GET_ARTIFACT => unimplemented!(),
                    IntrinsicFuncName::IR_BUILTIN_ADD_COVERAGE_COUNTER => self
                        .module
                        .get_function("ir_builtin_add_coverage_counter")
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_COCALL => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_cocall,
                    ),
                    IntrinsicFuncName::IR_BUILTIN_GET_CALL_RESULT => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_get_call_result,
                    ),
                    IntrinsicFuncName::IR_BUILTIN_BLOCK_NUMBER => self
                        .module
                        .get_function(HostAPI::GetBlockNumber.name())
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_BLOCK_TIMESTAMP => self
                        .module
                        .get_function(HostAPI::GetBlockTimestamp.name())
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_BLOCK_RANDOM_SEED => self
                        .build_intrinsic_function(
                            &intrinsic_function_name,
                            params,
                            ret,
                            self.is_runtime_abort(ir_func),
                            Self::build_builtin_block_random_seed,
                        ),
                    IntrinsicFuncName::IR_BUILTIN_BLOCK_VERSION => unimplemented!(),
                    IntrinsicFuncName::IR_BUILTIN_TX_SENDER => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_tx_sender,
                    ),
                    IntrinsicFuncName::IR_BUILTIN_TX_HASH => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_tx_hash,
                    ),
                    IntrinsicFuncName::IR_BUILTIN_TX_INDEX => self
                        .module
                        .get_function(HostAPI::GetTxIndex.name())
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_TX_GAS_LIMIT => self
                        .module
                        .get_function(HostAPI::GetTxGasLimit.name())
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_TX_TIMESTAMP => self
                        .module
                        .get_function(HostAPI::GetTxTimestamp.name())
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_TX_NONCE => self
                        .module
                        .get_function(HostAPI::GetTxNonce.name())
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_CALL_SENDER => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_call_sender,
                    ),
                    IntrinsicFuncName::IR_BUILTIN_CALL_THIS_CONTRACT => self
                        .build_intrinsic_function(
                            &intrinsic_function_name,
                            params,
                            ret,
                            self.is_runtime_abort(ir_func),
                            Self::build_builtin_call_this_contract,
                        ),
                    IntrinsicFuncName::IR_BUILTIN_CALL_OP_CONTRACT => self
                        .build_intrinsic_function(
                            &intrinsic_function_name,
                            params,
                            ret,
                            self.is_runtime_abort(ir_func),
                            Self::build_builtin_call_op_contract,
                        ),
                    IntrinsicFuncName::IR_BUILTIN_CALL_GAS_LEFT => self
                        .module
                        .get_function(HostAPI::GetCallGasLeft.name())
                        .unwrap(),
                    IntrinsicFuncName::IR_BUILTIN_CALL_GAS_LIMIT => self
                        .module
                        .get_function(HostAPI::GetCallGasLimit.name())
                        .unwrap(),
                    IntrinsicFuncName::IR_STR_SPLIT => {
                        self.module.get_function("vector_split").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_LEN => {
                        self.module.get_function("vector_len").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_LOWER => {
                        self.module.get_function("vector_lower").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_UPPER => {
                        self.module.get_function("vector_upper").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_AT => self.module.get_function("vector_at").unwrap(),
                    IntrinsicFuncName::IR_STR_COUNT => {
                        self.module.get_function("vector_count").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_STARTSWITH => {
                        self.module.get_function("vector_startswith").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_ENDSWITH => {
                        self.module.get_function("vector_endswith").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_ISALNUM => {
                        self.module.get_function("vector_isalnum").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_ISALPHA => {
                        self.module.get_function("vector_isalpha").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_ISDIGIT => {
                        self.module.get_function("vector_isdigit").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_ISLOWER => {
                        self.module.get_function("vector_islower").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_ISUPPER => {
                        self.module.get_function("vector_isupper").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_ISSPACE => {
                        self.module.get_function("vector_isspace").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_STRIP => {
                        self.module.get_function("vector_strip").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_LSTRIP => {
                        self.module.get_function("vector_lstrip").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_RSTRIP => {
                        self.module.get_function("vector_rstrip").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_JOIN => {
                        self.module.get_function("vector_join").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_CONCAT => {
                        self.module.get_function("vector_concat").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_REPLACE => {
                        self.module.get_function("vector_replace").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_FIND => {
                        self.module.get_function("vector_find").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_SUBSTR => {
                        self.module.get_function("vector_substr").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_INSERT => {
                        self.module.get_function("vector_insert").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_TO_BYTES => {
                        self.module.get_function("vector_to_bytes").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_TO_I128 => {
                        self.module.get_function("ir_builtin_str_to_i128").unwrap()
                    }
                    IntrinsicFuncName::IR_STR_TO_U128 => {
                        self.module.get_function("ir_builtin_str_to_u128").unwrap()
                    }
                    IntrinsicFuncName::IR_BASE64_ENCODE => self
                        .module
                        .get_function("ir_builtin_encode_base64")
                        .unwrap(),
                    IntrinsicFuncName::IR_BASE64_DECODE => self
                        .module
                        .get_function("ir_builtin_decode_base64")
                        .unwrap(),
                    IntrinsicFuncName::IR_HEX_ENCODE => {
                        self.module.get_function("ir_builtin_encode_hex").unwrap()
                    }
                    IntrinsicFuncName::IR_HEX_DECODE => {
                        self.module.get_function("ir_builtin_decode_hex").unwrap()
                    }
                    IntrinsicFuncName::IR_SSZ_ENCODE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_ssz_encode,
                    ),
                    IntrinsicFuncName::IR_SSZ_DECODE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_ssz_decode,
                    ),
                    IntrinsicFuncName::IR_JSON_ENCODE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_json_encode,
                    ),
                    IntrinsicFuncName::IR_JSON_DECODE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_json_decode,
                    ),
                    IntrinsicFuncName::IR_RLP_ENCODE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_rlp_encode,
                    ),
                    IntrinsicFuncName::IR_RLP_DECODE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_builtin_rlp_decode,
                    ),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_BOOL => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_bool")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U8 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u8")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U16 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u16")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U32 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u32")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U64 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u64")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U128 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u128")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I8 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i8")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I16 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i16")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I32 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i32")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I64 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i64")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I128 => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i128")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STR => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_str")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_BOOLARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_boolarray")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U8ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u8array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U16ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u16array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U32ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u32array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U64ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u64array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_U128ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_u128array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I8ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i8array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I16ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i16array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I32ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i32array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I64ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i64array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_I128ARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_i128array")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRARRAY => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_strarray")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRBOOLMAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_strboolmap")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU8MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stru8map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU16MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stru16map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU32MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stru32map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU64MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stru64map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU128MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stru128map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI8MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stri8map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI16MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stri16map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI32MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stri32map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI64MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stri64map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI128MAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_stri128map")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRSTRMAP => self
                        .module
                        .get_function("ir_builtin_data_stream_encode_strstrmap")
                        .unwrap(),
                    IntrinsicFuncName::IR_DATASTREAM_DECODE => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_datastream_decode,
                    ),
                    IntrinsicFuncName::IR_MATH_ITOA => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_math_itoa,
                    ),
                    IntrinsicFuncName::IR_MATH_POW => self.build_intrinsic_function(
                        &intrinsic_function_name,
                        params,
                        ret,
                        self.is_runtime_abort(ir_func),
                        Self::build_math_pow,
                    ),

                    _ => {
                        let func_intrinsic = if has_extend_context() {
                            let ext_ctx = get_extend_context();
                            ext_ctx.find_ir_func_intrinsic_by_func_name(ir_func)
                        } else {
                            None
                        };
                        if let Some(func_intrinsic) = func_intrinsic {
                            let ext_ctx = get_extend_context();
                            ext_ctx.add_or_get_intrinsic_function(self, func_intrinsic, params, ret)
                        } else {
                            unreachable!("not found func intrinsic {}", ir_func.key);
                        }
                    }
                };
                intrinsics.insert(intrinsic_function_name, func);
                func
            }
        }
    }
}
