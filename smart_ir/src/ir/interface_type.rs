// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::builder::MetaDataId;
use std::collections::HashMap;

pub trait PartialFuncNameBehavior {
    fn apply_name(&self) -> String;
}

impl PartialFuncNameBehavior for () {
    fn apply_name(&self) -> String {
        unreachable!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PartialFuncNameKind<T: PartialFuncNameBehavior, U: PartialFuncNameBehavior> {
    UserDefFunc(String),
    Intrinsic(T),
    HostAPI(U),
    Otherwise,
}

/// Specify the specification of HostAPI here
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DefaultHostAPI {}

static mut REGISTERED_INTRINSIC_FUNC_NAMES: Option<HashMap<String, IntrinsicFuncName>> = None;

/// Specify the specification of Intrinsic functions here
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntrinsicFuncName {
    pub key: &'static str,
    pub func_name: &'static str,
}
impl IntrinsicFuncName {
    pub fn new(key: &'static str, func_name: &'static str) -> IntrinsicFuncName {
        IntrinsicFuncName { key, func_name }
    }
}
pub fn register_intrinsic_func_name(intrinsic_func_name: &IntrinsicFuncName) {
    unsafe {
        if REGISTERED_INTRINSIC_FUNC_NAMES.is_none() {
            REGISTERED_INTRINSIC_FUNC_NAMES = Some(HashMap::new());
        }
        {
            if REGISTERED_INTRINSIC_FUNC_NAMES
                .as_ref()
                .unwrap()
                .get(intrinsic_func_name.key)
                .is_some()
            {
                return;
            }
        }
        let borrowed = REGISTERED_INTRINSIC_FUNC_NAMES.as_mut().unwrap();
        borrowed.insert(
            intrinsic_func_name.key.to_string(),
            intrinsic_func_name.clone(),
        );
    };
}
pub fn get_all_intrinsic_func_names() -> &'static HashMap<String, IntrinsicFuncName> {
    unsafe {
        if REGISTERED_INTRINSIC_FUNC_NAMES.is_none() {
            REGISTERED_INTRINSIC_FUNC_NAMES = Some(HashMap::new());
        }
        REGISTERED_INTRINSIC_FUNC_NAMES.as_ref().unwrap()
    }
}

pub fn get_intrinsic_func_by_key(key: &str) -> Option<IntrinsicFuncName> {
    for (item_key, value) in get_all_intrinsic_func_names().iter() {
        if item_key == key {
            return Some(value.clone());
        }
    }
    None
}

pub fn get_intrinsic_func_by_func_name(func_name: &str) -> Option<IntrinsicFuncName> {
    for (_key, value) in get_all_intrinsic_func_names().iter() {
        if value.func_name == func_name {
            return Some(value.clone());
        }
    }
    None
}

/// Default name of Intrinsic functions
/// OVERRIDE `apply_name` to concrete exact function name in later codegen stage
impl PartialFuncNameBehavior for IntrinsicFuncName {
    fn apply_name(&self) -> String {
        self.func_name.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartialFuncName {
    pub kind: PartialFuncNameKind<IntrinsicFuncName, ()>,
    pub metadata: Option<MetaDataId>,
}

impl Default for PartialFuncName {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialFuncName {
    pub fn new() -> Self {
        Self {
            kind: PartialFuncNameKind::Otherwise,
            metadata: None,
        }
    }

    pub fn get_name(&self) -> String {
        match &self.kind {
            PartialFuncNameKind::UserDefFunc(str) => str.clone(),
            PartialFuncNameKind::Intrinsic(intrinsic) => intrinsic.apply_name(),
            PartialFuncNameKind::HostAPI(_) => unimplemented!(),
            PartialFuncNameKind::Otherwise => unreachable!(),
        }
    }
}

impl From<String> for PartialFuncName {
    fn from(val: String) -> Self {
        let mut p = PartialFuncName::new();
        p.kind = PartialFuncNameKind::UserDefFunc(val);
        p
    }
}

impl From<IntrinsicFuncName> for PartialFuncName {
    fn from(val: IntrinsicFuncName) -> Self {
        let mut p = PartialFuncName::new();
        p.kind = PartialFuncNameKind::Intrinsic(val);
        p
    }
}
impl From<String> for IntrinsicFuncName {
    fn from(s: String) -> IntrinsicFuncName {
        match parse_intrinsic_func_name(&s) {
            Some(intrinsic) => intrinsic,
            None => unimplemented!("api {} unimplemented", s),
        }
    }
}

pub(crate) fn parse_intrinsic_func_name(func_name: &str) -> Option<IntrinsicFuncName> {
    for (_key, value) in get_all_intrinsic_func_names().iter() {
        if value.func_name == func_name {
            return Some(value.clone());
        }
    }
    None
}
