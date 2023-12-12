// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::cfg::Literal;
use crate::ir::cfg::MetaData;
use crate::ir::cfg::MetaDataNode;
use crate::ir::context::IRContext;
use smart_ir_macro::MetadataDefinition;

#[derive(Clone, Debug, PartialEq, Eq, MetadataDefinition, Default)]
#[MetaDataKey(extend_hostapi)]
pub struct ExtendHostAPI {
    name: String,
}
