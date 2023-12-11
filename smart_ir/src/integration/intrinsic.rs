// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::interface_type::IntrinsicFuncName;

pub struct IrHostapiIntrinsic {
    // enum name as key
    pub func_name: IntrinsicFuncName,
    pub ir_func_name: &'static str,
}
