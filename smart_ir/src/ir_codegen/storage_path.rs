// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use inkwell::{
    values::{BasicValueEnum, PointerValue},
    IntPredicate,
};
use num_bigint::BigInt;

use crate::ir::cfg::{Expr, Type};
use crate::ir::metadata::{asset::Asset, ssz_info::SSZInfo};
use crate::ir_codegen::common::global::{get_extend_context, has_extend_context};
use crate::ir_codegen::traits::{BaseTypeMethods, BuilderMethods};
use crate::tools::leb128::leb128_encode;

use super::{
    context::{CompileResult, IR2LLVMCodeGenContext},
    encoding::MALLOC_FUNC_NAME,
};

pub const DATA_EMPTY_LENGTH: i32 = -1;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct IRStoragePath {
    pub keys: Vec<PathExpr>,
    pub extra_args: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PathExpr {
    Const(Vec<u8>),
    Val(Expr),
}

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    pub fn eval_path_expr(&self, expr: &PathExpr) -> (PointerValue<'ctx>, BasicValueEnum<'ctx>) {
        match expr {
            PathExpr::Const(val) => {
                let arr_length = self.i32_value(val.len().try_into().unwrap());

                let key = String::from_utf8_lossy(val.as_slice());

                let bytes = self.native_global_string(&key, "");
                (bytes, arr_length)
            }
            PathExpr::Val(val) => {
                let ty = {
                    let func = self.current_function.borrow();
                    self.ir_context
                        .ir_expr_ty(func.as_ref().unwrap(), val)
                        .unwrap()
                };
                let val = self.walk_ir_expr(val).unwrap();

                if ty.is_string() {
                    (self.vector_bytes(val), self.vector_len(val).into())
                } else if ty.is_integer() {
                    //  Convert int to string
                    let val = self.int_cast(val, self.i32_type(), true);
                    let leb128_bytes_length = self
                        .build_call("uleb128_value_length", &[val])
                        .into_int_value();
                    let bytes = self
                        .builder
                        .build_array_alloca(self.i8_type(), leb128_bytes_length, "")
                        .unwrap();
                    self.build_call("encode_uleb128", &[val, bytes.into(), self.i32_value(0)]);

                    (bytes, leb128_bytes_length.into())
                } else {
                    panic!("can't eval path expr :{expr:?} to llvm value")
                }
            }
        }
    }

    pub fn build_storage_path(&self, keys: &[Expr], extra_args: &[u32]) -> IRStoragePath {
        let mut result = IRStoragePath {
            extra_args: extra_args.to_vec(),
            ..Default::default()
        };
        for key in keys {
            if let Expr::Literal(lit) = key {
                let lit_str = lit.to_string();
                let val = if lit.literal_type().is_integer() {
                    let big_int = BigInt::from_str(&lit_str).unwrap();
                    leb128_encode(&big_int)
                } else {
                    lit_str.as_bytes().to_vec()
                };
                result.keys.push(PathExpr::Const(val));
            } else {
                result.keys.push(PathExpr::Val(key.clone()));
            }
        }
        result
    }

    pub fn build_path_ptr(&self, path: &IRStoragePath) -> BasicValueEnum<'ctx> {
        if path.keys.is_empty() {
            panic!("can't build empty storage path")
        } else {
            if let PathExpr::Val(key_expr) = &path.keys[0] {
                let key_ty = {
                    let func = self.current_function.borrow();
                    self.ir_context
                        .ir_expr_ty(func.as_ref().unwrap(), key_expr)
                        .unwrap()
                };
                if key_ty.is_storage_path() {
                    let storage_t_ptr = self.walk_ir_expr(key_expr).unwrap();

                    if path.keys.len() > 1 {
                        let mut rest_path = path.clone();
                        rest_path.keys = path.keys[1..].to_vec();
                        let (key_count, key_datas, key_lengths, extra_args_count, extra_args) =
                            self.build_keys(&rest_path);
                        let rest_ptr = self.build_call(
                            "build_storage_t_path_ptr",
                            &[
                                key_datas.into(),
                                key_count,
                                key_lengths.into(),
                                extra_args.into(),
                                extra_args_count,
                            ],
                        );
                        return self
                            .build_call("builtin_storage_path_join", &[storage_t_ptr, rest_ptr]);
                    }
                    return storage_t_ptr;
                }
            }

            let (key_count, key_datas, key_lengths, extra_args_count, extra_args) =
                self.build_keys(path);

            self.build_call(
                "build_storage_t_path_ptr",
                &[
                    key_datas.into(),
                    key_count,
                    key_lengths.into(),
                    extra_args.into(),
                    extra_args_count,
                ],
            )
        }
    }

    pub fn build_keys(
        &self,
        path: &IRStoragePath,
    ) -> (
        BasicValueEnum<'ctx>,
        PointerValue<'ctx>,
        PointerValue<'ctx>,
        BasicValueEnum<'ctx>, /* extra_args_count */
        PointerValue<'ctx>,   /* extra_args */
    ) {
        let key_len = path.keys.len();
        let key_count = self.i32_value(key_len.try_into().unwrap());

        let key_datas = self
            .builder
            .build_array_alloca(
                self.i8_ptr_type(),
                self.native_i8(key_len.try_into().unwrap()),
                "",
            )
            .unwrap();

        let key_lengths = self
            .builder
            .build_array_alloca(
                self.i32_type(),
                self.native_i8(key_len.try_into().unwrap()),
                "",
            )
            .unwrap();

        for i in 0..key_len {
            let key = path.keys.get(i).unwrap();
            let (key_data_value, key_data_len) = { self.eval_path_expr(key) };
            let key_data = unsafe {
                self.builder
                    .build_in_bounds_gep(
                        key_datas.get_type(),
                        key_datas,
                        &[self.native_i8(i.try_into().unwrap())],
                        "",
                    )
                    .unwrap()
            };
            self.builder.build_store(key_data, key_data_value).unwrap();

            let key_length = unsafe {
                self.builder
                    .build_in_bounds_gep(
                        key_lengths.get_type(),
                        key_lengths,
                        &[self.native_i8(i.try_into().unwrap())],
                        "",
                    )
                    .unwrap()
            };
            self.builder.build_store(key_length, key_data_len).unwrap();
        }

        // extra_args
        let extra_args_count = path.extra_args.len();
        let extra_args_count_value = self.i32_value(extra_args_count as u64);

        let extra_args_array = self
            .builder
            .build_array_alloca(
                self.i32_type(),
                self.native_i8(extra_args_count.try_into().unwrap()),
                "",
            )
            .unwrap();
        for (i, &extra_arg) in path.extra_args.iter().enumerate() {
            let extra_arg_data = unsafe {
                self.builder
                    .build_in_bounds_gep(
                        extra_args_array.get_type(),
                        extra_args_array,
                        &[self.native_i8(i.try_into().unwrap())],
                        "",
                    )
                    .unwrap()
            };
            let extra_arg_value = self.i32_value(extra_arg as u64);
            self.builder
                .build_store(extra_arg_data, extra_arg_value)
                .unwrap();
        }
        (
            key_count,
            key_datas,
            key_lengths,
            extra_args_count_value,
            extra_args_array,
        )
    }

    pub fn read_storage_object(
        &self,
        path_ptr: BasicValueEnum<'ctx>,
        ty: &Type,
        ssz_info: Option<SSZInfo>,
    ) -> CompileResult<'ctx> {
        let deref_ty = if let Type::Pointer(deref_ty) = ty {
            deref_ty
        } else {
            ty
        };

        let asset_tag = if let Type::Def(ty_def) = deref_ty {
            let md = Asset::get_from_context(self.ir_context, ty_def.as_ref());
            if let Some(md) = md {
                md.get_ty()
            } else {
                0
            }
        } else {
            0
        };

        let length = if asset_tag > 0 {
            // If asset metadata is active, please impl your owe business logic and/or HostAPI for asset object
            if has_extend_context() {
                get_extend_context().asset_get_data_length(asset_tag, self, path_ptr)
            } else {
                unimplemented!("Please impl trait ExtendContext")
            }
        } else {
            self.build_call("storage_read_object_length", &[path_ptr])
                .into_int_value()
        };

        let is_empty = self
            .builder
            .build_int_compare(
                IntPredicate::EQ,
                length,
                self.i32_value(DATA_EMPTY_LENGTH as u64).into_int_value(),
                "",
            )
            .unwrap();

        let value_ptr = self.builder.build_alloca(self.llvm_type(ty), "").unwrap();

        let then_block = self.append_block("default value");
        let else_block = self.append_block("read storage object");
        let end_block = self.append_block("");
        self.cond_br(is_empty.into(), then_block, else_block);
        self.builder.position_at_end(then_block);
        // Default value
        // Check if storage path has exceeded index
        self.build_void_call("assert_storage_array_index", &[path_ptr]);
        let then_value = self.type_default_value(ty);
        self.builder.build_store(value_ptr, then_value).unwrap();
        self.br(end_block);
        self.builder.position_at_end(else_block);
        let data = self
            .build_call(MALLOC_FUNC_NAME, &[length.into()])
            .into_pointer_value();
        let data = unsafe {
            self.builder
                .build_in_bounds_gep(data.get_type(), data, &[self.native_i8(0)], "")
                .unwrap()
                .into()
        };

        if asset_tag > 0 {
            // If asset metadata is active, please impl your owe business logic and/or HostAPI for asset object
            if has_extend_context() {
                get_extend_context().asset_get_data(asset_tag, self, path_ptr, data);
            } else {
                unimplemented!("Please impl trait ExtendContext")
            }
        } else {
            self.build_void_call("storage_load", &[path_ptr, data]);
        }

        let else_value = self.ssz_decode_with_version(ty, ssz_info, data, length.into());
        self.builder.build_store(value_ptr, else_value).unwrap();
        self.br(end_block);
        self.builder.position_at_end(end_block);
        Ok(self
            .builder
            .build_load(value_ptr.get_type(), value_ptr, "")
            .unwrap())
    }

    pub fn write_storage_object(
        &self,
        path_ptr: BasicValueEnum<'ctx>,
        value: BasicValueEnum<'ctx>,
        ty: &Type,
        ssz_info: Option<SSZInfo>,
    ) -> CompileResult<'ctx> {
        let deref_ty = if let Type::Pointer(deref_ty) = ty {
            deref_ty
        } else {
            ty
        };
        let asset_tag = if let Type::Def(ty_def) = deref_ty {
            let md = Asset::get_from_context(self.ir_context, ty_def.as_ref());
            if let Some(md) = md {
                md.get_ty()
            } else {
                0
            }
        } else {
            0
        };
        let (value_ptr, value_length) = self.ssz_encode_with_version(ty, ssz_info, value);
        if asset_tag > 0 {
            // If asset metadata is active, please impl your owe business logic and/or HostAPI for asset object
            if has_extend_context() {
                get_extend_context().asset_set_data(
                    asset_tag,
                    self,
                    path_ptr,
                    value_ptr,
                    value_length,
                );
            } else {
                unimplemented!("Please impl trait ExtendContext")
            }
        } else {
            self.build_void_call("storage_store", &[path_ptr, value_ptr, value_length]);
        }
        self.ok_result()
    }
}
