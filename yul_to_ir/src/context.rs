// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;
use smart_ir::ir::{builder::IdentifierId, cfg::Type};

use crate::ast::Object;

pub const WORD_SIZE: usize = 256 / 8;
pub const WORD_TY: Type = Type::u256();
pub const SIGNED_WORD_TY: Type = Type::i256();
pub fn mem_data_ty() -> Type {
    Type::Array {
        elem: Rc::new(Type::u8()),
        len: None,
    }
}

pub type FunctionId = u32;

#[derive(Debug, Default)]
pub struct Yul2IRContext {
    pub ir_context: smart_ir::ir::context::IRContext,
    pub yul_ast: Option<Object>,
    pub vars: RefCell<IndexMap<String, IdentifierId>>,
    pub ret_var_has_init: RefCell<IndexMap<IdentifierId, bool>>,
    pub data_id: RefCell<IdentifierId>,
    pub caller_data_id: RefCell<IdentifierId>,
    pub return_data_id: RefCell<IdentifierId>,
    pub current_module_name: RefCell<String>,
    pub current_contract_name: RefCell<String>,
}

impl Yul2IRContext {
    pub fn new() -> Self {
        Yul2IRContext {
            ..Default::default()
        }
    }

    pub fn new_with_object(object: Object) -> Self {
        Yul2IRContext {
            yul_ast: Some(object),
            ..Default::default()
        }
    }
}
