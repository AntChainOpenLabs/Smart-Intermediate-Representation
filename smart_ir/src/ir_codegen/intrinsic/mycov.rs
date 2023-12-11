// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

// coverage test instruction codegen

use inkwell::values::BasicValueEnum;

use crate::ir::cfg::Type;
use crate::ir_codegen::context::IR2LLVMCodeGenContext;
impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    // tail of contract calling(before return and abort) throw event log of coverage, works only when coverage test switch on
    pub fn build_call_mycov_call_dump_log(
        &self,
        params: &[BasicValueEnum<'ctx>],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let success = *params.get(0).unwrap();
        self.build_void_call(
            "ir_builtin_call_coverage_log",
            &[self.get_runtime_ctx(), success],
        );
        self.i32_value(1)
    }
}
