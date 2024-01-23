// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::types::{BasicType, BasicTypeEnum, FunctionType, VoidType};
use inkwell::values::BasicValueEnum;

use super::builtin_constants::{
    Q_HASHTBL_OBJ_S, Q_MAP_NEW_FUNC_NAME, Q_VECTOR_OBJ_S, Q_VEC_NEW_FUNC_NAME,
    RUNTIME_CONTEXT_LLVM_TY, VECTOR_NEW_FUNC_NAME,
};
use super::context::IR2LLVMCodeGenContext;

use super::error::INTERNAL_ERROR_MSG;
use crate::ir_codegen::traits::{BaseTypeMethods, BuilderMethods};

use crate::ir::cfg::{BuiltinType, FunctionDefinition, IntType, PrimitiveType, Type};
use crate::ir::metadata::ssz_info::SSZInfo;

pub const VEC_LLVM_TY: &str = "struct.vector";
pub const Q_VEC_LLVM_TY: &str = "struct.qvector_s";
pub const Q_MAP_LLVM_TY: &str = "struct.qhashtbl_s";

/// Impl TypedResultWalker for LLVMCodeGenContext to visit AST nodes to emit LLVM IR.
impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    /// Return the llvm type for the resolved type.
    pub fn llvm_type(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Primitive(prim_ty) => match prim_ty {
                PrimitiveType::Str => self.vec_ptr_type(),
                PrimitiveType::Bool => self.llvm_context.bool_type().into(),
                PrimitiveType::Void => self.i8_type(),
                PrimitiveType::Int(int_ty) => match int_ty {
                    IntType::I8 => self.i8_type(),
                    IntType::I16 => self.llvm_context.i16_type().into(),
                    IntType::I32 => self.i32_type(),
                    IntType::I64 => self.i64_type(),
                    IntType::I128 => self.llvm_context.i128_type().into(),
                    IntType::I256 => self.llvm_context.custom_width_int_type(256).into(),
                    IntType::U8 => self.i8_type(),
                    IntType::U16 => self.llvm_context.i16_type().into(),
                    IntType::U32 => self.i32_type(),
                    IntType::U64 => self.i64_type(),
                    IntType::U128 => self.llvm_context.i128_type().into(),
                    IntType::U256 => self.llvm_context.custom_width_int_type(256).into(),
                },
            },
            Type::Map { key: _, value: _ } => self.ptr_type_to(
                self.module
                    .get_struct_type(Q_MAP_LLVM_TY)
                    .expect(INTERNAL_ERROR_MSG)
                    .into(),
            ),
            Type::Array { elem: _, len: _ } => self.ptr_type_to(
                self.module
                    .get_struct_type(Q_VEC_LLVM_TY)
                    .expect(INTERNAL_ERROR_MSG)
                    .into(),
            ),
            Type::Compound(fields) => self
                .llvm_context
                .struct_type(
                    &fields
                        .iter()
                        .map(|field| self.llvm_type(&field.ty))
                        .collect::<Vec<BasicTypeEnum>>(),
                    true,
                )
                .into(),
            Type::Pointer(ptr) => self.ptr_type_to(self.llvm_type(ptr)),
            Type::Def(def) => self.llvm_type(&def.ty),
            Type::Builtin(builtin_ty) => match builtin_ty {
                BuiltinType::VectorIter => self.ptr_type_to(self.vec_iter_struct_type()),

                BuiltinType::MapIter => self.ptr_type_to(self.map_iter_struct_type()),
                BuiltinType::Parampack => self.vec_ptr_type(),
                BuiltinType::StoragePath => self.i32_type(),
            },
        }
    }

    // String default empty value.
    pub fn string_default_value(&self) -> BasicValueEnum<'ctx> {
        let size = self.i32_value(1);
        let length = self.i32_value(0);
        self.build_call(
            VECTOR_NEW_FUNC_NAME,
            &[length, size, self.native_global_string("", "").into()],
        )
    }

    /// Type default value e.g., int -> 0, str -> "", vec -> []
    pub fn type_default_value(&self, ty: &Type) -> BasicValueEnum<'ctx> {
        match &ty {
            Type::Primitive(prim_ty) => match prim_ty {
                PrimitiveType::Str => self.string_default_value(),
                _ => self.llvm_type_default_value(ty),
            },
            Type::Map {
                key: key_ty,
                value: _,
            } => self.build_call(
                Q_MAP_NEW_FUNC_NAME,
                &[
                    self.i32_value(0),
                    self.i1_value(u64::from(key_ty.is_integer())),
                    self.i32_value(0),
                ],
            ),
            Type::Array { elem, len: _ } => {
                let elem_size = self
                    .llvm_type(elem)
                    .size_of()
                    .unwrap()
                    .const_cast(self.i32_type().into_int_type(), false);
                let size = self.i32_value(1_u64);
                let vector_ptr = self
                    .build_call(
                        Q_VEC_NEW_FUNC_NAME,
                        &[size, elem_size.into(), self.i32_value(2)],
                    )
                    .into_pointer_value();
                vector_ptr.into()
            }
            Type::Compound(_) => unreachable!(),
            Type::Pointer(elem_ty) => self.type_default_ptr_value(elem_ty),
            Type::Def(def) => self.type_default_value(&def.ty),
            Type::Builtin(_) => unimplemented!(),
        }
    }

    pub fn type_default_ptr_value(&self, elem_ty: &Type) -> BasicValueEnum<'ctx> {
        let llvm_ty = self.llvm_type(elem_ty);
        let s = self
            .build_call(
                "__malloc",
                &[llvm_ty
                    .size_of()
                    .unwrap()
                    .const_cast(self.i32_type().into_int_type(), false)
                    .into()],
            )
            .into_pointer_value();
        match elem_ty {
            Type::Compound(fields) => {
                let ptr = self.ptr_cast(s.into(), self.ptr_type_to(llvm_ty));
                for (i, field_ty) in fields.iter().enumerate() {
                    let field = unsafe {
                        self.builder
                            .build_gep(
                                llvm_ty,
                                ptr.into_pointer_value(),
                                &[
                                    self.llvm_context.i32_type().const_zero(),
                                    self.llvm_context.i32_type().const_int(i as u64, false),
                                ],
                                "",
                            )
                            .unwrap()
                    };
                    self.builder
                        .build_store(field, self.type_default_value(&field_ty.ty))
                        .unwrap();
                }
                ptr
            }
            Type::Def(def) => self.type_default_ptr_value(&def.ty),
            Type::Builtin(_) => unimplemented!(),
            _ => {
                self.builder
                    .build_store(s, self.type_default_value(elem_ty))
                    .unwrap();
                s.into()
            }
        }
    }

    fn llvm_type_default_value(&self, ty: &Type) -> BasicValueEnum<'ctx> {
        let llvm_ty = self.llvm_type(ty);
        match llvm_ty {
            BasicTypeEnum::ArrayType(_) => unimplemented!(),
            BasicTypeEnum::FloatType(_) => unimplemented!(),
            BasicTypeEnum::IntType(int_ty) => int_ty.const_zero().into(),
            BasicTypeEnum::PointerType(ptr_ty) => ptr_ty.const_zero().into(),
            BasicTypeEnum::StructType(_) => unimplemented!(),
            BasicTypeEnum::VectorType(_) => {
                let size = self.i32_value(1);
                let length = self.i32_value(0);
                self.build_call(
                    VECTOR_NEW_FUNC_NAME,
                    &[length, size, self.native_global_string("", "").into()],
                )
            }
        }
    }

    #[inline]
    pub fn llvm_function_type(&self, func: &FunctionDefinition) -> FunctionType<'ctx> {
        // Data stream bytes denotes the vector type.
        self.build_llvm_function_type(func.params.as_slice(), &func.ret, false)
    }

    #[inline]
    pub fn build_llvm_function_type(
        &self,
        params: &[Type],
        ret: &Type,
        runtime_abort: bool,
    ) -> FunctionType<'ctx> {
        // Data stream bytes denotes the vector type.
        let mut args: Vec<BasicTypeEnum> = params.iter().map(|t| self.llvm_type(t)).collect();
        if runtime_abort {
            args.push(
                self.ptr_type_to(
                    self.module
                        .get_struct_type(RUNTIME_CONTEXT_LLVM_TY)
                        .unwrap()
                        .into(),
                ),
            );
        }
        if ret.is_void() {
            self.void_function_let(&args)
        } else {
            self.function_let(&args, self.llvm_type(ret))
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn void_type(&self) -> VoidType<'ctx> {
        self.llvm_context.void_type()
    }

    #[inline]
    pub fn i8_ptr_type(&self) -> BasicTypeEnum<'ctx> {
        self.ptr_type_to(self.i8_type())
    }

    #[inline]
    pub fn i32_ptr_type(&self) -> BasicTypeEnum<'ctx> {
        self.ptr_type_to(self.i32_type())
    }

    #[inline]
    pub fn vec_type(&self) -> BasicTypeEnum<'ctx> {
        self.module
            .get_struct_type(VEC_LLVM_TY)
            .expect(INTERNAL_ERROR_MSG)
            .into()
    }

    #[inline]
    pub fn vec_ptr_type(&self) -> BasicTypeEnum<'ctx> {
        self.ptr_type_to(self.vec_type())
    }

    /*
        struct.vector.iter {
            struct.qvector_s* vec,
            struct.qvector_obj_s* obj,
        }
    */
    #[inline]
    pub fn vec_iter_struct_type(&self) -> BasicTypeEnum<'ctx> {
        self.llvm_context
            .struct_type(
                &[
                    self.ptr_type_to(
                        self.module
                            .get_struct_type(Q_VEC_LLVM_TY)
                            .expect(INTERNAL_ERROR_MSG)
                            .into(),
                    ),
                    self.ptr_type_to(
                        self.module
                            .get_struct_type(Q_VECTOR_OBJ_S)
                            .expect(INTERNAL_ERROR_MSG)
                            .into(),
                    ),
                ],
                true,
            )
            .into()
    }

    /*
       struct.map.iter {
           struct.qhashtbl_s* map,
           struct.qhashtbl_obj_s* obj,
       }
    */
    #[inline]
    pub fn map_iter_struct_type(&self) -> BasicTypeEnum<'ctx> {
        self.llvm_context
            .struct_type(
                &[
                    self.ptr_type_to(
                        self.module
                            .get_struct_type(Q_MAP_LLVM_TY)
                            .expect(INTERNAL_ERROR_MSG)
                            .into(),
                    ),
                    self.ptr_type_to(
                        self.module
                            .get_struct_type(Q_HASHTBL_OBJ_S)
                            .expect(INTERNAL_ERROR_MSG)
                            .into(),
                    ),
                ],
                true,
            )
            .into()
    }

    #[inline]
    pub fn storage_path_ptr_type(&self) -> BasicTypeEnum<'ctx> {
        self.ptr_type_to(
            self.module
                .get_struct_type("struct.storage_path")
                .expect(INTERNAL_ERROR_MSG)
                .into(),
        )
    }

    pub fn is_versioned(&self, ty: &Type) -> bool {
        let deref_ty = if let Type::Pointer(deref_ty) = ty {
            deref_ty
        } else {
            ty
        };
        if let Type::Def(def) = deref_ty {
            if let Some(info) = SSZInfo::get_from_context(self.ir_context, def.as_ref()) {
                info.get_versioned()
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn var_ty(&self, id: &str) -> Type {
        let id = id.parse::<u32>().unwrap();
        let cur_func = self.current_function.borrow();
        cur_func
            .as_ref()
            .unwrap()
            .vars
            .get(&id)
            .unwrap_or_else(|| panic!("unknown var id: {id}"))
            .clone()
    }

    pub fn intern_ir_runtime_class_offset(&self, ty: &Type) -> BasicValueEnum<'ctx> {
        let arg_runtime_class_offset = self.class_generator.borrow().intern_ir_runtime_class(ty);
        self.i32_value(arg_runtime_class_offset as u64)
    }
}
