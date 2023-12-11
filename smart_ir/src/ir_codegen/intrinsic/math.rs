// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::values::BasicValueEnum;

use crate::ir::cfg::Type;
use crate::ir_codegen::builtin_constants::BUILTIN_FUNCTION_MANGLE_PREFIX;
use crate::ir_codegen::context::IR2LLVMCodeGenContext;

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    pub fn build_math_itoa(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let num = *params.get(0).unwrap();
        let base = *params.get(1).unwrap();
        let num_ty = params_ty.get(0).unwrap();
        let func_name = format!("{BUILTIN_FUNCTION_MANGLE_PREFIX}_{num_ty}_to_str");
        self.build_call(&func_name, &[num, base])
    }

    pub fn build_math_pow(
        &self,
        params: &[BasicValueEnum<'ctx>],
        params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        let num = *params.get(0).unwrap();
        let base = *params.get(1).unwrap();
        let num_ty = params_ty.get(0).unwrap();
        let func_name = format!("{BUILTIN_FUNCTION_MANGLE_PREFIX}_pow_{num_ty}");
        self.build_call(&func_name, &[num, base])
    }
}
