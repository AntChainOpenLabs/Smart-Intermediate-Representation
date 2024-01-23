// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::cfg::{
    FunctionDefinition, InstrDescription, IntType, PrimitiveType, Type, TypeDefinitionKind,
};
use crate::ir::metadata::asset::Asset;
use crate::ir::{
    builder::Builder,
    builder::MetaDataId,
    cfg,
    cfg::{Expr, MetaData, Module},
};
use indexmap::IndexMap;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct ASTLoweringError {
    pub message: String,
}

/// The compiler function result
pub type CompileResult = Result<Expr, ASTLoweringError>;

pub const IR_RUNTIME_U8_TYPE: u8 = 0;
pub const IR_RUNTIME_U16_TYPE: u8 = 1;
pub const IR_RUNTIME_U32_TYPE: u8 = 2;
pub const IR_RUNTIME_U64_TYPE: u8 = 3;
pub const IR_RUNTIME_U128_TYPE: u8 = 4;
pub const IR_RUNTIME_U256_TYPE: u8 = 5;
pub const IR_RUNTIME_I8_TYPE: u8 = 6;
pub const IR_RUNTIME_I16_TYPE: u8 = 7;
pub const IR_RUNTIME_I32_TYPE: u8 = 8;
pub const IR_RUNTIME_I64_TYPE: u8 = 9;
pub const IR_RUNTIME_I128_TYPE: u8 = 10;
pub const IR_RUNTIME_I256_TYPE: u8 = 5;
pub const IR_RUNTIME_BOOL_TYPE: u8 = 12;
pub const IR_RUNTIME_STR_TYPE: u8 = 13;
pub const IR_RUNTIME_ASSET_TYPE: u8 = 14;
pub const IR_RUNTIME_STRUCT_TYPE: u8 = 15;
pub const IR_RUNTIME_ARRAY_TYPE: u8 = 16;
pub const IR_RUNTIME_MAP_TYPE: u8 = 17;

/// AntChain IR IRContext
#[derive(Debug, Default, Clone)]
pub struct IRContext {
    // current
    pub metadata: RefCell<IndexMap<MetaDataId, MetaData>>,
    pub modules: RefCell<IndexMap<String, Module>>,
    pub current_module: RefCell<Module>,
    pub main_module: String,
    pub builder: Builder,
}

impl IRContext {
    pub fn new() -> Self {
        IRContext {
            ..Default::default()
        }
    }

    pub fn add_metadata(&self, metadata: MetaData) -> MetaDataId {
        let md_idx = self.builder.get_metadata_id();
        self.metadata.borrow_mut().insert(md_idx, metadata);
        md_idx
    }

    pub fn get_metadata(&self, md_idx: &MetaDataId) -> Option<Ref<MetaData>> {
        Ref::filter_map(self.metadata.borrow(), |m| m.get(md_idx)).ok()
    }

    pub fn get_main_module(&self) -> Option<Ref<Module>> {
        Ref::filter_map(self.modules.borrow(), |m| m.get(&self.main_module)).ok()
    }

