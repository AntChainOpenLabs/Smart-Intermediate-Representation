// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

//! Reference: https://github.com/rust-lang/rust/blob/master/compiler/rustc_target/src/lib.rs

/// An identifier that specifies the address space that some operation
/// should operate on. Special address spaces have an effect on code generation,
/// depending on the target and the address spaces it implements.
pub struct AddressSpace(pub u32);

impl AddressSpace {
    /// The default address space, corresponding to data space.
    pub const DATA: Self = AddressSpace(0);
}
