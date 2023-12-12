// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::cfg::IntLiteral;
use crate::ir::cfg::Literal;
use crate::ir::cfg::MetaData;
use crate::ir::cfg::MetaDataNode;
use crate::ir::context::IRContext;
use smart_ir_macro::MetadataDefinition;

/// debug location metadata
/// !{ {line}: u64, {column}: u64, {file}: str}
/// e.g.  %0 = add(1, 2) !ir_debug_location !0
///       !0 = !{2: u64, 5: u64, "/Users/admin/ir/test.ir": str}
#[derive(Clone, Debug, PartialEq, Eq, MetadataDefinition, Default)]
#[MetaDataKey(ir_debug_location)]
pub struct DebugLocation {
    start_line: u32,
    end_line: u32,
    file: String,
}

#[cfg(test)]
mod debug_info_test {

    use crate::ir::cfg::InstrDescription;

    use super::DebugLocation;
    use crate::ir::cfg::Instr;
    use crate::ir::context::IRContext;

    #[test]
    fn test_debug_info_location_normal() {
        let mut ctx = IRContext::default();
        let loc = DebugLocation::default()
            .start_line(2)
            .end_line(5)
            .file("/user/admin/ir/test.ir".to_string());
        let mut instr = Instr::new(InstrDescription::br(0));

        DebugLocation::add_to_context(&mut ctx, &mut instr, &loc);

        let result = DebugLocation::get_from_context(&ctx, &instr);
        assert!(result.is_some());
        assert_eq!(loc, result.unwrap());
    }
}
