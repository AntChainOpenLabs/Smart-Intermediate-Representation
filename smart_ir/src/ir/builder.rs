// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use indexmap::IndexMap;

use super::cfg::{BasicBlock, ControlFlowGraph, FunctionDefinition};
use super::cfg::{Expr, Instr};
use crate::ir::cfg::InstrDescription;
use crate::ir::cfg::Type;
use crate::ir::context::IRContext;
use crate::ir::interface_type::PartialFuncName;
use crate::ir::metadata::debug_info::DebugLocation;
use std::cell::RefCell;

/// The Builder and BuilderContext only maintains current functions and basic blocks.
/// After parsing, IRContext is required to save the FunctionDefinition
#[derive(Debug, Clone, Default)]
pub struct Builder {
    id_generator: IdGenerator,
    pub context: BuilderContext,
}

#[derive(Debug, Clone, Default)]
pub struct Label {
    pub break_target_label: RefCell<BasicBlock>,
    pub continue_target_label: RefCell<BasicBlock>,
}

impl Label {
    pub fn new(break_target_label: &BasicBlock, continue_target_label: &BasicBlock) -> Self {
        Self {
            break_target_label: RefCell::new(break_target_label.clone()),
            continue_target_label: RefCell::new(continue_target_label.clone()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BuilderContext {
    pub current_function: RefCell<Option<FunctionDefinition>>,
    pub current_bb: RefCell<Option<BasicBlock>>,
    pub current_loc: RefCell<Option<MetaDataId>>,
    pub labels: RefCell<Vec<Label>>,
}

pub type IdentifierId = u32;
pub type BasicBlockId = u32;
pub type MetaDataId = u32;

#[derive(Debug, Default, Clone)]
pub struct IdGenerator {
    ident: RefCell<IdentifierId>,
    basic_block: RefCell<BasicBlockId>,
    metadata: RefCell<MetaDataId>,
}

impl IdGenerator {
    fn get_ident_id(&self) -> u32 {
        let mut id = self.ident.borrow_mut();
        let result = *id;
        *id += 1;
        result
    }

    fn reset_ident_id(&self) {
        let mut id = self.ident.borrow_mut();
        *id = 0;
    }

    fn get_bb_id(&self) -> u32 {
        let mut id = self.basic_block.borrow_mut();
        let result = *id;
        *id += 1;
        result
    }

    fn get_metadata_id(&self) -> u32 {
        let mut id = self.metadata.borrow_mut();
        let result = *id;
        *id += 1;
        result
    }
}

impl Builder {
    // instr
    pub fn instr_br_if(&self, cond: Expr, if_bb: &BasicBlock, else_bb: &BasicBlock) -> Instr {
        Instr::new(InstrDescription::br_if(
            cond,
            if_bb.get_id(),
            else_bb.get_id(),
        ))
    }

    pub fn instr_bit_xor(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::bit_xor(op1, op2))
    }

    pub fn instr_not(&self, op: Expr) -> Instr {
        Instr::new(InstrDescription::not(op))
    }

    pub fn instr_eq(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::eq(op1, op2))
    }

    pub fn instr_and(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::and(op1, op2))
    }

    pub fn instr_or(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::or(op1, op2))
    }

    pub fn instr_ne(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::ne(op1, op2))
    }

    pub fn instr_add(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::add(op1, op2))
    }

    pub fn instr_sub(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::sub(op1, op2))
    }

    pub fn instr_mul(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::mul(op1, op2))
    }

    pub fn instr_div(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::div(op1, op2))
    }

    pub fn instr_mod(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::r#mod(op1, op2))
    }

    pub fn instr_pow(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::exp(op1, op2))
    }

    pub fn instr_lshift(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::shl(op1, op2))
    }

    pub fn instr_rshift(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::shr(op1, op2))
    }

    pub fn instr_bit_and(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::bit_and(op1, op2))
    }

    pub fn instr_bit_or(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::bit_or(op1, op2))
    }

    pub fn instr_bit_not(&self, op: Expr) -> Instr {
        Instr::new(InstrDescription::bit_not(op))
    }

    pub fn instr_lt(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::lt(op1, op2))
    }

    pub fn instr_le(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::le(op1, op2))
    }

    pub fn instr_gt(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::gt(op1, op2))
    }

    pub fn instr_ge(&self, op1: Expr, op2: Expr) -> Instr {
        Instr::new(InstrDescription::ge(op1, op2))
    }

    pub fn instr_get_storage_path(&self, storage_path: Vec<Expr>) -> Instr {
        Instr::new(InstrDescription::get_storage_path(storage_path))
    }

    pub fn instr_storage_load(&self, storage_path: Expr, load_ty: Type) -> Instr {
        Instr::new(InstrDescription::storage_load(storage_path, load_ty))
    }

    pub fn instr_storage_store(&self, storage_path: Expr, store_val: Expr) -> Instr {
        Instr::new(InstrDescription::storage_store(storage_path, store_val))
    }

    pub fn instr_int_cast(&self, val: Expr, target_ty: Type) -> Instr {
        Instr::new(InstrDescription::int_cast(val, target_ty))
    }

    pub fn instr_call(&self, func_name: PartialFuncName, args: Vec<Expr>, ret_ty: Type) -> Instr {
        Instr::new(InstrDescription::call(func_name, args, ret_ty))
    }

    pub fn instr_get_field(&self, ptr: Expr, field_path: Vec<u32>, field_ty: Type) -> Instr {
        Instr::new(InstrDescription::get_field(ptr, field_path, field_ty))
    }

    pub fn instr_alloca(&self, ty: Type) -> Instr {
        Instr::new(InstrDescription::alloca(ty))
    }

    pub fn instr_malloc(&self, ty: Type) -> Instr {
        Instr::new(InstrDescription::malloc(ty))
    }

    pub fn instr_free(&self, ptr: Expr) -> Instr {
        Instr::new(InstrDescription::free(ptr))
    }
}

