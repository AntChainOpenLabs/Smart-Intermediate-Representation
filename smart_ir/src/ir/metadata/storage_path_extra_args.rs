// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::cfg::Literal;
use crate::ir::cfg::MetaData;
use crate::ir::cfg::{IntLiteral, MetaDataNode};
use crate::ir::context::IRContext;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct StoragePathExtraArgs {
    extra_args: Vec<u32>,
}

impl StoragePathExtraArgs {
    pub fn extra_args(mut self, value: Vec<u32>) -> Self {
        self.extra_args = value;
        self
    }

    pub fn get_extra_args(&self) -> &Vec<u32> {
        &self.extra_args
    }
    pub fn from(metadata: &MetaData) -> Result<Self, String> {
        Ok(Self {
            extra_args: metadata
                .data
                .iter()
                .map(|lit| lit.get_u32().unwrap())
                .collect(),
        })
    }
    pub fn to_metadata(&self) -> MetaData {
        let mut metadata = MetaData::default();
        for extra_arg in self.extra_args.iter() {
            metadata.push_field(Literal::Int(IntLiteral::U32(*extra_arg)));
        }
        metadata
    }
    pub fn get_metadata_key(&self) -> String {
        "ir_storage_path_extra_args".to_string()
    }
    pub fn add_to_context(
        ctx: &IRContext,
        md_node: &mut dyn MetaDataNode,
        loc: &StoragePathExtraArgs,
    ) {
        let md_idx = ctx.add_metadata(loc.to_metadata());
        let metadata = md_node.get_metadata_mut();
        metadata.insert("ir_storage_path_extra_args".to_string(), md_idx);
    }
    pub fn get_from_context(
        ctx: &IRContext,
        md_node: &dyn MetaDataNode,
    ) -> Option<StoragePathExtraArgs> {
        let metadata = md_node.get_metadata();
        let md_idx = metadata.get("ir_storage_path_extra_args")?;
        StoragePathExtraArgs::from(ctx.get_metadata(md_idx).as_ref()?).ok()
    }
}
