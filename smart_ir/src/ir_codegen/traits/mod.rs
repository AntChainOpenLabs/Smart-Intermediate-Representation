// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

mod backends;
mod builder;
mod intrinsic;
mod ty;
mod value;

use crate::ir_config::IROptions;
pub use backends::*;
pub use builder::*;
pub use intrinsic::*;
pub use ty::*;

/// CodeGenContext is a trait used by the compiler to emit code to different targets.
pub trait CodeGenContext {
    fn emit(&self, opts: &IROptions) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

/// Emit code with the options using CodeGenContext.
pub fn emit_code(
    ctx: impl CodeGenContext,
    opts: &IROptions,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    ctx.emit(opts)
}
