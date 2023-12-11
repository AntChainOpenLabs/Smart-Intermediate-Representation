// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::values::IntValue;
use inkwell::values::{BasicValueEnum, FunctionValue};
use smart_ir::integration::intrinsic::IrHostapiIntrinsic;
use smart_ir::ir::cfg::Type;
use smart_ir::ir::interface_type::IntrinsicFuncName;
use smart_ir::ir_codegen::common::global::ExtendContext;
use smart_ir::ir_codegen::context::IR2LLVMCodeGenContext;

pub struct MockExtendContext {}

impl MockExtendContext {
    pub fn new() -> Self {
        MockExtendContext {}
    }
}

impl Default for MockExtendContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtendContext for MockExtendContext {
    fn asset_get_data_length<'ctx>(
        &self,
        _asset_tag: u32,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _path_ptr: BasicValueEnum<'ctx>,
    ) -> IntValue<'ctx> {
        context.i32_value(0).into_int_value()
    }

    fn asset_get_data<'ctx>(
        &self,
        _asset_tag: u32,
        _context: &IR2LLVMCodeGenContext<'ctx>,
        _path_ptr: BasicValueEnum<'ctx>,
        _data: BasicValueEnum<'ctx>,
    ) {
    }

    fn asset_set_data<'ctx>(
        &self,
        _asset_tag: u32,
        _context: &IR2LLVMCodeGenContext<'ctx>,
        _path_ptr: BasicValueEnum<'ctx>,
        _value_ptr: BasicValueEnum<'ctx>,
        _value_length: BasicValueEnum<'ctx>,
    ) {
    }

    fn issue_asset_or_token<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        context.i32_value(0)
    }

    fn burn_asset_or_token<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        context.i32_value(0)
    }

    fn destory_asset_or_token<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        context.i32_value(0)
    }

    fn transfer_asset_or_token<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        context.i32_value(0)
    }

    fn has_asset<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        context.i32_value(0)
    }

    fn get_balance<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        context.i32_value(0)
    }

    fn check_asset_is_fungible<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        context.i32_value(0)
    }

    fn build_storage_set_bss<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        context.i32_value(0)
    }

    fn build_storage_get_bss<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        _params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx> {
        context.i32_value(0)
    }

    fn init_extend_host_apis(&self, _context: &IR2LLVMCodeGenContext<'_>) {}

    fn all_extend_host_api_names(&self) -> Vec<String> {
        vec![]
    }

    fn get_ir_func_intrinsics(&self) -> &[&IrHostapiIntrinsic] {
        &[]
    }

    fn find_ir_func_intrinsic_by_func_name(
        &self,
        _func_name: &IntrinsicFuncName,
    ) -> Option<&IrHostapiIntrinsic> {
        None
    }

    fn add_or_get_intrinsic_function<'ctx>(
        &self,
        _context: &IR2LLVMCodeGenContext<'ctx>,
        _intrinsic_info: &IrHostapiIntrinsic,
        _params: &[Type],
        _ret: &Type,
    ) -> FunctionValue<'ctx> {
        unreachable!();
    }
}
