// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::types::BasicType;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};

use super::builtin_constants::{
    IR_BUILTIN_SSZ_DECODE_VOID_PTR, IR_BUILTIN_SSZ_ENCODE_VOID_PTR, Q_MAP_NEW_FUNC_NAME,
    Q_VEC_NEW_FUNC_NAME, SSZ_ENCODE_LEN, VECTOR_NEW_FUNC_NAME,
};
use super::context::IR2LLVMCodeGenContext;
use super::error::INTERNAL_ERROR_MSG;
use crate::encoding::datastream::{ParamType, DEFAULT_VERSION};
use crate::ir::cfg::Type;
use crate::ir::metadata::asset::Asset;
use crate::ir::metadata::ssz_info::SSZInfo;
use crate::ir_codegen::traits::{BaseTypeMethods, BuilderMethods};

pub const ULEB128_VALUE_LENGTH_FUNC_NAME: &str = "uleb128_value_length";
pub const DECODE_ULEB128_VALUE_FUNC_NAME: &str = "decode_uleb128_value";

pub const MALLOC_FUNC_NAME: &str = "__malloc";
pub const BUILTIN_CO_CALL_AUTO_REVERT: &str = "builtin_co_call_or_revert";
const U16_SIZE: u64 = 2;

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    pub fn ssz_decode_with_version(
        &self,
        ty: &Type,
        ssz_info: Option<SSZInfo>,
        data: BasicValueEnum<'ctx>,
        data_length: BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        let is_versioned = if let Some(info) = ssz_info {
            info.get_versioned()
        } else {
            self.is_versioned(ty)
        };
        let ssz_version_size = self.i32_value(U16_SIZE);
        // when data is parampack/struct, then its type is pointer
        let data = if data.is_pointer_value() {
            self.bit_cast(data, self.i32_type())
        } else {
            data
        };
        let is_versioned = self.i1_value(u64::from(is_versioned));
        let data = self.build_call(
            "ir_builtin_versioned_ssz_get_data_ptr",
            &[data, data_length, is_versioned, ssz_version_size],
        );
        let data_length = self.build_call(
            "ir_builtin_versioned_ssz_get_data_len",
            &[data_length, is_versioned, ssz_version_size],
        );
        let offset_value = self.intern_ir_runtime_class_offset(ty);

        // asset/ vector [T] data maybe empty bytes, need ssz decode support
        let is_allow_empty = match ty {
            Type::Array { elem: _, len } => u64::from(len.is_none()),
            Type::Pointer(ptr) => {
                if let Type::Def(def) = ptr.as_ref() {
                    u64::from(Asset::get_from_context(self.ir_context, def.as_ref()).is_some())
                } else {
                    0
                }
            }
            _ => 0,
        };

        let is_allow_empty_object = self.i1_value(is_allow_empty);
        let void_ptr = self.build_call(
            IR_BUILTIN_SSZ_DECODE_VOID_PTR,
            &[offset_value, is_allow_empty_object, data, data_length],
        );

        if ty.is_reference_type() || ty.is_string() {
            let value_ptr = self.ptr_cast(void_ptr, self.llvm_type(ty));
            value_ptr
        } else {
            let value_llvm_ptr_ty = self.ptr_type_to(self.llvm_type(ty));
            let value_ptr = self.ptr_cast(void_ptr, value_llvm_ptr_ty);
            return self
                .builder
                .build_load(self.llvm_type(ty), value_ptr.into_pointer_value(), "")
                .unwrap();
        }
    }

    pub fn ssz_encode_with_version(
        &self,
        ty: &Type,
        ssz_info: Option<SSZInfo>,
        value: BasicValueEnum<'ctx>,
    ) -> (BasicValueEnum<'ctx>, BasicValueEnum<'ctx>) {
        let offset_value = self.intern_ir_runtime_class_offset(ty);

        let void_ptr = if ty.is_reference_type() || ty.is_string() {
            self.ptr_cast(value, self.i8_ptr_type())
        } else {
            let val_ptr = self.builder.build_alloca(self.llvm_type(ty), "").unwrap();
            self.builder.build_store(val_ptr, value).unwrap();
            self.ptr_cast(val_ptr.into(), self.i8_ptr_type())
        };

        let value_ptr = self.build_call(IR_BUILTIN_SSZ_ENCODE_VOID_PTR, &[offset_value, void_ptr]);
        let value_length = self.build_call(SSZ_ENCODE_LEN, &[offset_value, void_ptr]);

        let is_versioned = if let Some(info) = ssz_info {
            info.get_versioned()
        } else {
            self.is_versioned(ty)
        };
        if is_versioned {
            let versioned_value_length = self
                .builder
                .build_int_add(
                    value_length.into_int_value(),
                    self.i32_value(U16_SIZE).into_int_value(),
                    "",
                )
                .unwrap()
                .into();
            let versioned_value_ptr = self.build_call(MALLOC_FUNC_NAME, &[versioned_value_length]);

            self.build_call(
                "ssz_encode_u16",
                &[
                    self.llvm_context.i16_type().const_int(1, false).into(),
                    versioned_value_ptr,
                    self.i32_value(0),
                ],
            );
            self.build_call(
                "memcpy_offset",
                &[
                    versioned_value_ptr,
                    versioned_value_length,
                    self.i32_value(U16_SIZE),
                    value_ptr,
                    value_length,
                ],
            );

            (versioned_value_ptr, versioned_value_length)
        } else {
            (value_ptr, value_length)
        }
    }

    /// Call runtime data_stream decode function with the variable type u8 pointer and length,
    /// and return the decoded value and the next offset.
    pub fn data_stream_decode(
        &self,
        ty: &Type,
        data: BasicValueEnum<'ctx>,
        offset: BasicValueEnum<'ctx>,
        len: BasicValueEnum<'ctx>,
        name: &str,
    ) -> (PointerValue<'ctx>, BasicValueEnum<'ctx>) {
        let param_ty: ParamType = if ty.is_parampack() {
            ParamType::Str
        } else {
            (*ty).clone().try_into().expect(INTERNAL_ERROR_MSG)
        };
        let mut ptr = if ty.is_string() || ty.is_parampack() {
            // Data stream decoding string buffer (equals to a slice [u8]).
            self.new_data_stream_decode_vector(
                self.llvm_type(&Type::u8())
                    .size_of()
                    .unwrap()
                    .const_cast(self.i32_type().into_int_type(), false)
                    .into(),
                data,
                offset,
                len,
                name,
            )
        } else if ty.is_boolean() {
            self.builder
                .build_alloca(self.llvm_type(&Type::u8()), name)
                .unwrap()
        } else if ty.is_integer() {
            self.builder.build_alloca(self.llvm_type(ty), name).unwrap()
        } else if ty.is_array() {
            let stream_len = len;
            // Data stream decoding vector buffer
            if let Type::Array { elem, len: _ } = ty {
                let elem_size = self
                    .llvm_type(elem)
                    .size_of()
                    .unwrap()
                    .const_cast(self.i32_type().into_int_type(), false)
                    .into();
                let size =
                    self.build_call(DECODE_ULEB128_VALUE_FUNC_NAME, &[data, offset, stream_len]);
                self.build_call(Q_VEC_NEW_FUNC_NAME, &[size, elem_size, self.i32_value(0)])
                    .into_pointer_value()
            } else {
                unreachable!("invalid array type")
            }
        } else if let Type::Map {
            key: key_ty,
            value: _,
        } = &ty
        {
            let key_runtime_ty = self.ir_context.ir_runtime_class_c_enum(key_ty);
            self.build_call(
                Q_MAP_NEW_FUNC_NAME,
                &[
                    self.i32_value(0),
                    self.i8_value(u64::from(key_runtime_ty)),
                    self.i32_value(0),
                ],
            )
            .into_pointer_value()
        } else {
            unimplemented!()
        };

        // Data stream decode
        let next_offset = self.build_call(
            &param_ty.get_decode_func_name(),
            &[ptr.into(), data, offset, len],
        );
        if ty.is_boolean() {
            let u8_value = self
                .builder
                .build_load(self.llvm_type(&Type::u8()), ptr, "")
                .unwrap();
            let bool_value: BasicValueEnum = self
                .builder
                .build_int_compare(
                    inkwell::IntPredicate::NE,
                    u8_value.into_int_value(),
                    self.i8_value(0).into_int_value(),
                    "",
                )
                .unwrap()
                .into();
            ptr = self.builder.build_alloca(self.llvm_type(ty), name).unwrap();
            self.builder.build_store(ptr, bool_value).unwrap();
        }
        (ptr, next_offset)
    }

    /// Call runtime date stream encode function with the variable bytes u8 pointer and length,
    /// and return the encoded bytes and offset.
    pub fn data_stream_encode(
        &self,
        types: &[Type],
        values: &[BasicValueEnum<'ctx>],
        is_parampack: bool,
    ) -> (BasicValueEnum<'ctx>, BasicValueEnum<'ctx>) {
        let version = self.i8_value(DEFAULT_VERSION as u64);

        // Temporary solution for passing parampack parameter
        if !is_parampack && types.len() == 1 && types[0].is_parampack() {
            let len = self.vector_len(values[0]);
            let total_size = self
                .builder
                .build_int_add(len, self.i32_value(1).into_int_value(), "")
                .unwrap();
            let bytes = self.build_call(MALLOC_FUNC_NAME, &[total_size.into()]);
            let first_byte = unsafe {
                self.builder
                    .build_in_bounds_gep(
                        self.i8_type(),
                        bytes.into_pointer_value(),
                        &[self.native_i8(0)],
                        "",
                    )
                    .unwrap()
            };
            self.builder.build_store(first_byte, version).unwrap();

            self.build_call(
                "memcpy_offset",
                &[
                    bytes,
                    total_size.into(),
                    self.i32_value(1),
                    self.build_call("vector_bytes", &[values[0]]),
                    len.into(),
                ],
            );
            return (bytes, total_size.into());
        }

        let mut total_size = self.data_stream_len(types, values);
        if !is_parampack {
            total_size = self
                .builder
                .build_int_add(total_size, self.i32_value(1).into_int_value(), "")
                .unwrap();
        }

        let bytes = self
            .build_call(MALLOC_FUNC_NAME, &[total_size.into()])
            .into_pointer_value();
        if !is_parampack {
            // Set the first version byte.
            let first_byte = unsafe {
                self.builder
                    .build_in_bounds_gep(self.i8_type(), bytes, &[self.native_i8(0)], "")
            }
            .unwrap();
            self.builder.build_store(first_byte, version).unwrap();
        }

        let mut offset = if is_parampack {
            self.i32_value(0)
        } else {
            // Skip the first version byte.
            self.i32_value(1)
        };

        for (i, ty) in types.iter().enumerate() {
            let value = values.get(i).unwrap();
            if ty.is_array() {
                let param_ty: ParamType = (*ty).clone().try_into().unwrap();
                offset = self.build_call(
                    &param_ty.get_encode_func_name(),
                    &[*value, bytes.into(), offset, self.get_runtime_ctx()],
                );
            } else {
                let param_ty: ParamType = if ty.is_parampack() {
                    ParamType::Str
                } else {
                    (*ty).clone().try_into().unwrap()
                };
                offset = self.build_call(
                    &param_ty.get_encode_func_name(),
                    &[*value, bytes.into(), offset],
                );
            }
        }
        (bytes.into(), offset)
    }

    fn data_stream_len(&self, types: &[Type], values: &[BasicValueEnum<'ctx>]) -> IntValue<'ctx> {
        let mut total_size = self.i32_value(0).into_int_value();
        for (i, ty) in (*types).iter().enumerate() {
            let value = values.get(i).unwrap();
            let val_size = if ty.is_integer() || ty.is_boolean() {
                self.llvm_type(ty)
                    .size_of()
                    .unwrap()
                    .const_cast(self.i32_type().into_int_type(), false)
            } else if ty.is_string() || ty.is_parampack() {
                let len = self.vector_len(*value);
                let leb128_bytes_length = self
                    .build_call(ULEB128_VALUE_LENGTH_FUNC_NAME, &[len.into()])
                    .into_int_value();
                self.builder
                    .build_int_add(len, leb128_bytes_length, "")
                    .unwrap()
            } else if ty.is_array() {
                let len = self.q_vector_len(*value);
                let elem_size = if let Type::Array { elem, len: _ } = ty {
                    self.llvm_type(elem)
                        .size_of()
                        .unwrap()
                        .const_cast(self.i32_type().into_int_type(), false)
                } else {
                    unreachable!("invalid array type")
                };
                let leb128_bytes_length = self
                    .build_call(ULEB128_VALUE_LENGTH_FUNC_NAME, &[len.into()])
                    .into_int_value();
                self.builder
                    .build_int_add(
                        self.builder.build_int_mul(len, elem_size, "").unwrap(),
                        leb128_bytes_length,
                        "",
                    )
                    .unwrap()
            } else if ty.is_map() {
                self.build_call("qhashtbl_total_space", &[*value])
                    .into_int_value()
            } else {
                unimplemented!()
            };
            total_size = self
                .builder
                .build_int_add(total_size, val_size, "")
                .unwrap()
        }
        total_size
    }
    /// New a data stream decode vector with the element size.
    fn new_data_stream_decode_vector(
        &self,
        elem_size: BasicValueEnum<'ctx>,
        data: BasicValueEnum<'ctx>,
        offset: BasicValueEnum<'ctx>,
        len: BasicValueEnum<'ctx>,
        name: &str,
    ) -> PointerValue<'ctx> {
        // Data stream decoding vector buffer
        let size = self.build_call(DECODE_ULEB128_VALUE_FUNC_NAME, &[data, offset, len]);
        let init = self
            .builder
            .build_int_to_ptr(
                self.llvm_context.i128_type().const_zero(),
                self.i8_ptr_type().into_pointer_type(),
                name,
            )
            .unwrap();
        self.build_call(VECTOR_NEW_FUNC_NAME, &[size, elem_size, init.into()])
            .into_pointer_value()
    }
}
