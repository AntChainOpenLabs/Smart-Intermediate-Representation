// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;

use crate::ir::context::IRContext;
use crate::ir_config::IROptions;
use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;
use once_cell::sync::OnceCell;

use self::context::IR2LLVMCodeGenContext;

pub mod abi;
pub mod builtin_constants;
mod class_generator;
pub mod common;
mod const_storage_path_generator;
pub mod context;
mod encoding;
pub mod error;
mod intrinsic;
mod ir;
mod storage_path;
pub mod traits;
mod ty;
mod val;

static LLVM_INIT: OnceCell<()> = OnceCell::new();
/// The runner main function name.
pub const MODULE_NAME: &str = "ir_main";
/// Generate LLVM IR of ast module.
pub fn ir_emit_code(
    ir_ctx: &IRContext,
    abi_names: RefCell<Vec<String>>,
    opts: &IROptions,
    extend_runtime: Vec<&[u8]>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    LLVM_INIT.get_or_init(|| {
        inkwell::targets::Target::initialize_webassembly(&Default::default());
    });
    // Create a LLVM context
    let context = Context::create();
    // Create a empty LLVM module
    let module = context.create_module(MODULE_NAME);
    // Link stdlib
    let intr = load_stdlib(&context, extend_runtime);
    module.link_in_module(intr)?;
    let ctx = IR2LLVMCodeGenContext::new(ir_ctx, &context, module, opts, abi_names);
    traits::emit_code(ctx, opts)
}

/// Load standard libraries.
pub fn load_stdlib<'a>(context: &'a Context, a: Vec<&'a [u8]>) -> Module<'a> {
    let memory = MemoryBuffer::create_from_memory_range(WASM_IR[0], "wasm_bc");

    let module = Module::parse_bitcode_from_buffer(&memory, context).unwrap();

    for bc in WASM_IR.iter().skip(1) {
        let memory = MemoryBuffer::create_from_memory_range(bc, "wasm_bc");

        module
            .link_in_module(Module::parse_bitcode_from_buffer(&memory, context).unwrap())
            .unwrap();
    }

    for bc in a.iter() {
        let memory = MemoryBuffer::create_from_memory_range(bc, "wasm_bc");

        module
            .link_in_module(Module::parse_bitcode_from_buffer(&memory, context).unwrap())
            .unwrap();
    }

    module
}

static WASM_IR: [&[u8]; 21] = [
    include_bytes!("../runtime/stdlib/wasm/stdlib.bc"),
    include_bytes!("../runtime/stdlib/wasm/ir_type.bc"),
    include_bytes!("../runtime/stdlib/wasm/wasmheap.bc"),
    include_bytes!("../runtime/stdlib/wasm/base64.bc"),
    include_bytes!("../runtime/stdlib/wasm/hex.bc"),
    include_bytes!("../runtime/stdlib/wasm/mycrypto.bc"),
    include_bytes!("../runtime/stdlib/wasm/ssz.bc"),
    include_bytes!("../runtime/stdlib/wasm/data_stream.bc"),
    include_bytes!("../runtime/stdlib/wasm/data_stream_builtin.bc"),
    include_bytes!("../runtime/stdlib/wasm/call_log.bc"),
    include_bytes!("../runtime/stdlib/wasm/qvector.bc"),
    include_bytes!("../runtime/stdlib/wasm/qhash.bc"),
    include_bytes!("../runtime/stdlib/wasm/qhashtbl.bc"),
    include_bytes!("../runtime/stdlib/wasm/qstring.bc"),
    include_bytes!("../runtime/stdlib/wasm/math.bc"),
    include_bytes!("../runtime/stdlib/wasm/mycov.bc"),
    include_bytes!("../runtime/stdlib/wasm/cJSON.bc"),
    include_bytes!("../runtime/stdlib/wasm/json.bc"),
    include_bytes!("../runtime/stdlib/wasm/stream.bc"),
    include_bytes!("../runtime/stdlib/wasm/rlp.bc"),
    include_bytes!("../runtime/stdlib/wasm/chain.bc"),
];
