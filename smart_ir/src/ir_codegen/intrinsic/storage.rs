// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use crate::ir::cfg::Type;
use crate::ir_codegen::traits::BaseTypeMethods;

use crate::ir_codegen::common::global::{get_extend_context, has_extend_context};
use crate::ir_codegen::context::IR2LLVMCodeGenContext;
use crate::ir_codegen::storage_path::DATA_EMPTY_LENGTH;

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    pub fn build_storage_push(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let storage_path = *params.get(0).unwrap();
        let val = *params.get(1).unwrap();
        let elem_ty = params_ty.get(1).unwrap();

        let (value_ptr, value_length) = self.ssz_encode_with_version(elem_ty, None, val);
        let storage_path_ptr = self
            .builder
            .build_int_to_ptr(
                storage_path.into_int_value(),
                self.storage_path_ptr_type().into_pointer_type(),
                "",
            )
            .unwrap();

        let size_path_ptr = self.build_call(
            "storage_path_join_must_immut_string",
            &[
                storage_path_ptr.into(),
                self.native_global_string("size", "").into(),
                self.i32_value(4),
            ],
        );

        let size_path = self
            .builder
            .build_ptr_to_int(
                size_path_ptr.into_pointer_value(),
                self.i32_type().into_int_type(),
                "",
            )
            .unwrap();

        self.build_void_call(
            "builtin_storage_array_push",
            &[storage_path, size_path.into(), value_ptr, value_length],
        );
        self.i32_value(1)
    }

    pub fn build_storage_pop(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let storage_path = *params.get(0).unwrap();
        let storage_path_ptr = self
            .builder
            .build_int_to_ptr(
                storage_path.into_int_value(),
                self.storage_path_ptr_type().into_pointer_type(),
                "",
            )
            .unwrap();
        let size_path_ptr = self.build_call(
            "storage_path_join_must_immut_string",
            &[
                storage_path_ptr.into(),
                self.native_global_string("size", "").into(),
                self.i32_value(4),
            ],
        );
        let size_path = self
            .builder
            .build_ptr_to_int(
                size_path_ptr.into_pointer_value(),
                self.i32_type().into_int_type(),
                "",
            )
            .unwrap();

        self.build_void_call(
            "builtin_storage_array_pop",
            &[storage_path, size_path.into()],
        );
        self.i32_value(1)
    }

    pub fn build_storage_len(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let storage_path = *params.get(0).unwrap();
        let storage_path_ptr = self
            .builder
            .build_int_to_ptr(
                storage_path.into_int_value(),
                self.storage_path_ptr_type().into_pointer_type(),
                "",
            )
            .unwrap();
        let size_path_ptr = self.build_call(
            "storage_path_join_must_immut_string",
            &[
                storage_path_ptr.into(),
                self.native_global_bytes(&"size".to_string().into_bytes(), "size")
                    .into(),
                self.i32_value(4),
            ],
        );

        self.build_call("read_array_size_from_path", &[size_path_ptr])
    }

    pub fn build_storage_mint(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        if has_extend_context() {
            get_extend_context().issue_asset_or_token(self, params, _params_ty, _ret)
        } else {
            self.i32_value(1)
        }
    }

    pub fn build_storage_burn(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        if has_extend_context() {
            get_extend_context().burn_asset_or_token(self, params, _params_ty, _ret)
        } else {
            self.i32_value(1)
        }
    }

    pub fn build_storage_destroy(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        if has_extend_context() {
            get_extend_context().destory_asset_or_token(self, params, _params_ty, ret)
        } else {
            self.i32_value(1)
        }
    }

    pub fn build_storage_delete(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let storage_path = *params.get(0).unwrap();
        let length = self
            .build_call("storage_read_object_length", &[storage_path])
            .into_int_value();
        self.build_void_call("storage_delete_object", &[storage_path]);
        self.builder
            .build_int_compare(
                IntPredicate::NE,
                length,
                self.i32_value(DATA_EMPTY_LENGTH as u64).into_int_value(),
                "",
            )
            .unwrap()
            .into()
    }

    pub fn build_storage_get_balance(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        if has_extend_context() {
            get_extend_context().get_balance(self, params, _params_ty, _ret)
        } else {
            self.i32_value(1)
        }
    }

    pub fn build_storage_get_tag(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        if has_extend_context() {
            get_extend_context().check_asset_is_fungible(self, params, _params_ty, _ret)
        } else {
            self.i32_value(1)
        }
    }

    pub fn build_storage_transfer(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        if has_extend_context() {
            get_extend_context().transfer_asset_or_token(self, params, _params_ty, _ret)
        } else {
            self.i32_value(1)
        }
    }

    pub fn build_storage_verify_index(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let storage_path = *params.get(0).unwrap();
        let index = *params.get(1).unwrap();
        let storage_path_ptr = self
            .builder
            .build_int_to_ptr(
                storage_path.into_int_value(),
                self.storage_path_ptr_type().into_pointer_type(),
                "",
            )
            .unwrap();

        let size_path_ptr = self.build_call(
            "storage_path_join_must_immut_string",
            &[
                storage_path_ptr.into(),
                self.native_global_string("size", "").into(),
                self.i32_value(4),
            ],
        );

        self.build_call(
            "verify_storage_array_index",
            &[storage_path_ptr.into(), size_path_ptr, index],
        )
    }

    pub fn build_storage_contains_key(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let storage_path = *params.get(0).unwrap();
        let length = self
            .build_call("storage_read_object_length", &[storage_path])
            .into_int_value();
        self.builder
            .build_int_compare(
                IntPredicate::NE,
                length,
                self.i32_value(DATA_EMPTY_LENGTH as u64).into_int_value(),
                "",
            )
            .unwrap()
            .into()
    }

    pub fn build_storage_contains_asset(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        if has_extend_context() {
            get_extend_context().has_asset(self, params, _params_ty, _ret)
        } else {
            self.i32_value(1)
        }
    }

    pub fn build_storage_set_bss(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        if has_extend_context() {
            get_extend_context().build_storage_set_bss(self, params, _params_ty, _ret)
        } else {
            self.i32_value(1)
        }
    }

    pub fn build_storage_get_bss(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        if has_extend_context() {
            get_extend_context().build_storage_get_bss(self, params, _params_ty, _ret)
        } else {
            self.i32_value(1)
        }
    }
}
