// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::integration::intrinsic::IrHostAPIIntrinsic;
use crate::ir::cfg::Type;
use crate::ir::interface_type::IntrinsicFuncName;
use crate::ir_codegen::context::IR2LLVMCodeGenContext;
use inkwell::values::{BasicValueEnum, FunctionValue};

pub trait ExtendContext {
    fn get_ir_func_intrinsics(&self) -> &[&IrHostAPIIntrinsic];

    fn find_ir_func_intrinsic_by_func_name(
        &self,
        func_name: &IntrinsicFuncName,
    ) -> Option<&IrHostAPIIntrinsic>;

    fn add_or_get_intrinsic_function<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        intrinsic_info: &IrHostAPIIntrinsic,
        params_ty: &[Type],
        ret: &Type,
    ) -> FunctionValue<'ctx>;

    fn asset_get_data_length<'ctx>(
        &self,
        asset_tag: u32,
        context: &IR2LLVMCodeGenContext<'ctx>,
        path_ptr: BasicValueEnum<'ctx>,
    ) -> inkwell::values::IntValue<'ctx>;

    fn asset_get_data<'ctx>(
        &self,
        asset_tag: u32,
        context: &IR2LLVMCodeGenContext<'ctx>,
        path_ptr: BasicValueEnum<'ctx>,
        data: BasicValueEnum<'ctx>,
    );

    fn asset_set_data<'ctx>(
        &self,
        asset_tag: u32,
        context: &IR2LLVMCodeGenContext<'ctx>,
        path_ptr: BasicValueEnum<'ctx>,
        value_ptr: BasicValueEnum<'ctx>,
        value_length: BasicValueEnum<'ctx>,
    );

    fn issue_asset_or_token<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx>;

    fn burn_asset_or_token<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx>;

    fn destory_asset_or_token<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx>;

    fn transfer_asset_or_token<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx>;

    fn has_asset<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx>;

    fn get_balance<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx>;

    fn check_asset_is_fungible<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx>;

    fn build_storage_set_bss<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        _ret: &Type,
    ) -> BasicValueEnum<'ctx>;

    fn build_storage_get_bss<'ctx>(
        &self,
        context: &IR2LLVMCodeGenContext<'ctx>,
        params: &[BasicValueEnum<'ctx>],
        _params_ty: &[Type],
        ret: &Type,
    ) -> BasicValueEnum<'ctx>;

    fn init_extend_host_apis(&self, context: &IR2LLVMCodeGenContext<'_>);

    fn all_extend_host_api_names(&self) -> Vec<String>;
}

static mut EXTEND_CONTEXT_INSTANCE: Option<Box<dyn ExtendContext>> = None;

pub fn has_extend_context() -> bool {
    unsafe { EXTEND_CONTEXT_INSTANCE.is_some() }
}

#[allow(clippy::borrowed_box)]
pub fn get_extend_context() -> &'static Box<dyn ExtendContext> {
    unsafe { EXTEND_CONTEXT_INSTANCE.as_ref().unwrap() }
}

pub fn set_extend_context(inst: Box<dyn ExtendContext>) {
    unsafe {
        EXTEND_CONTEXT_INSTANCE = Some(inst);
    }
}
