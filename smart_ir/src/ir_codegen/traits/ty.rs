// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir_codegen::abi::AddressSpace;

use super::BackendTypes;

/// BaseTypeMethods defines all native base type APIs, e.g. i8/i16/f32/f64 types, etc.
pub trait BaseTypeMethods: BackendTypes {
    /// Native i8 type
    fn i8_type(&self) -> Self::Type;
    /// Native i32 type
    fn i32_type(&self) -> Self::Type;
    /// Native i64 type
    fn i64_type(&self) -> Self::Type;
    /// Native pointer type of `ty`.
    fn ptr_type_to(&self, ty: Self::Type) -> Self::Type;
    /// Native pointer type of `ty` with the address space.
    fn ptr_type_to_ext(&self, ty: Self::Type, address_space: AddressSpace) -> Self::Type;
    /// Retrieves the bit width of the integer type `self`.
    fn int_width(&self, ty: Self::Type) -> usize;
    /// Native function type
    fn function_let(&self, args: &[Self::Type], ret: Self::Type) -> Self::FunctionLet;
    /// Native void function type
    fn void_function_let(&self, args: &[Self::Type]) -> Self::FunctionLet;
}

/// DerivedTypeMethods defines all extended type APIs.
pub trait DerivedTypeMethods: BaseTypeMethods {
    /// Get the context pointer type.
    fn context_ptr_type(&self) -> Self::Type {
        self.ptr_type_to(self.i32_type())
    }
}

/// TypeCodeGen defines all type APIs.
pub trait TypeCodeGen: DerivedTypeMethods {}