impl Builder {
    pub fn get_ident_id(&self) -> IdentifierId {
        self.id_generator.get_ident_id()
    }

    pub fn rest_ident_id(&self) {
        self.id_generator.reset_ident_id()
    }

    pub fn get_bb_id(&self) -> BasicBlockId {
        self.id_generator.get_bb_id()
    }

    pub fn get_metadata_id(&self) -> MetaDataId {
        self.id_generator.get_metadata_id()
    }

    // build Expr
    pub fn create_identifier(&self, ty: Type) -> Expr {
        let id = self.get_ident_id();
        self.add_ir_ty(id, ty);
        Expr::Identifier(id)
    }

    pub fn build_identifier(&self, id: &u32) -> Expr {
        Expr::Identifier(*id)
    }

    pub fn build_nop(&self) -> Expr {
        Expr::NOP
    }

    // build instr
    pub fn build_declaration(&self, id: IdentifierId, init_val: Option<Expr>, ty: Type) {
        let instr = Instr::new(InstrDescription::declaration(id, init_val, ty));
        self.insert_instr(instr);
    }

    pub fn build_assignment(&self, id: IdentifierId, val: Expr) {
        let instr = Instr::new(InstrDescription::assignment(id, val));
        self.insert_instr(instr);
    }

    pub fn build_ret(&self, val: Option<Expr>) {
        let instr = Instr::new(InstrDescription::ret(val));
        self.insert_instr(instr);
    }

    pub fn build_br(&self, dest: &BasicBlock) {
        let instr = Instr::new(InstrDescription::br(dest.get_id()));
        self.insert_instr(instr);
    }

    pub fn build_cond_br(&self, cond: Expr, if_bb: &BasicBlock, else_bb: &BasicBlock) {
        let instr = self.instr_br_if(cond, if_bb, else_bb);
        self.insert_instr(instr);
    }

