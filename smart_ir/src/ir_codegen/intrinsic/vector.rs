// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::types::BasicType;
use inkwell::values::BasicValueEnum;

use crate::ir::cfg::Type;
use crate::ir_codegen::builtin_constants::{
    Q_VECTOR_ITER, Q_VECTOR_OBJ_S, Q_VEC_NEW_FUNC_NAME, Q_VEC_SLICE_FUNC_NAME,
};
use crate::ir_codegen::context::IR2LLVMCodeGenContext;
use crate::ir_codegen::encoding::MALLOC_FUNC_NAME;
use crate::ir_codegen::traits::{BaseTypeMethods, BuilderMethods};
use crate::ir_codegen::ty::Q_VEC_LLVM_TY;

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    pub fn build_vector_set(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let vec_ptr = *params.get(0).unwrap();
        let idx = *params.get(1).unwrap();
        let val = *params.get(2).unwrap();
        let runtime_ctx = *params.get(3).unwrap();

        let idx = self.int_cast(idx, self.i32_type(), true);

        let val_ptr = self.builder.build_alloca(val.get_type(), "").unwrap();
        self.builder.build_store(val_ptr, val).unwrap();

        // the part of qvector_setat accepts value is a pointer points to actual value(i8*)
        let store_value_u8_ptr = self.ptr_cast(val_ptr.into(), self.i8_ptr_type());
        // the new value of qvector_setat passed is the memory address of such value
        self.build_call(
            "qvector_setat",
            &[vec_ptr, idx, store_value_u8_ptr, runtime_ctx],
        );
        self.i32_value(1)
    }

    pub fn build_vector_get(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let vec_ptr = *params.get(0).unwrap();
        let idx = *params.get(1).unwrap();
        let runtime_ctx = *params.get(2).unwrap();
        // here, return type of qvector_getat is i8*, points to initial address of vector element, the value of vector element is primitive type or pointer
        let pointer_to_element = self.build_call(
            "qvector_getat",
            &[
                vec_ptr,
                idx,
                self.i1_value(0), /* newmem = false */
                runtime_ctx,
            ],
        );
        let value_llvm_ptr_ty = self.ptr_type_to(self.llvm_type(ret));
        // need convert i8* to the pointer of target type of vector element, struct T** or int32* etc. that is pointer to pointer or primitive type pointer
        let value_ptr = self.ptr_cast(pointer_to_element, value_llvm_ptr_ty);

        self.builder
            .build_load(self.llvm_type(ret), value_ptr.into_pointer_value(), "")
            .unwrap()
    }

    pub fn build_vector_push(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let vec_ptr = *params.get(0).unwrap();
        let val = *params.get(1).unwrap();

        let val_ptr = self.builder.build_alloca(val.get_type(), "").unwrap();
        self.builder.build_store(val_ptr, val).unwrap();

        // the part of qvector_setat accepts value is a pointer points to actual value(i8*)
        let store_value_u8_ptr = self.ptr_cast(val_ptr.into(), self.i8_ptr_type());

        self.build_void_call("qvector_addlast", &[vec_ptr, store_value_u8_ptr]);
        self.i32_value(1)
    }

    pub fn build_vector_create_iter(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let vec_ptr = *params.get(0).unwrap();

        let iter_size = self
            .module
            .get_struct_type(Q_VECTOR_ITER)
            .unwrap()
            .size_of()
            .unwrap()
            .const_cast(self.i32_type().into_int_type(), false);
        let malloc_ptr = self.build_call(MALLOC_FUNC_NAME, &[iter_size.into()]);

        let iter_ptr = self.ptr_cast(malloc_ptr, self.llvm_type(&Type::vec_iter()));

        let vec_field = self
            .builder
            .build_struct_gep(
                self.module.get_struct_type(Q_VECTOR_ITER).unwrap(),
                iter_ptr.into_pointer_value(),
                0,
                "",
            )
            .unwrap();

        self.builder.build_store(vec_field, vec_ptr).unwrap();

        let qvector_obj_ty = self.module.get_struct_type(Q_VECTOR_OBJ_S).unwrap();
        let malloc_ptr = self.build_call(
            MALLOC_FUNC_NAME,
            &[qvector_obj_ty
                .size_of()
                .unwrap()
                .const_cast(self.i32_type().into_int_type(), false)
                .into()],
        );

        let obj_ptr = self.ptr_cast(malloc_ptr, self.ptr_type_to(qvector_obj_ty.into()));
        self.memset_struct_ptr(obj_ptr.into_pointer_value(), qvector_obj_ty.into(), 0);

        let obj_field = self
            .builder
            .build_struct_gep(
                self.module.get_struct_type(Q_VECTOR_ITER).unwrap(),
                iter_ptr.into_pointer_value(),
                1,
                "",
            )
            .unwrap();

        self.builder.build_store(obj_field, obj_ptr).unwrap();

        iter_ptr
    }

    pub fn build_vector_get_next(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let iter_ptr = *params.get(0).unwrap();

        let vec_field = self
            .builder
            .build_struct_gep(
                self.module.get_struct_type(Q_VECTOR_ITER).unwrap(),
                iter_ptr.into_pointer_value(),
                0,
                "",
            )
            .unwrap();
        let vec_ptr = self
            .builder
            .build_load(
                self.ptr_type_to(self.module.get_struct_type(Q_VEC_LLVM_TY).unwrap().into()),
                vec_field,
                "",
            )
            .unwrap();

        let obj_field = self
            .builder
            .build_struct_gep(
                self.module.get_struct_type(Q_VECTOR_ITER).unwrap(),
                iter_ptr.into_pointer_value(),
                1,
                "",
            )
            .unwrap();

        let obj_ptr = self
            .builder
            .build_load(
                self.ptr_type_to(self.module.get_struct_type(Q_VECTOR_OBJ_S).unwrap().into()),
                obj_field,
                "",
            )
            .unwrap();

        self.build_call(
            "qvector_getnext",
            &[vec_ptr, obj_ptr, self.i1_value(0) /* newmem = false */],
        )
    }

    pub fn build_vector_obj_key(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let iter_ptr = *params.get(0).unwrap();

        let obj_field = self
            .builder
            .build_struct_gep(
                self.module.get_struct_type(Q_VECTOR_ITER).unwrap(),
                iter_ptr.into_pointer_value(),
                1,
                "",
            )
            .unwrap();

        let obj_ptr = self
            .builder
            .build_load(
                self.ptr_type_to(self.module.get_struct_type(Q_VECTOR_OBJ_S).unwrap().into()),
                obj_field,
                "",
            )
            .unwrap();

        let idx_field = self
            .builder
            .build_struct_gep(
                self.module.get_struct_type(Q_VECTOR_OBJ_S).unwrap(),
                obj_ptr.into_pointer_value(),
                1,
                "",
            )
            .unwrap();

        let next_idx = self
            .builder
            .build_load(self.i32_type(), idx_field, "")
            .unwrap();
        self.builder
            .build_int_nsw_sub(
                next_idx.into_int_value(),
                self.i32_value(1).into_int_value(),
                "",
            )
            .unwrap()
            .into()
    }

    pub fn build_vector_obj_value(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let iter_ptr = *params.get(0).unwrap();
        let obj_field = self
            .builder
            .build_struct_gep(
                self.module.get_struct_type(Q_VECTOR_ITER).unwrap(),
                iter_ptr.into_pointer_value(),
                1,
                "",
            )
            .unwrap();

        let obj_ptr = self
            .builder
            .build_load(
                self.ptr_type_to(self.module.get_struct_type(Q_VECTOR_OBJ_S).unwrap().into()),
                obj_field,
                "",
            )
            .unwrap();
        let data_field = self
            .builder
            .build_struct_gep(
                self.module.get_struct_type(Q_VECTOR_OBJ_S).unwrap(),
                obj_ptr.into_pointer_value(),
                0,
                "",
            )
            .unwrap();
        let data_ptr = self
            .builder
            .build_load(self.i8_ptr_type(), data_field, "")
            .unwrap();
        let elem_ptr = self.ptr_cast(data_ptr, self.ptr_type_to(self.llvm_type(ret)));
        self.builder
            .build_load(self.llvm_type(ret), elem_ptr.into_pointer_value(), "")
            .unwrap()
    }

    pub fn build_vector_insert(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let vec_ptr = *params.get(0).unwrap();
        let idx = *params.get(1).unwrap();
        let val = *params.get(2).unwrap();

        let idx = self.int_cast(idx, self.i32_type(), true);

        let val_ptr = self.builder.build_alloca(val.get_type(), "").unwrap();
        self.builder.build_store(val_ptr, val).unwrap();

        // the part of qvector_setat accepts value is a pointer points to actual value(i8*)
        let store_value_u8_ptr = self.ptr_cast(val_ptr.into(), self.i8_ptr_type());
        // the new value of qvector_setat passed is the memory address of such value
        self.build_call("qvector_addat", &[vec_ptr, idx, store_value_u8_ptr])
    }

    pub fn build_vector_delete(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let vec_ptr = *params.get(0).unwrap();
        let idx = *params.get(1).unwrap();

        let idx = self.int_cast(idx, self.i32_type(), true);

        self.build_call("qvector_removeat", &[vec_ptr, idx])
    }

    pub fn build_vector_new(&self, elem_ty: &Type, len: &Option<u32>) -> BasicValueEnum<'ctx> {
        let elem_size = self
            .llvm_type(elem_ty)
            .size_of()
            .unwrap()
            .const_cast(self.i32_type().into_int_type(), false);
        let mut capacity = len.unwrap_or(1);
        if capacity == 0 {
            capacity = 1
        }
        let capacity_value = self.i32_value(capacity as u64);
        let vector_mode: u64 = 2; // qvector.c mode: double when capacity not enough
        let vector_ptr = self.build_call(
            Q_VEC_NEW_FUNC_NAME,
            &[
                capacity_value,
                elem_size.into(),
                self.i32_value(vector_mode),
            ],
        );
        vector_ptr
    }

    pub fn build_vector_slice(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let from_vec_ptr = *params.get(0).unwrap();
        let begin = *params.get(1).unwrap();
        let end = *params.get(2).unwrap();
        let runtime_ctx = *params.get(3).unwrap();

        self.build_call(
            Q_VEC_SLICE_FUNC_NAME,
            &[from_vec_ptr, begin, end, runtime_ctx],
        )
    }
}
