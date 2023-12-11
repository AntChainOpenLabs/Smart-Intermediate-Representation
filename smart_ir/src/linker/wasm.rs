// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use parity_wasm::builder;
use parity_wasm::elements::{ExportEntry, InitExpr, Instruction, Module};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::ffi::CString;
use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

use crate::integration::hostapi::HostAPI;
use crate::ir_codegen::common::global::{get_extend_context, has_extend_context};
use crate::ir_config::{IROptions, OptimizationLevel};
use crate::linker::util::get_clang_rt_lib_dir;

fn generate_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn do_start_optimize(input: &[u8]) -> Option<Vec<u8>> {
    let home_dir = home::home_dir().unwrap();
    let tool_path = home_dir.join("tools").join("start_optimizer");
    if !tool_path.exists() {
        return None;
    }
    // write input to tmp file
    let tmp_dir = tempdir();
    if tmp_dir.is_err() {
        return None;
    }
    let tmp_dir = &tmp_dir.expect("");
    let rand_input_filename = format!("wasm_input_{}.wasm", generate_string(10));
    let input_path = tmp_dir.path().join(rand_input_filename);
    fs::write(input_path.clone(), input).unwrap();

    let output_filenaem = format!("wasm_output_{}.wasm", generate_string(10));
    let output_path = tmp_dir.path().join(output_filenaem);

    // call start_optimizer
    println!("running start_optimizer to optimize file");
    let optimize_result = Command::new(tool_path)
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .arg(input_path.to_str().unwrap())
        .spawn()
        .and_then(|mut c| c.wait());
    // delete tmp input file
    fs::remove_file(input_path).unwrap();
    match optimize_result {
        Ok(child) => {
            if child.success() {
                let output_wasm_bytes = fs::read(output_path.clone());
                // remove output tmp file
                fs::remove_file(output_path).unwrap();
                if let Ok(output_wasm_bytes) = output_wasm_bytes {
                    Some(output_wasm_bytes)
                } else {
                    None
                }
            } else {
                None
            }
        }
        Err(_err) => None,
    }
}

pub fn link(
    input: &[u8],
    name: &str,
    export_names: &[String],
    opt_level: &OptimizationLevel,
    _no_contract: bool,
    options: &IROptions,
) -> Vec<u8> {
    let dir = tempdir().expect("failed to create temp directory for linking");

    let object_filename = dir.path().join(format!("{name}.o"));
    let res_filename = dir.path().join(format!("{name}.wasm"));

    let mut objectfile =
        File::create(object_filename.clone()).expect("failed to create object file");

    objectfile
        .write_all(input)
        .expect("failed to write object file to temp file");

    let clang_rt_lib_dir =
        std::env::var("CHAIN_IR_CLANG_RT_LIB_DIR").unwrap_or_else(|_| get_clang_rt_lib_dir());

    let mut command_line = vec![
        CString::new(format!("-{}", opt_level.level_string())).unwrap(),
        CString::new("--allow-undefined").unwrap(),
        CString::new("--gc-sections").unwrap(),
        CString::new("--global-base=0").unwrap(),
        CString::new("--stack-first").unwrap(),
        // Link compiler-rt for wasm32 target.
        CString::new("-lclang_rt.builtins-wasm32").unwrap(),
        CString::new(format!("-L/{clang_rt_lib_dir}")).unwrap(),
    ];
    if !options.create_empty_start {
        command_line.push(CString::new("--no-entry").unwrap());
    }

    if !options.no_contract {
        // Force undefined host api symbol during linking.
        for name in HostAPI::all_names() {
            command_line.push(CString::new(format!("--undefined={name}")).unwrap());
        }

        if has_extend_context() {
            for name in get_extend_context().all_extend_host_api_names() {
                command_line.push(CString::new(format!("--undefined={name}")).unwrap());
            }
        }
    }
    if export_names.is_empty() {
        command_line.push(CString::new("--export-all").unwrap());
    } else {
        command_line.push(CString::new("--export").unwrap());
        command_line.push(CString::new("__wasm_call_ctors").unwrap());
        for name in export_names {
            command_line.push(CString::new("--export").unwrap());
            command_line.push(CString::new(name.as_str()).unwrap());
        }
        if options.no_contract {
            // export my abi internal func for test in no-contract
            for name in export_names {
                command_line.push(CString::new("--export").unwrap());
                let mut internal_abi_name = "$ir_contract_internal_".to_owned();
                internal_abi_name.push_str(name.as_str());
                command_line.push(CString::new(internal_abi_name).unwrap());
            }
        }
    }
    command_line.push(
        CString::new(
            object_filename
                .to_str()
                .expect("temp path should be unicode"),
        )
        .unwrap(),
    );
    if options.no_contract {
        // need hostapi_mock.wasm in bin workspace path
        command_line.push(CString::new("hostapi_mock.wasm").unwrap());
    }
    command_line.push(CString::new("-o").unwrap());
    command_line
        .push(CString::new(res_filename.to_str().expect("temp path should be unicode")).unwrap());

    assert!(!super::wasm_linker(&command_line), "linker failed");

    let mut output = Vec::new();
    // read the whole file
    let mut outputfile = File::open(res_filename).expect("output file should exist");

    outputfile
        .read_to_end(&mut output)
        .expect("failed to read output file");

    let mut module: Module =
        parity_wasm::deserialize_buffer(&output).expect("cannot deserialize llvm wasm");

    // use _start function as start section
    let start_export_name = "_inner_start";
    let start_func_idx = if let Some(sec) = module.export_section() {
        let export_entries: Vec<&ExportEntry> = sec
            .entries()
            .iter()
            .filter(|&x| x.field() == start_export_name)
            .collect();
        if export_entries.is_empty() {
            None
        } else if let parity_wasm::elements::Internal::Function(internal_func) =
            export_entries[0].internal()
        {
            Some(*internal_func)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(start_func_idx) = start_func_idx {
        module.set_start_section(start_func_idx);
        // remove the start export section
        if let Some(sec) = module.export_section_mut() {
            let new_entries: Vec<ExportEntry> = sec
                .entries()
                .iter()
                .filter(|&x| x.field() != start_export_name)
                .map(|x| x.to_owned())
                .collect();
            *sec.entries_mut() = new_entries;
        }
    }

    // remove empty initializers
    if let Some(data_section) = module.data_section_mut() {
        let _entries = data_section.entries_mut();
        // TODO: can't rm the entries directly when mulitple modules
        // let mut index = 0;
        // while index < entries.len() {
        //     if entries[index].value().iter().all(|b| *b == 0) {
        //         entries.remove(index);
        //     } else {
        //         index += 1;
        //     }
        // }
    }

    // set stack pointer to 64k (there is only one global)
    for global in module.global_section_mut().unwrap().entries_mut() {
        let init_expr = global.init_expr_mut();
        *init_expr = InitExpr::new(vec![Instruction::I32Const(0x10000), Instruction::End]);
    }

    let linked = builder::module().with_module(module);

    let linked_wasm_bytes =
        parity_wasm::serialize(linked.build()).expect("cannot serialize linked wasm");
    // use start_optimizer to optimize the wasm

    if let Some(out_bytes) = do_start_optimize(&linked_wasm_bytes) {
        out_bytes
    } else {
        linked_wasm_bytes
    }
}