    pub fn build_match(&self, val: Expr, otherwise: BasicBlockId, jump_table: IndexMap<u32, u32>) {
        let instr = Instr::new(InstrDescription::r#match(val, otherwise, jump_table));
        self.insert_instr(instr);
    }

    pub fn build_not(&self, op: Expr) {
        let instr = Instr::new(InstrDescription::not(op));
        self.insert_instr(instr);
    }

    pub fn build_bit_not(&self, op: Expr) {
        let instr = Instr::new(InstrDescription::bit_not(op));
        self.insert_instr(instr);
    }

    pub fn build_add(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::add(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_sub(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::sub(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_mul(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::mul(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_div(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::mul(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_mod(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::r#mod(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_and(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::and(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_bit_xor(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::bit_xor(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_bit_and(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::bit_and(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_or(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::or(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_bit_or(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::bit_or(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_shl(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::shl(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_shr(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::shr(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_sar(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::sar(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_eq(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::eq(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_ne(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::ne(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_gt(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::gt(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_ge(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::ge(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_lt(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::lt(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_le(&self, op1: Expr, op2: Expr) {
        let instr = Instr::new(InstrDescription::le(op1, op2));
        self.insert_instr(instr);
    }

    pub fn build_alloca(&self, ty: Type) {
        let instr = Instr::new(InstrDescription::alloca(ty));
        self.insert_instr(instr);
    }

    pub fn build_malloc(&self, ty: Type) {
        let instr = Instr::new(InstrDescription::malloc(ty));
        self.insert_instr(instr);
    }

    pub fn build_free(&self, ptr: Expr) {
        let instr = Instr::new(InstrDescription::free(ptr));
        self.insert_instr(instr);
    }

    pub fn build_get_field(&self, ptr: Expr, field_path: Vec<u32>, field_ty: Type) {
        let instr = Instr::new(InstrDescription::get_field(ptr, field_path, field_ty));
        self.insert_instr(instr);
    }

    pub fn build_set_field(&self, ptr: Expr, field_path: Vec<u32>, val: Expr) {
        let instr = Instr::new(InstrDescription::set_field(ptr, field_path, val));
        self.insert_instr(instr);
    }

    pub fn build_get_storage_path(&self, storage_path: Vec<Expr>) {
        let instr = Instr::new(InstrDescription::get_storage_path(storage_path));
        self.insert_instr(instr);
    }

    pub fn build_storage_load(&self, storage_path: Expr, load_ty: Type) {
        let instr = Instr::new(InstrDescription::storage_load(storage_path, load_ty));
        self.insert_instr(instr);
    }

    pub fn build_storage_store(&self, storage_path: Expr, store_val: Expr) {
        let instr = Instr::new(InstrDescription::storage_store(storage_path, store_val));
        self.insert_instr(instr);
    }

    pub fn build_call(&self, func_name: PartialFuncName, args: Vec<Expr>, ret_ty: Type) {
        let instr = Instr::new(InstrDescription::call(func_name, args, ret_ty));
        self.insert_instr(instr);
    }

    pub fn build_int_cast(&self, val: Expr, target_ty: Type) {
        let instr = Instr::new(InstrDescription::int_cast(val, target_ty));
        self.insert_instr(instr);
    }

    pub fn insert_instr(&self, mut instr: Instr) {
        let mut ref_bb = self.context.current_bb.borrow_mut();
        if let Some(bb) = ref_bb.as_mut() {
            let cur_loc = self.context.current_loc.borrow();
            if let Some(id) = cur_loc.as_ref() {
                instr
                    .metadata
                    .insert(DebugLocation::get_metadata_key(), *id);
            }
            bb.insert_instr(instr);
        }
    }

    // BasicBlock builder functions

    /// Append basic block into build.context.current_function
    pub fn append_basic_block(&self, _name: &str) -> BasicBlock {
        let id = self.id_generator.get_bb_id();
        let mut curr_func = self.context.current_function.borrow_mut();
        if let Some(func) = curr_func.as_mut() {
            func.cfg.append_new_block(id)
        } else {
            unreachable!()
        }
    }

    pub fn update_debug_location(&self, id: MetaDataId) {
        let mut curr_loc = self.context.current_loc.borrow_mut();
        *curr_loc = Some(id);
    }

    /// Change the current_bb to dest
    pub fn position_at_end(&self, dest: &BasicBlock) {
        self.save_bb();
        let mut curr_bb = self.context.current_bb.borrow_mut();
        *curr_bb = Some(dest.clone());
    }

    pub fn build_function(
        &self,
        name: &str,
        params: Vec<Type>,
        ret: Type,
        is_external: bool,
        vars: IndexMap<IdentifierId, Type>,
    ) {
        let id = self.get_bb_id();
        let func = FunctionDefinition {
            name: name.to_string(),
            params,
            ret,
            is_external,
            cfg: ControlFlowGraph::new(id),
            vars,
            metadata: IndexMap::new(),
        };
        // set current function
        let mut curr_func = self.context.current_function.borrow_mut();
        *curr_func = Some(func.clone());
        // set current basic block
        let bb = func.cfg.get_entry_block();
        let mut curr_bb = self.context.current_bb.borrow_mut();
        *curr_bb = Some(bb);
    }

    /// Save the current basic block into current_function
    pub fn save_bb(&self) {
        let mut curr_bb = self.context.current_bb.borrow_mut();
        if let Some(bb) = curr_bb.as_mut() {
            let mut curr_func = self.context.current_function.borrow_mut();
            if let Some(func) = curr_func.as_mut() {
                func.cfg.update_block(bb.get_id(), bb.clone());
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    pub fn add_ir_ty(&self, id: IdentifierId, ty: Type) {
        let mut curr_func = self.context.current_function.borrow_mut();
        if let Some(func) = curr_func.as_mut() {
            func.vars.insert(id, ty);
        } else {
            unreachable!("not found current function")
        }
    }
}

impl IRContext {
    pub fn append_block(&self, name: &str) -> BasicBlock {
        self.builder.append_basic_block(name)
    }

    /// pop last basic block in current_bb to the current_func
    pub fn func_end(&self) {
        self.builder.save_bb();
    }
}
