// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::values::BasicValueEnum;

use crate::integration::hostapi::HostAPI;
use crate::ir::cfg::Type;
use crate::ir_codegen::encoding::BUILTIN_CO_CALL_AUTO_REVERT;
use crate::ir_codegen::traits::{BaseTypeMethods, BuilderMethods};
use crate::ir_codegen::{
    builtin_constants::{
        Q_VEC_DATA_FUNC_NAME, Q_VEC_NEW_FUNC_NAME, Q_VEC_SETDATA_FUNC_NAME, Q_VEC_SIZE_FUNC_NAME,
        VECTOR_NEW_FUNC_NAME,
    },
    context::IR2LLVMCodeGenContext,
    encoding::MALLOC_FUNC_NAME,
};

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    pub fn build_builtin_require(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let ll_func = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();
        let cond = *params.get(0).unwrap();
        let msg_vec = *params.get(1).unwrap();
        let (msg, length) = (self.vector_bytes(msg_vec), self.vector_len(msg_vec));
        let then_block = self.llvm_context.append_basic_block(ll_func, "");
        let else_block = self.llvm_context.append_basic_block(ll_func, "");
        let end_block = self.llvm_context.append_basic_block(ll_func, "");
        self.cond_br(cond, then_block, else_block);
        self.builder.position_at_end(then_block);
        self.br(end_block);
        self.builder.position_at_end(else_block);
        self.build_void_call("builtin_abort", &[msg.into(), length.into()]);
        self.br(end_block);
        self.builder.position_at_end(end_block);
        self.i32_value(1)
    }

    pub fn build_builtin_encode_params(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let (bytes, len) = self.data_stream_encode(params_ty, params, true);
        let size = self.i32_value(1);
        self.build_call(VECTOR_NEW_FUNC_NAME, &[len, size, bytes])
    }

    pub fn build_builtin_print_type(
        &self,
        _params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let arg_runtime_class_offset = self
            .class_generator
            .borrow()
            .intern_ir_runtime_class(params_ty.get(0).unwrap());
        let offset_value = self.i32_value(arg_runtime_class_offset as u64);
        self.build_void_call("ir_builtin_print_type", &[offset_value]);
        self.i32_value(1)
    }

    pub fn build_builtin_rlp_encode(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let val_ty = params_ty.get(0).unwrap();
        let val = *params.get(0).unwrap();
        let arg_runtime_class_offset = self
            .class_generator
            .borrow()
            .intern_ir_runtime_class(params_ty.get(0).unwrap());
        let offset_value = self.i32_value(arg_runtime_class_offset as u64);
        let void_ptr = if val_ty.is_reference_type() || val_ty.is_string() {
            self.ptr_cast(val, self.i8_ptr_type())
        } else {
            let val_ptr = self.builder.build_alloca(self.llvm_type(val_ty), "");
            self.builder.build_store(val_ptr, val);
            self.ptr_cast(val_ptr.into(), self.i8_ptr_type())
        };
        self.build_call("ir_builtin_rlp_encode", &[offset_value, void_ptr])
    }

    pub fn build_builtin_rlp_decode(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let val = *params.get(0).unwrap();
        let ret_runtime_class_offset = self.class_generator.borrow().intern_ir_runtime_class(ret);
        let offset_value = self.i32_value(ret_runtime_class_offset as u64);
        let void_ptr = self.build_call("ir_builtin_rlp_decode", &[offset_value, val]);
        if ret.is_reference_type() || ret.is_string() {
            self.ptr_cast(void_ptr, self.llvm_type(ret))
        } else {
            let val_ptr = self.ptr_cast(void_ptr, self.ptr_type_to(self.llvm_type(ret)));
            self.builder.build_load(val_ptr.into_pointer_value(), "")
        }
    }

    pub fn build_builtin_json_encode(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let val_ty = params_ty.get(0).unwrap();
        let val = *params.get(0).unwrap();
        let arg_runtime_class_offset = self
            .class_generator
            .borrow()
            .intern_ir_runtime_class(params_ty.get(0).unwrap());
        let offset_value = self.i32_value(arg_runtime_class_offset as u64);
        let void_ptr = if val_ty.is_reference_type() || val_ty.is_string() {
            self.ptr_cast(val, self.i8_ptr_type())
        } else {
            let val_ptr = self.builder.build_alloca(self.llvm_type(val_ty), "");
            self.builder.build_store(val_ptr, val);
            self.ptr_cast(val_ptr.into(), self.i8_ptr_type())
        };
        self.build_call("ir_builtin_json_encode", &[offset_value, void_ptr])
    }

    pub fn build_builtin_json_decode(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let val = *params.get(0).unwrap();
        let ret_runtime_class_offset = self.class_generator.borrow().intern_ir_runtime_class(ret);
        let offset_value = self.i32_value(ret_runtime_class_offset as u64);
        let void_ptr = self.build_call("ir_builtin_json_decode", &[offset_value, val]);
        if ret.is_reference_type() || ret.is_string() {
            self.ptr_cast(void_ptr, self.llvm_type(ret))
        } else {
            let val_ptr = self.ptr_cast(void_ptr, self.ptr_type_to(self.llvm_type(ret)));
            self.builder.build_load(val_ptr.into_pointer_value(), "")
        }
    }

    pub fn build_builtin_ssz_encode(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let val_ty = params_ty.get(0).unwrap();
        let val = *params.get(0).unwrap();
        let arg_runtime_class_offset = self
            .class_generator
            .borrow()
            .intern_ir_runtime_class(params_ty.get(0).unwrap());
        let offset_value = self.i32_value(arg_runtime_class_offset as u64);
        let void_ptr = if val_ty.is_reference_type() || val_ty.is_string() {
            self.ptr_cast(val, self.i8_ptr_type())
        } else {
            let val_ptr = self.builder.build_alloca(self.llvm_type(val_ty), "");
            self.builder.build_store(val_ptr, val);
            self.ptr_cast(val_ptr.into(), self.i8_ptr_type())
        };
        self.build_call("ir_builtin_ssz_encode", &[offset_value, void_ptr])
    }

    pub fn build_builtin_ssz_decode(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let val = *params.get(0).unwrap();
        let ret_runtime_class_offset = self.class_generator.borrow().intern_ir_runtime_class(ret);
        let offset_value = self.i32_value(ret_runtime_class_offset as u64);
        let void_ptr = self.build_call("ir_builtin_ssz_decode", &[offset_value, val]);
        if ret.is_reference_type() || ret.is_string() {
            self.ptr_cast(void_ptr, self.llvm_type(ret))
        } else {
            let val_ptr = self.ptr_cast(void_ptr, self.ptr_type_to(self.llvm_type(ret)));
            self.builder.build_load(val_ptr.into_pointer_value(), "")
        }
    }

    pub fn build_datastream_decode(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let val = *params.get(0).unwrap();

        let size = self.build_call(Q_VEC_SIZE_FUNC_NAME, &[val]);
        let data = self.build_call(Q_VEC_DATA_FUNC_NAME, &[val]);

        let (void_ptr, _) = self.data_stream_decode(ret, data, self.i32_value(0), size, "");

        if ret.is_reference_type() || ret.is_string() {
            void_ptr.into()
        } else {
            self.builder.build_load(void_ptr, "")
        }
    }

    pub fn build_builtin_cocall(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let co_name = *params.get(0).unwrap();
        let method_name = *params.get(1).unwrap();
        let (bytes, len) = self.data_stream_encode(&params_ty[2..], &params[2..], false);

        self.build_void_call(
            BUILTIN_CO_CALL_AUTO_REVERT,
            &[
                self.vector_bytes(co_name).into(),
                self.vector_len(co_name).into(),
                self.vector_bytes(method_name).into(),
                self.vector_len(method_name).into(),
                bytes,
                len,
            ],
        );

        // TODO: if co_call return not zero, call hostapi revert

        // If the function return is not void,
        // Call HostAPI 'get_call_result_length' and HostAPI 'get_call_result' to get the return.
        if !ret.is_void() {
            let length = self
                .build_call(HostAPI::GetCallResultLength.name(), &[])
                .into_int_value();
            let data = self
                .build_call(MALLOC_FUNC_NAME, &[length.into()])
                .into_pointer_value();
            let data = unsafe {
                self.builder
                    .build_in_bounds_gep(data, &[self.native_i8(0)], "")
                    .into()
            };
            self.build_void_call(HostAPI::GetCallResult.name(), &[data]);
            let (ptr, _) = self.data_stream_decode(ret, data, self.i32_value(1), length.into(), "");
            if ret.is_reference_type() || ret.is_string() {
                ptr.into()
            } else {
                self.builder.build_load(ptr, "")
            }
        } else {
            self.i32_value(1)
        }
    }

    pub fn build_builtin_get_call_result(
        &self,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let length = self.build_call(HostAPI::GetCallResultLength.name(), &[]);
        let data = self
            .build_call(MALLOC_FUNC_NAME, &[length])
            .into_pointer_value();
        let data = unsafe {
            self.builder
                .build_in_bounds_gep(data, &[self.native_i8(0)], "")
                .into()
        };

        self.build_void_call(HostAPI::GetCallResult.name(), &[data]);

        let size = self.i32_value(1);
        let vector_mode = self.i32_value(1);
        let vector_ptr = self.build_call(Q_VEC_NEW_FUNC_NAME, &[length, size, vector_mode]);
        self.build_call(Q_VEC_SETDATA_FUNC_NAME, &[vector_ptr, data, length]);

        vector_ptr
    }

    pub fn build_builtin_call_sender(
        &self,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let length = self.build_call(HostAPI::GetCallSenderLength.name(), &[]);
        let addr = self.build_call(MALLOC_FUNC_NAME, &[length]);
        self.build_void_call(HostAPI::GetCallSender.name(), &[addr]);
        let size = self.i32_value(1);
        self.build_call(VECTOR_NEW_FUNC_NAME, &[length, size, addr])
    }

    pub fn build_builtin_call_this_contract(
        &self,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let length = self.build_call(HostAPI::GetCallContractLength.name(), &[]);
        let addr = self.build_call(MALLOC_FUNC_NAME, &[length]);
        self.build_void_call(HostAPI::GetCallContract.name(), &[addr]);
        let size = self.i32_value(1);
        self.build_call(VECTOR_NEW_FUNC_NAME, &[length, size, addr])
    }

    pub fn build_builtin_call_op_contract(
        &self,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let length = self.build_call(HostAPI::GetOpContractLength.name(), &[]);
        let addr = self.build_call(MALLOC_FUNC_NAME, &[length]);
        self.build_void_call(HostAPI::GetOpContract.name(), &[addr]);
        let size = self.i32_value(1);
        self.build_call(VECTOR_NEW_FUNC_NAME, &[length, size, addr])
    }

    pub fn build_builtin_tx_sender(
        &self,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let length = self.build_call(HostAPI::GetTxSenderLength.name(), &[]);
        let addr = self.build_call(MALLOC_FUNC_NAME, &[length]);
        self.build_void_call(HostAPI::GetTxSender.name(), &[addr]);
        let size = self.i32_value(1);
        self.build_call(VECTOR_NEW_FUNC_NAME, &[length, size, addr])
    }

    pub fn build_builtin_tx_hash(
        &self,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let length = self.build_call(HostAPI::GetTxHashLength.name(), &[]);
        let addr = self.build_call(MALLOC_FUNC_NAME, &[length]);
        self.build_void_call(HostAPI::GetTxHash.name(), &[addr]);
        let size = self.i32_value(1);
        self.build_call(VECTOR_NEW_FUNC_NAME, &[length, size, addr])
    }

    pub fn build_builtin_block_random_seed(
        &self,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let length = self.i32_value(32);
        let addr = self.build_call(MALLOC_FUNC_NAME, &[length]);
        self.build_void_call(HostAPI::GetBlockRandomSeed.name(), &[addr]);
        let size = self.i32_value(1);
        let vector_mode: u64 = 1;
        let vector_ptr = self.build_call(
            Q_VEC_NEW_FUNC_NAME,
            &[length, size, self.i32_value(vector_mode)],
        );
        self.build_call(Q_VEC_SETDATA_FUNC_NAME, &[vector_ptr, addr, length]);

        vector_ptr
    }
}