    pub fn ir_runtime_class_c_enum(&self, ty: &Type) -> u32 {
        (match ty {
            // match with runtime/stdlib.h
            Type::Primitive(prim_ty) => match prim_ty {
                PrimitiveType::Int(int_ty) => match int_ty {
                    IntType::I8 => IR_RUNTIME_I8_TYPE,
                    IntType::I16 => IR_RUNTIME_I16_TYPE,
                    IntType::I32 => IR_RUNTIME_I32_TYPE,
                    IntType::I64 => IR_RUNTIME_I64_TYPE,
                    IntType::I128 => IR_RUNTIME_I128_TYPE,
                    IntType::I256 => IR_RUNTIME_I256_TYPE,
                    IntType::U8 => IR_RUNTIME_U8_TYPE,
                    IntType::U16 => IR_RUNTIME_U16_TYPE,
                    IntType::U32 => IR_RUNTIME_U32_TYPE,
                    IntType::U64 => IR_RUNTIME_U64_TYPE,
                    IntType::U128 => IR_RUNTIME_U128_TYPE,
                    IntType::U256 => IR_RUNTIME_U256_TYPE,
                },
                PrimitiveType::Str => IR_RUNTIME_STR_TYPE,
                PrimitiveType::Bool => IR_RUNTIME_BOOL_TYPE,
                PrimitiveType::Void => unimplemented!(),
            },
            Type::Pointer(ptr) => self.ir_runtime_class_c_enum(ptr) as u8,
            Type::Def(def) => match def.kind {
                TypeDefinitionKind::Alias => self.ir_runtime_class_c_enum(&def.ty) as u8,
                TypeDefinitionKind::Struct => {
                    let asset = Asset::get_from_context(self, def.as_ref());
                    if asset.is_some() {
                        IR_RUNTIME_ASSET_TYPE
                    } else {
                        IR_RUNTIME_STRUCT_TYPE
                    }
                }
                _ => unimplemented!(),
            },
            Type::Array { elem: _, len: _ } => IR_RUNTIME_ARRAY_TYPE,
            Type::Map { key: _, value: _ } => IR_RUNTIME_MAP_TYPE,
            Type::Compound(_) => unimplemented!(),
            Type::Tuple(_) => unimplemented!(),
            Type::Builtin(_) => unimplemented!(),
        }) as u32
    }

    pub fn ir_expr_ty(&self, func: &FunctionDefinition, expr: &Expr) -> Option<Type> {
        match expr {
            Expr::Identifier(id) => func.vars.get(id).cloned(),
            Expr::Instr(instr) => match &instr.inner {
                InstrDescription::Declaration {
                    id: _,
                    init_val: _,
                    ty: _,
                } => None,
                InstrDescription::Assignment { id: _, val: _ } => None,
                InstrDescription::Ret { val: _ } => None,
                InstrDescription::Br { target: _ } => None,
                InstrDescription::BrIf {
                    cond: _,
                    then_bb: _,
                    else_bb: _,
                } => None,
                InstrDescription::Match {
                    val: _,
                    otherwise: _,
                    jump_table: _,
                } => None,
                InstrDescription::Not { op: _ } => Some(Type::bool()),
                InstrDescription::BitNot { op } => self.ir_expr_ty(func, op),
                InstrDescription::Binary {
                    op_code: _,
                    op1,
                    op2: _,
                } => self.ir_expr_ty(func, op1),
                InstrDescription::Cmp {
                    op_code: _,
                    op1: _,
                    op2: _,
                } => Some(Type::bool()),
                InstrDescription::Alloca { ty } => Some(self.ptr_type_to(ty.clone())),
                InstrDescription::Malloc { ty } => Some(self.ptr_type_to(ty.clone())),
                InstrDescription::Free { ptr: _ } => None,
                InstrDescription::GetField {
                    ptr: _,
                    field_path: _,
                    field_ty,
                } => Some(field_ty.clone()),
                InstrDescription::SetField {
                    ptr: _,
                    val: _,
                    field_path: _,
                } => None,
                InstrDescription::GetStoragePath { storage_path: _ } => Some(Type::storage_path()),
                InstrDescription::StorageLoad {
                    storage_path: _,
                    load_ty,
                } => Some(load_ty.clone()),
                InstrDescription::StorageStore {
                    storage_path: _,
                    store_val: _,
                } => None,
                InstrDescription::Call {
                    func_name: _,
                    args: _,
                    ret_ty,
                } => Some(ret_ty.clone()),
                InstrDescription::IntCast { val: _, target_ty } => Some(target_ty.clone()),
            },
            Expr::Literal(lit) => Some(lit.literal_type()),
            Expr::NOP => unreachable!(),
        }
    }

    /// Native pointer type of `ty`.
    #[inline]
    pub fn ptr_type_to(&self, ty: cfg::Type) -> cfg::Type {
        cfg::Type::Pointer(Rc::new(ty))
    }
}
