// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::{types::BasicType, values::BasicValueEnum};

use crate::ir::cfg::Type;
use crate::ir_codegen::traits::{BaseTypeMethods, BuilderMethods};
use crate::ir_codegen::{
    builtin_constants::{Q_HASHTBL_OBJ_S, VECTOR_NEW_FUNC_NAME},
    context::IR2LLVMCodeGenContext,
    encoding::MALLOC_FUNC_NAME,
};

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    pub fn build_map_set(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let map_ptr = *params.get(0).unwrap();
        let key = *params.get(1).unwrap();
        let val = *params.get(2).unwrap();
        let key_ty = params_ty.get(1).unwrap();

        let tbl_key = if key_ty.is_string() {
            let key32: BasicValueEnum = self
                .builder
                .build_ptr_to_int(self.vector_bytes(key), self.i32_type().into_int_type(), "")
                .into();
            self.int_cast(key32, self.i64_type(), false)
        } else if key_ty.is_integer() {
            self.int_cast(key, self.i64_type(), false)
        } else {
            let ptr = self.builder.build_alloca(key.get_type(), "");
            self.builder.build_store(ptr, key);
            let key32: BasicValueEnum = self
                .builder
                .build_ptr_to_int(ptr, self.i32_type().into_int_type(), "")
                .into();
            self.int_cast(key32, self.i64_type(), false)
        };

        let val_ptr = self.builder.build_alloca(val.get_type(), "");
        self.builder.build_store(val_ptr, val);

        let entry_u8_ptr_value = self.ptr_cast(val_ptr.into(), self.i8_ptr_type());

        let size = val
            .get_type()
            .size_of()
            .unwrap()
            .const_cast(self.i32_type().into_int_type(), false);

        self.build_hashtbl_put(
            map_ptr.into_pointer_value(),
            tbl_key.into_int_value(),
            entry_u8_ptr_value.into_pointer_value(),
            size,
        )
    }

    pub fn build_map_get(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let map_ptr = *params.get(0).unwrap();
        let key = *params.get(1).unwrap();
        let key_ty = params_ty.get(1).unwrap();

        let table_key = if key_ty.is_string() {
            let key_i32: BasicValueEnum = self
                .builder
                .build_ptr_to_int(self.vector_bytes(key), self.i32_type().into_int_type(), "")
                .into();
            self.int_cast(key_i32, self.i64_type(), false)
        } else if key_ty.is_integer() {
            self.int_cast(key, self.i64_type(), false)
        } else {
            let ptr = self.builder.build_alloca(key.get_type(), "");
            self.builder.build_store(ptr, key);
            let key_i32: BasicValueEnum = self
                .builder
                .build_ptr_to_int(ptr, self.i32_type().into_int_type(), "")
                .into();
            self.int_cast(key_i32, self.i64_type(), false)
        };

        // TODO: qhashtbl_get maybe return NULL. need change to qhashtbl_get_or_default
        let data = self.build_hashtbl_get(
            map_ptr.into_pointer_value(),
            table_key.into_int_value(),
            None,
            false,
        );

        let value_llvm_ptr_ty = self.ptr_type_to(self.llvm_type(ret));
        // the part of qhashtbl_get accepts value is a pointer points to actual value(i8*)
        let value_ptr = self.ptr_cast(data.into(), value_llvm_ptr_ty);
        self.builder.build_load(value_ptr.into_pointer_value(), "")
    }

    pub fn build_map_contains_key(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let map_ptr = *params.get(0).unwrap();
        let key = *params.get(1).unwrap();
        let key_ty = params_ty.get(1).unwrap();

        let table_key = if key_ty.is_string() {
            let key_i32: BasicValueEnum = self
                .builder
                .build_ptr_to_int(self.vector_bytes(key), self.i32_type().into_int_type(), "")
                .into();
            self.int_cast(key_i32, self.i64_type(), false)
        } else if key_ty.is_integer() {
            self.int_cast(key, self.i64_type(), false)
        } else {
            let ptr = self.builder.build_alloca(key.get_type(), "");
            self.builder.build_store(ptr, key);
            let key_i32: BasicValueEnum = self
                .builder
                .build_ptr_to_int(ptr, self.i32_type().into_int_type(), "")
                .into();
            self.int_cast(key_i32, self.i64_type(), false)
        };

        self.build_hashtbl_contains_key(map_ptr.into_pointer_value(), table_key.into_int_value())
    }

    pub fn build_map_delete(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let map_ptr = *params.get(0).unwrap();
        let key = *params.get(1).unwrap();
        let key_ty = params_ty.get(1).unwrap();

        let tbl_key = if key_ty.is_string() {
            let key_i32: BasicValueEnum = self
                .builder
                .build_ptr_to_int(self.vector_bytes(key), self.i32_type().into_int_type(), "")
                .into();
            self.int_cast(key_i32, self.i64_type(), false)
        } else if key_ty.is_integer() {
            self.int_cast(key, self.i64_type(), false)
        } else {
            let ptr = self.builder.build_alloca(key.get_type(), "");
            self.builder.build_store(ptr, key);
            let key_i32: BasicValueEnum = self
                .builder
                .build_ptr_to_int(ptr, self.i32_type().into_int_type(), "")
                .into();
            self.int_cast(key_i32, self.i64_type(), false)
        };

        self.build_hashtbl_remove(map_ptr.into_pointer_value(), tbl_key.into_int_value())
    }

    pub fn build_map_create_iter(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let map_ptr = *params.get(0).unwrap();

        let iter_size = self
            .llvm_type(&Type::map_iter())
            .into_pointer_type()
            .get_element_type()
            .size_of()
            .unwrap()
            .const_cast(self.i32_type().into_int_type(), false);
        let malloc_ptr = self.build_call(MALLOC_FUNC_NAME, &[iter_size.into()]);

        let iter_ptr = self.ptr_cast(malloc_ptr, self.llvm_type(&Type::map_iter()));

        let map_field = self
            .builder
            .build_struct_gep(iter_ptr.into_pointer_value(), 0, "")
            .unwrap();

        self.builder.build_store(map_field, map_ptr);

        let qhashtbl_obj_ty = self.module.get_struct_type(Q_HASHTBL_OBJ_S).unwrap();
        let malloc_ptr = self.build_call(
            MALLOC_FUNC_NAME,
            &[qhashtbl_obj_ty
                .size_of()
                .unwrap()
                .const_cast(self.i32_type().into_int_type(), false)
                .into()],
        );

        let obj_ptr = self.ptr_cast(malloc_ptr, self.ptr_type_to(qhashtbl_obj_ty.into()));
        self.memset_struct_ptr(obj_ptr.into_pointer_value(), 0);

        let obj_field = self
            .builder
            .build_struct_gep(iter_ptr.into_pointer_value(), 1, "")
            .unwrap();

        self.builder.build_store(obj_field, obj_ptr);

        iter_ptr
    }

    pub fn build_map_get_next(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let iter_ptr = *params.get(0).unwrap();

        let map_field = self
            .builder
            .build_struct_gep(iter_ptr.into_pointer_value(), 0, "")
            .unwrap();
        let map_ptr = self.builder.build_load(map_field, "");

        let obj_field = self
            .builder
            .build_struct_gep(iter_ptr.into_pointer_value(), 1, "")
            .unwrap();

        let obj_ptr = self.builder.build_load(obj_field, "");

        self.build_hashtbl_getnext(
            map_ptr.into_pointer_value(),
            obj_ptr.into_pointer_value(),
            false,
        )
    }

    pub fn build_map_obj_key(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let iter_ptr = *params.get(0).unwrap();

        let obj_field = self
            .builder
            .build_struct_gep(iter_ptr.into_pointer_value(), 1, "")
            .unwrap();

        let obj_ptr = self.builder.build_load(obj_field, "");

        let key_field = self
            .builder
            .build_struct_gep(obj_ptr.into_pointer_value(), 1, "")
            .unwrap();

        let key_data = self.builder.build_load(key_field, "");

        if ret.is_string() {
            let key_bytes = self.int_to_ptr(key_data, self.i8_ptr_type());
            let key_len = self.build_call("__strlen", &[key_bytes]);
            self.build_call(
                VECTOR_NEW_FUNC_NAME,
                &[key_len, self.i32_value(1), key_bytes],
            )
        } else {
            self.builder
                .build_int_cast(
                    key_data.into_int_value(),
                    self.llvm_type(ret).into_int_type(),
                    "",
                )
                .into()
        }
    }

    pub fn build_map_obj_value(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let iter_ptr = *params.get(0).unwrap();

        let obj_field = self
            .builder
            .build_struct_gep(iter_ptr.into_pointer_value(), 1, "")
            .unwrap();

        let obj_ptr = self.builder.build_load(obj_field, "");

        let value_field = self
            .builder
            .build_struct_gep(obj_ptr.into_pointer_value(), 2, "")
            .unwrap();

        let value_byte = self.builder.build_load(value_field, "");
        let value_ptr = self.ptr_cast(value_byte, self.ptr_type_to(self.llvm_type(ret)));
        self.builder.build_load(value_ptr.into_pointer_value(), "")
    }
}
