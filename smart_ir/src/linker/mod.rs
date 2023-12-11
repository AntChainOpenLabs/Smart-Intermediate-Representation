// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

mod util;
mod wasm;

use crate::ir_config::{IROptions, OptimizationLevel, Target};
use std::ffi::CString;

/// Take an object file and turn it into a final linked binary ready for deployment.
/// The lld linker is totally not thread-safe.
/// Ref: https://github.com/llvm/llvm-project/blob/main/lld/tools/lld/lld.cpp
pub fn link(
    input: &[u8],
    name: &str,
    export_names: &[String],
    target: Target,
    opt_level: &OptimizationLevel,
    no_contract: bool,
    opts: &IROptions,
) -> Vec<u8> {
    // DO NOT use a linker lock to speedup the parallel test.
    match &target {
        Target::Wasm => wasm::link(input, name, export_names, opt_level, no_contract, opts),
        Target::Generic => unimplemented!("generic target link"),
    }
}

#[cfg(feature = "ir_release")]
extern "C" {
    fn LLDWasmLink(args: *const *const libc::c_char, size: libc::size_t) -> libc::c_int;
}

#[cfg(feature = "ir_release")]
pub fn wasm_linker(args: &[CString]) -> bool {
    let mut command_line: Vec<*const libc::c_char> = Vec::with_capacity(args.len() + 1);

    let executable_name = CString::new("wasm-ld").unwrap();

    command_line.push(executable_name.as_ptr());

    for arg in args {
        command_line.push(arg.as_ptr());
    }

    unsafe { LLDWasmLink(command_line.as_ptr(), command_line.len()) == 0 }
}

#[cfg(not(feature = "ir_release"))]
pub fn wasm_linker(args: &[CString]) -> bool {
    use std::process::Command;

    let mut command_line: Vec<String> = Vec::with_capacity(args.len() + 1);

    for arg in args {
        command_line.push(arg.to_str().unwrap().to_string());
    }

    let result = Command::new("wasm-ld")
        .args(&command_line)
        .output()
        .expect("wasm-ld run error");
    if !result.stderr.is_empty() {
        println!("{}", String::from_utf8(result.stderr).unwrap());
    }
    false
}
