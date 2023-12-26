// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use super::BackendTypes;
/// BuilderMethods defines SSA builder methods including calculation, condition, SSA instructions etc.
pub trait BuilderMethods: BackendTypes {
    /// SSA append a basic block named `name`.
    fn append_block(&self, name: &str) -> Self::BasicBlock;
    /// SSA ret instruction.
    fn ret_void(&self);
    /// SSA ret instruction with returned value.
    fn ret(&self, v: Self::Value);
    /// SSA br instruction.
    fn br(&self, dest: Self::BasicBlock);
    /// SSA cond br instruction.
    fn cond_br(&self, cond: Self::Value, then_bb: Self::BasicBlock, else_bb: Self::BasicBlock);
    /// SSA load instruction.
    fn load(&self, pointee_ty: Self::Type, ptr: Self::Value, name: &str) -> Self::Value;
    /// SSA cast int to pointer.
    fn int_to_ptr(&self, val: Self::Value, dest_ty: Self::Type) -> Self::Value;
    /// SSA bit cast.
    fn bit_cast(&self, val: Self::Value, dest_ty: Self::Type) -> Self::Value;
    /// SSA int cast.
    fn int_cast(&self, val: Self::Value, dest_ty: Self::Type, is_signed: bool) -> Self::Value;
    /// SSA pointer cast.
    fn ptr_cast(&self, val: Self::Value, dest_ty: Self::Type) -> Self::Value;
    /// Lookup a known function named `name`.
    fn lookup_function(&self, name: &str) -> Self::Function;
}
