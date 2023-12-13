// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate clap;

use compiler_base_span::fatal_error::FatalError;
use compiler_base_span::{FilePathMapping, SourceMap};
use ir_cli::abi::IRContractABIMeta;
use ir_cli::vm::WASM_IR;
use ir_cli::vm::{init_mock_runtime, MockRuntime};
use smart_ir::ir::context::IRContext;
use smart_ir::ir::frontend::translate::translate_main_module;
use smart_ir::ir::printer::IRPrinter;
use smart_ir::ir_config::IROptions;
use smart_ir::runtime::vm::VirtualMachine;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use wasmi::*;

fn main() {
    let matches = clap_app!(ir_cli =>
        (version: "0.1.0")
        (@subcommand run =>
            (@arg INPUT: +required +multiple "Sets the input wasm file and ir abi json and args to run")
            (@arg FUNC: -f --func +takes_value +required "Set the func name to call")
            (@arg VERBOSE: -v --verbose "Print test information verbosely")
        )
        (@subcommand build =>
            (@arg INPUT: +required +multiple "Sets the input textual ir file to build")
            (@arg VERBOSE: -v --verbose "Print test information verbosely")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(file_and_args) = matches.values_of("INPUT") {
            let file_and_args: Vec<&str> = file_and_args.collect();
            let file = file_and_args[0];
            let ir_abi_json_file = file_and_args[1];
            let input_args: Vec<&str> = file_and_args[2..].to_vec();

            let func_name = matches.value_of("FUNC").unwrap();

            let wasm_bytes = {
                let mut f = File::open(file).expect("no input file found");
                let f_meta = fs::metadata(file).expect("unable to read input file metadata");
                let mut file_bytes = vec![0; f_meta.len() as usize];
                f.read_exact(&mut file_bytes).expect("buffer overflow");
                file_bytes
            };
            let ir_abi_json_bytes = {
                let mut f = File::open(ir_abi_json_file).expect("no input abi file found");
                let f_meta =
                    fs::metadata(ir_abi_json_file).expect("unable to read input abi file metadata");
                let mut file_bytes = vec![0; f_meta.len() as usize];
                f.read_exact(&mut file_bytes).expect("buffer overflow");
                file_bytes
            };
            let ir_abi_meta_info = IRContractABIMeta::from_json(&ir_abi_json_bytes);

            let mut mock_runtime = MockRuntime {
                contract_ir_meta: ir_abi_meta_info,
                accounts: HashMap::new(),
                vm: VirtualMachine::new(wasm_bytes.clone(), "me".to_string()),
                store: HashMap::new(),
                events: vec![],
                caller: "root".to_string(),
                module: RefCell::new(None),
                abort_msg: None,
                revert_err_code: 0,
                abort_and_exit: true,
                print_logs: "".to_owned(),
                last_visited_storage_hints: vec![],
                codec: vec![],
                hash: [0; 32],
                call_result: vec![],
                call_args: vec![],
                wasm_start_called: false,
            };

            init_mock_runtime();

            let module = Module::from_buffer(&wasm_bytes).unwrap();
            let module_ref = ModuleInstance::new(
                &module,
                &ImportsBuilder::new().with_resolver("env", &mock_runtime),
            )
            .expect("Failed to instantiate module")
            .run_start(&mut NopExternals)
            .expect("Failed to run start function in module");
            if let Some(ExternVal::Memory(memory_ref)) = module_ref.export_by_name("memory") {
                mock_runtime.vm.memory = memory_ref;
            }

            let abi_method = mock_runtime.contract_ir_meta.get_method(func_name);
            if abi_method.is_none() {
                println!("error: func {func_name} not found");
                std::process::exit(1);
            }
            let abi_method = abi_method.unwrap();

            // inputs should be strings, and encode them by ir_abi_meta_info
            if !input_args.is_empty() {
                let encoded = abi_method.encode_params(&input_args);
                if encoded.is_err() {
                    println!("error: encode params error {}", encoded.err().unwrap());
                    std::process::exit(1);
                }
                mock_runtime.vm.input = encoded.unwrap();
            } else {
                mock_runtime.vm.input = hex::decode("00").unwrap();
            }

            if let Err(err) = module_ref.invoke_export(func_name, &[], &mut mock_runtime) {
                println!("error: {err}");
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("build") {
        if let Some(files) = matches.values_of("INPUT") {
            let files: Vec<&str> = files.collect();
            let source_map = Arc::new(SourceMap::new(FilePathMapping::empty()));
            let mut codes: Vec<String> = vec![];
            for filename in &files {
                let filename = *filename;
                let src = match std::fs::read_to_string(filename) {
                    Ok(src) => src,
                    Err(_) => {
                        println!("Failed to load source file, can't find {filename}");
                        FatalError.raise();
                    }
                };
                codes.push(src.clone());
                source_map.new_source_file(PathBuf::from(filename).into(), src.to_string());
            }

            init_mock_runtime();

            let module = smart_ir::ir::frontend::parser::compile(codes[0].as_str());
            let mut ctx = IRContext::default();
            translate_main_module(&mut ctx, &module);
            let mut p = IRPrinter::new(&ctx);
            let mut w = String::new();
            p.print_main_module(&mut w).unwrap();
            println!("compiled module: {w}");
            let abi_names = RefCell::new(Vec::new());
            let options = IROptions::default();
            let emit_wasm_bytes =
                smart_ir::ir_codegen::ir_emit_code(&ctx, abi_names, &options, WASM_IR.to_vec())
                    .unwrap();
            let wasm_output_file = "a.out.wasm";
            std::fs::write(wasm_output_file, emit_wasm_bytes).unwrap();
            println!("writen file {wasm_output_file}");

            // dump ir_abi.json
            let ctx_main_module = ctx.get_main_module();
            if let Some(ctx_main_module) = ctx_main_module {
                if let Some(main_contract) = &ctx_main_module.contract {
                    // dump contract meta json (IRContractABIMeta)
                    let ir_contract_abi_info = IRContractABIMeta::from_contract(main_contract);
                    println!("ir_contract_abi_info: {ir_contract_abi_info:?}");
                    let ir_contract_abi_json = ir_contract_abi_info.to_json();
                    let ir_abi_json_filepath = "a.out.abi.json";
                    std::fs::write(ir_abi_json_filepath, ir_contract_abi_json).unwrap();
                    println!("writen file {ir_abi_json_filepath}");
                }
            }
        }
    } else {
        println!("{}", matches.usage());
    }
}
