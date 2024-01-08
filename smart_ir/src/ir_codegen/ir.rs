// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::{
    types::StringRadix,
    values::{BasicMetadataValueEnum, BasicValueEnum},
    IntPredicate,
};

use crate::ir::metadata::{ssz_info::SSZInfo, storage_path_extra_args::StoragePathExtraArgs};
use crate::ir::{
    cfg::{
        BasicBlock, BinaryOp, CmpOp, Expr, Instr, InstrDescription, IntLiteral, Literal, Type,
        TypeDefinitionKind,
    },
    interface_type::PartialFuncNameBehavior,
    metadata::debug_info::DebugLocation,
};
use crate::ir_codegen::traits::BuilderMethods;

use super::{
    builtin_constants::{MEMCMP_FUNC, Q_MAP_NEW_FUNC_NAME, VECTOR_NEW_FUNC_NAME},
    context::{CodeGenError, CompileResult, IR2LLVMCodeGenContext},
    error::FUNCTION_RETURN_VALUE_NOT_FOUND_MSG,
};

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    pub fn walk_bb(&self, bb: &BasicBlock) -> CompileResult<'ctx> {
        let bb_map = self.bb_map.borrow();
        let b = *bb_map.get(&bb.id).unwrap();
        self.builder.position_at_end(b);
        for instr in &bb.instrs {
            self.walk_instr(instr)?;
        }
        self.ok_result()
    }

    pub fn walk_instr(&self, instr: &Instr) -> CompileResult<'ctx> {
        self.update_runtime_ctx(DebugLocation::get_from_context(self.ir_context, instr));
        match &instr.inner {
            InstrDescription::Declaration { id, init_val, ty } => {
                let name = id.to_string();

                let var_ty = self.var_ty(&name);

                let init_val = match init_val {
                    Some(expr) => self.walk_ir_expr(expr)?,
                    None => self.type_default_value(ty),
                };

                let ptr = self.build_or_get_variable(&name, &var_ty);
                self.builder.build_store(ptr, init_val).unwrap();
                self.ok_result()
            }
            InstrDescription::Assignment { id, val } => {
                let name = id.to_string();
                let val = self.walk_ir_expr(val)?;

                let ptr = self
                    .get_variable_ptr(&name)
                    .unwrap_or_else(|| panic!("can't find variable %{name}"));

                self.builder.build_store(ptr, val).unwrap();

                self.ok_result()
            }
            InstrDescription::Ret { val } => {
                match val {
                    Some(v) => {
                        let ret = self.walk_ir_expr(v)?;
                        self.ret(ret);
                    }
                    None => self.ret_void(),
                }
                self.ok_result()
            }
            InstrDescription::Br { target } => {
                let bb_map = self.bb_map.borrow();
                self.br(*bb_map.get(target).unwrap());
                self.ok_result()
            }
            InstrDescription::BrIf {
                cond,
                then_bb,
                else_bb,
            } => {
                let bb_map = self.bb_map.borrow();
                let then_bb = *bb_map.get(then_bb).unwrap();
                let else_bb = *bb_map.get(else_bb).unwrap();

                let cond = self.walk_ir_expr(cond)?;
                self.cond_br(cond, then_bb, else_bb);
                self.ok_result()
            }
            InstrDescription::Match {
                val: _,
                otherwise: _,
                jump_table: _,
            } => unimplemented!(),
            InstrDescription::Not { op } => {
                let op = self.walk_ir_expr(op)?;
                let res = self.builder.build_not(op.into_int_value(), "").unwrap();
                Ok(res.into())
            }
            InstrDescription::BitNot { op } => {
                let op = self.walk_ir_expr(op)?;
                let res = self.builder.build_not(op.into_int_value(), "").unwrap();
                Ok(res.into())
            }
            InstrDescription::Binary { op_code, op1, op2 } => {
                let left = self.walk_ir_expr(op1)?;
                let right = self.walk_ir_expr(op2)?;
                let expr_ty = {
                    let func = self.current_function.borrow();
                    self.ir_context
                        .ir_expr_ty(func.as_ref().unwrap(), op1)
                        .unwrap()
                };
                if self.opts.overflow_check
                    && matches!(op_code, BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul)
                {
                    return Ok(self.integer_overflow_check(
                        left.into_int_value(),
                        right.into_int_value(),
                        &expr_ty,
                        op_code,
                    ));
                }

                let res: BasicValueEnum = match op_code {
                    BinaryOp::Add => {
                        if expr_ty.is_signed_int() {
                            self.builder
                                .build_int_nsw_add(
                                    left.into_int_value(),
                                    right.into_int_value(),
                                    "",
                                )
                                .unwrap()
                        } else {
                            self.builder
                                .build_int_add(left.into_int_value(), right.into_int_value(), "")
                                .unwrap()
                        }
                    }
                    BinaryOp::Sub => {
                        if expr_ty.is_signed_int() {
                            self.builder
                                .build_int_nsw_sub(
                                    left.into_int_value(),
                                    right.into_int_value(),
                                    "",
                                )
                                .unwrap()
                        } else {
                            self.builder
                                .build_int_sub(left.into_int_value(), right.into_int_value(), "")
                                .unwrap()
                        }
                    }
                    BinaryOp::Mul => {
                        if expr_ty.is_signed_int() {
                            self.builder
                                .build_int_nsw_mul(
                                    left.into_int_value(),
                                    right.into_int_value(),
                                    "",
                                )
                                .unwrap()
                        } else {
                            self.builder
                                .build_int_mul(left.into_int_value(), right.into_int_value(), "")
                                .unwrap()
                        }
                    }
                    BinaryOp::Div => {
                        if expr_ty.is_signed_int() {
                            self.builder
                                .build_int_signed_div(
                                    left.into_int_value(),
                                    right.into_int_value(),
                                    "",
                                )
                                .unwrap()
                        } else {
                            self.builder
                                .build_int_unsigned_div(
                                    left.into_int_value(),
                                    right.into_int_value(),
                                    "",
                                )
                                .unwrap()
                        }
                    }
                    BinaryOp::Mod => {
                        if expr_ty.is_signed_int() {
                            self.builder
                                .build_int_signed_rem(
                                    left.into_int_value(),
                                    right.into_int_value(),
                                    "",
                                )
                                .unwrap()
                        } else {
                            self.builder
                                .build_int_unsigned_rem(
                                    left.into_int_value(),
                                    right.into_int_value(),
                                    "",
                                )
                                .unwrap()
                        }
                    }
                    BinaryOp::Exp => self
                        .build_pow(left.into_int_value(), right.into_int_value(), &expr_ty)
                        .into_int_value(),
                    BinaryOp::And => self
                        .builder
                        .build_and(left.into_int_value(), right.into_int_value(), "")
                        .unwrap(),

                    BinaryOp::BitAnd => self
                        .builder
                        .build_and(left.into_int_value(), right.into_int_value(), "")
                        .unwrap(),

                    BinaryOp::Or => self
                        .builder
                        .build_or(left.into_int_value(), right.into_int_value(), "")
                        .unwrap(),

                    BinaryOp::BitOr => self
                        .builder
                        .build_or(left.into_int_value(), right.into_int_value(), "")
                        .unwrap(),

                    BinaryOp::BitXor => self
                        .builder
                        .build_xor(left.into_int_value(), right.into_int_value(), "")
                        .unwrap(),

                    BinaryOp::Shl => self
                        .builder
                        .build_left_shift(left.into_int_value(), right.into_int_value(), "")
                        .unwrap(),
                    BinaryOp::Shr => self
                        .builder
                        .build_right_shift(
                            left.into_int_value(),
                            right.into_int_value(),
                            expr_ty.is_signed_int(),
                            "",
                        )
                        .unwrap(),
                    BinaryOp::Sar => self
                        .builder
                        .build_right_shift(left.into_int_value(), right.into_int_value(), true, "")
                        .unwrap(),
                }
                .into();
                Ok(res)
            }
            InstrDescription::Cmp { op_code, op1, op2 } => {
                let left = self.walk_ir_expr(op1)?;
                let right = self.walk_ir_expr(op2)?;
                let expr_ty = {
                    let func = self.current_function.borrow();
                    self.ir_context
                        .ir_expr_ty(func.as_ref().unwrap(), op1)
                        .unwrap()
                };
                let res = if expr_ty.is_integer() || expr_ty.is_boolean() {
                    let llvm_cmp_code = match op_code {
                        CmpOp::Eq => IntPredicate::EQ,
                        CmpOp::Ne => IntPredicate::NE,
                        CmpOp::Gt => {
                            if expr_ty.is_signed_int() {
                                IntPredicate::SGT
                            } else {
                                IntPredicate::UGT
                            }
                        }

                        CmpOp::Ge => {
                            if expr_ty.is_signed_int() {
                                IntPredicate::SGE
                            } else {
                                IntPredicate::UGE
                            }
                        }
                        CmpOp::Lt => {
                            if expr_ty.is_signed_int() {
                                IntPredicate::SLT
                            } else {
                                IntPredicate::ULT
                            }
                        }
                        CmpOp::Le => {
                            if expr_ty.is_signed_int() {
                                IntPredicate::SLE
                            } else {
                                IntPredicate::ULE
                            }
                        }
                    };

                    self.builder
                        .build_int_compare(
                            llvm_cmp_code,
                            left.into_int_value(),
                            right.into_int_value(),
                            "",
                        )
                        .unwrap()
                        .into()
                } else if expr_ty.is_string() {
                    let left_bytes = self.vector_bytes(left);
                    let left_len = self.vector_len(left);
                    let right_bytes = self.vector_bytes(right);
                    let right_len = self.vector_len(right);
                    match op_code {
                        CmpOp::Eq => self.build_call(
                            MEMCMP_FUNC,
                            &[
                                left_bytes.into(),
                                left_len.into(),
                                right_bytes.into(),
                                right_len.into(),
                            ],
                        ),
                        CmpOp::Ne => {
                            let result = self.build_call(
                                MEMCMP_FUNC,
                                &[
                                    left_bytes.into(),
                                    left_len.into(),
                                    right_bytes.into(),
                                    right_len.into(),
                                ],
                            );
                            let i1_type = self.llvm_context.custom_width_int_type(1);
                            self.builder
                                .build_int_compare(
                                    inkwell::IntPredicate::NE,
                                    result.into_int_value(),
                                    i1_type.const_all_ones(),
                                    "",
                                )
                                .unwrap()
                                .into()
                        }
                        _ => unreachable!(),
                    }
                } else {
                    unimplemented!()
                };
                Ok(res)
            }
            InstrDescription::Alloca { ty: _ } => unimplemented!(),
            InstrDescription::Malloc { ty } => match ty {
                Type::Primitive(_) => unimplemented!(),
                Type::Map {
                    key: key_ty,
                    value: _,
                } => {
                    let key_runtime_type_enum = self.ir_context.ir_runtime_class_c_enum(key_ty);
                    Ok(self.build_call(
                        Q_MAP_NEW_FUNC_NAME,
                        &[
                            self.i32_value(0),
                            self.i8_value(key_runtime_type_enum as u64),
                            self.i32_value(0),
                        ],
                    ))
                }
                Type::Array { elem, len } => Ok(self.build_vector_new(elem, len)),
                Type::Compound(_) => unimplemented!(),
                Type::Pointer(_) => unimplemented!(),
                Type::Def(def) => match def.kind {
                    TypeDefinitionKind::Struct => Ok(self.type_default_ptr_value(&def.ty)),
                    TypeDefinitionKind::Enum => unimplemented!(),
                    TypeDefinitionKind::Builtin => unimplemented!(),
                    TypeDefinitionKind::Alias => unimplemented!(),
                },
                Type::Builtin(_) => unimplemented!(),
            },
            InstrDescription::Free { ptr: _ } => unimplemented!(),
            InstrDescription::GetField {
                ptr,
                field_path,
                field_ty,
            } => {
                let ptr_ty = self
                    .ir_context
                    .ir_expr_ty(self.current_function.borrow().as_ref().unwrap(), ptr);
                let mut ptr = self.walk_ir_expr(ptr)?;
                if let Some(Type::Pointer(pointee_ty)) = ptr_ty {
                    let mut struct_ty = pointee_ty;
                    for (i, field_idx) in field_path.iter().enumerate() {
                        if let Type::Def(def_ty) = struct_ty.as_ref() {
                            let field_ptr = self
                                .builder
                                .build_struct_gep(
                                    self.llvm_type(&def_ty.ty),
                                    ptr.into_pointer_value(),
                                    *field_idx,
                                    "",
                                )
                                .unwrap();
                            if i + 1 < field_path.len() {
                                if let Type::Compound(fields) = def_ty.ty.as_ref() {
                                    struct_ty = fields.get(*field_idx as usize).unwrap().ty.clone();
                                    ptr = self
                                        .builder
                                        .build_load(self.llvm_type(&struct_ty), field_ptr, "")
                                        .unwrap();
                                } else {
                                    return Err(CodeGenError {
                                        message: "wrong type defination".to_string(),
                                    });
                                }
                            } else {
                                ptr = field_ptr.into();
                            }
                        } else {
                            return Err(CodeGenError {
                                message:
                                    "try to get field from a pointer whose elemet isn't a struct"
                                        .to_string(),
                            });
                        }
                    }
                } else {
                    return Err(CodeGenError {
                        message: "try to get field from value which isn't a pointer".to_string(),
                    });
                };
                let res = self
                    .builder
                    .build_load(self.llvm_type(field_ty), ptr.into_pointer_value(), "")
                    .unwrap();
                Ok(res)
            }
            InstrDescription::SetField {
                ptr,
                val,
                field_path,
            } => {
                let ptr_ty = self
                    .ir_context
                    .ir_expr_ty(self.current_function.borrow().as_ref().unwrap(), ptr);
                let mut ptr = self.walk_ir_expr(ptr)?;
                let val = self.walk_ir_expr(val)?;

                if let Some(Type::Pointer(pointee_ty)) = ptr_ty {
                    let mut struct_ty = pointee_ty;
                    for (i, field_idx) in field_path.iter().enumerate() {
                        if let Type::Def(def_ty) = struct_ty.as_ref() {
                            let field_ptr = self
                                .builder
                                .build_struct_gep(
                                    self.llvm_type(&struct_ty),
                                    ptr.into_pointer_value(),
                                    *field_idx,
                                    "",
                                )
                                .unwrap();
                            if i + 1 < field_path.len() {
                                if let Type::Compound(fields) = def_ty.ty.as_ref() {
                                    struct_ty = fields.get(*field_idx as usize).unwrap().ty.clone();
                                    ptr = self
                                        .builder
                                        .build_load(self.llvm_type(&struct_ty), field_ptr, "")
                                        .unwrap();
                                } else {
                                    return Err(CodeGenError {
                                        message: "wrong type defination".to_string(),
                                    });
                                }
                            } else {
                                ptr = field_ptr.into();
                            }
                        } else {
                            return Err(CodeGenError {
                                message:
                                    "try to get field from a pointer whose elemet isn't a struct"
                                        .to_string(),
                            });
                        }
                    }

                    self.builder
                        .build_store(ptr.into_pointer_value(), val)
                        .unwrap();
                    self.ok_result()
                } else {
                    Err(CodeGenError {
                        message: "try to set field to value which isn't a pointer".to_string(),
                    })
                }
            }
            InstrDescription::GetStoragePath { storage_path } => {
                let md = StoragePathExtraArgs::get_from_context(self.ir_context, instr).unwrap();
                // FIXME: temp modify,
                let mut storage_extra_args = md.get_extra_args().clone();
                storage_extra_args.push(0);

                let storage_path =
                    self.build_storage_path(storage_path.as_slice(), &storage_extra_args);
                let value_ptr = self.build_path_ptr(&storage_path);
                Ok(value_ptr)
            }
            InstrDescription::StorageLoad {
                storage_path,
                load_ty,
            } => {
                let path_ptr = self.walk_ir_expr(storage_path)?;
                let result = self.read_storage_object(
                    path_ptr,
                    load_ty,
                    SSZInfo::get_from_context(self.ir_context, instr),
                )?;
                Ok(result)
            }
            InstrDescription::StorageStore {
                storage_path,
                store_val,
            } => {
                let path_ptr = self.walk_ir_expr(storage_path)?;
                let val = self.walk_ir_expr(store_val)?;
                let ty = {
                    let func = self.current_function.borrow();
                    self.ir_context
                        .ir_expr_ty(func.as_ref().unwrap(), store_val)
                        .unwrap()
                };
                self.write_storage_object(
                    path_ptr,
                    val,
                    &ty,
                    SSZInfo::get_from_context(self.ir_context, instr),
                )
            }
            InstrDescription::Call {
                func_name,
                args,
                ret_ty,
            } => {
                let params_ty = args
                    .iter()
                    .map(|arg| {
                        let func = self.current_function.borrow();
                        self.ir_context
                            .ir_expr_ty(func.as_ref().unwrap(), arg)
                            .unwrap()
                    })
                    .collect::<Vec<Type>>();
                let mut args = args
                    .iter()
                    .map(|arg| self.walk_ir_expr(arg).unwrap())
                    .collect::<Vec<BasicValueEnum>>();

                match &func_name.kind {
                    crate::ir::interface_type::PartialFuncNameKind::UserDefFunc(name) => {
                        let function_name = self.get_internal_function_name(
                            name.as_str(),
                            params_ty.as_slice(),
                            ret_ty,
                            false,
                        );
                        if ret_ty.is_void() {
                            self.build_void_call(&function_name, args.as_slice());
                            self.ok_result()
                        } else {
                            let ret_val = self.build_call(&function_name, args.as_slice());
                            Ok(ret_val)
                        }
                    }
                    crate::ir::interface_type::PartialFuncNameKind::Intrinsic(ir_func) => {
                        if self.is_runtime_abort(ir_func) {
                            args.push(self.get_runtime_ctx());
                        }
                        let args: Vec<BasicMetadataValueEnum> =
                            args.iter().map(|v| (*v).into()).collect();
                        let func = self.add_or_get_intrinsic_function(
                            ir_func,
                            params_ty.as_slice(),
                            ret_ty,
                        );
                        if ret_ty.is_void() {
                            self.builder.build_call(func, args.as_slice(), "").unwrap();
                            self.ok_result()
                        } else {
                            let ret_val = self
                                .builder
                                .build_call(func, args.as_slice(), "")
                                .unwrap()
                                .try_as_basic_value()
                                .left()
                                .unwrap_or_else(|| {
                                    panic!(
                                        "{FUNCTION_RETURN_VALUE_NOT_FOUND_MSG}: {}",
                                        ir_func.apply_name()
                                    )
                                });

                            Ok(ret_val)
                        }
                    }
                    crate::ir::interface_type::PartialFuncNameKind::HostAPI(_) => {
                        unimplemented!()
                    }
                    crate::ir::interface_type::PartialFuncNameKind::Otherwise => {
                        unimplemented!()
                    }
                }
            }
            InstrDescription::IntCast { val, target_ty } => {
                let val = self.walk_ir_expr(val)?;
                let ret = self
                    .builder
                    .build_int_cast(
                        val.into_int_value(),
                        self.llvm_type(target_ty).into_int_type(),
                        "",
                    )
                    .unwrap()
                    .into();
                Ok(ret)
            }
        }
    }

    pub fn walk_ir_expr(&self, expr: &Expr) -> CompileResult<'ctx> {
        match expr {
            Expr::Identifier(id) => {
                let name = id.to_string();
                let ptr = self
                    .get_variable_ptr(&name)
                    .unwrap_or_else(|| panic!("can't find variable %{name}"));
                let ty = self
                    .get_variable_ty(&name)
                    .unwrap_or_else(|| panic!("can't find variable %{name}"));

                let res = self.builder.build_load(ty, ptr, "").unwrap();
                Ok(res)
            }
            Expr::Instr(instr) => self.walk_instr(instr),
            Expr::Literal(lit) => {
                let lit_ty = lit.literal_type();
                let res = match lit {
                    Literal::Str(val) => {
                        let size = self.i32_value(1);
                        let length = self.i32_value(val.len() as u64);
                        self.build_call(
                            VECTOR_NEW_FUNC_NAME,
                            &[length, size, self.native_global_string(val, "").into()],
                        )
                    }
                    Literal::Bool(val) => self
                        .llvm_context
                        .bool_type()
                        .const_int(*val as u64, false)
                        .into(),
                    Literal::Int(int_lit) => {
                        let int_lit_str = match int_lit {
                            IntLiteral::I8(val) => val.to_string(),
                            IntLiteral::I16(val) => val.to_string(),
                            IntLiteral::I32(val) => val.to_string(),
                            IntLiteral::I64(val) => val.to_string(),
                            IntLiteral::I128(val) => val.to_string(),
                            IntLiteral::I256(val) => val.to_string(),
                            IntLiteral::U8(val) => val.to_string(),
                            IntLiteral::U16(val) => val.to_string(),
                            IntLiteral::U32(val) => val.to_string(),
                            IntLiteral::U64(val) => val.to_string(),
                            IntLiteral::U128(val) => val.to_string(),
                            IntLiteral::U256(val) => val.to_string(),
                        };

                        self.llvm_type(&lit_ty)
                            .into_int_type()
                            .const_int_from_string(&int_lit_str, StringRadix::Decimal)
                            .unwrap()
                            .into()
                    }
                };
                Ok(res)
            }
            Expr::NOP => unimplemented!(),
        }
    }
}
