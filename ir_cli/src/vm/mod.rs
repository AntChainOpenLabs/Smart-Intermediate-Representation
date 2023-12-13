// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

//! This test file is mainly an e2e test for smart contracts.
//! Compile and generate ABI and WASM modules from the source code of
//! the contract, and mock the runtime of a blockchain platform, in
//! which storage and reading, account models, events, assets, etc.
//! can be simulated.
#![allow(dead_code)]

mod context;

use chrono::Local;
#[allow(unused_imports)]
use log::info;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::panic::UnwindSafe;
use std::{cell::RefCell, collections::HashMap};

use bstr::ByteSlice;
use num_traits::FromPrimitive;
use rustc_serialize::hex::ToHex;
use wasmi::*;

use crate::abi::IRContractABIMeta;
use crate::vm::context::MockExtendContext;
use keccak_hash::keccak256;
use smart_ir::integration::hostapi::HostAPI;
use smart_ir::ir::context::IRContext;
use smart_ir::ir::frontend::translate::translate_main_module;
use smart_ir::ir_codegen::common::global::set_extend_context;
use smart_ir::ir_config::IROptions;
use smart_ir::runtime::vm::*;

pub static WASM_IR: [&[u8]; 1] = [include_bytes!(
    "../../../smart_ir/src/runtime/stdlib/wasm/storage_t.bc"
)];

// ----------------------------------------------
// Integration IR mock runtime tests
// ----------------------------------------------

/// MockRuntime is a mock blockchain platform runtime.
pub struct MockRuntime {
    pub contract_ir_meta: IRContractABIMeta,
    pub caller: Address,
    pub accounts: HashMap<Address, (Vec<u8>, u128)>,
    pub store: HashMap<Vec<Address>, Vec<u8>>,
    pub vm: VirtualMachine,
    pub events: Vec<Event>,
    pub module: RefCell<Option<ModuleRef>>,
    pub abort_msg: Option<String>,
    pub revert_err_code: i32,
    pub abort_and_exit: bool,
    pub print_logs: String,
    pub last_visited_storage_hints: Vec<u32>,
    pub codec: Vec<Vec<u8>>,
    pub hash: [u8; 32],
    /// Cross contract call result bytes.
    pub call_result: Vec<u8>,
    /// Cross contract call argument list bytes.
    pub call_args: Vec<Vec<u8>>,
    pub wasm_start_called: bool,
}

pub fn init_mock_runtime() {
    set_extend_context(Box::new(MockExtendContext::new()));
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Event is the contract event and emit elements in the mock runtime.
pub struct Event {
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
struct HostCodeFinish {}

impl HostError for HostCodeFinish {}

impl fmt::Display for HostCodeFinish {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "finish")
    }
}

#[derive(Debug, Clone, PartialEq)]
struct HostCodeRevert {}

impl fmt::Display for HostCodeRevert {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "revert")
    }
}

impl HostError for HostCodeRevert {}

/// Externals trait is used for mock platform host APIs.
impl Externals for MockRuntime {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match FromPrimitive::from_usize(index) {
            Some(HostAPI::WriteObject) => {
                // void
                // write_object(uint32_t comps, uint32_t comps_d1_length,
                //     uint32_t comps_d2_length,
                //     uint32_t hints_data, uint32_t hints_length,
                //     uint32_t value, uint32_t value_length);
                let keys = self.build_keys(&args, "SET STORAGE".to_string())?;

                let comps_len: u32 = args.nth_checked(1)?;
                let hints_len: u32 = args.nth_checked(4)?;
                let src: u32 = args.nth_checked(5)?;
                let len: u32 = args.nth_checked(6)?;

                let mut output = Vec::new();
                output.resize(len as usize, 0);

                self.vm.memory.get_into(src, &mut output).unwrap();
                println!("SET STORAGE KEYS LEN {comps_len:?}, hints LEN {hints_len:?}");
                println!("SET STORAGE KEYS {keys:?}");
                println!("SET STORAGE VALUE U8 VEC {output:?}");
                self.codec.push(output.clone());

                self.store.insert(keys, output);

                Ok(None)
            }
            Some(HostAPI::ReadObject) => {
                // void
                // read_object(uint32_t comps, uint32_t comps_d1_length,
                //             uint32_t comps_d2_length,
                //             uint32_t hints_data, uint32_t hints_length,
                //             uint32_t value);
                let keys = self.build_keys(&args, "GET STORAGE".to_string())?;

                let value_ptr: u32 = args.nth_checked(5)?;

                println!("GET SHARED STORAGE KEYS: {keys:?}");
                let value = match self.store.get(&keys) {
                    Some(v) => v.clone(),
                    None => vec![0],
                };
                println!("GET SHARED STORAGE VALUE {value:?}");
                self.codec.push(value.clone());
                self.vm
                    .memory
                    .set(value_ptr, &value)
                    .expect("copy key from wasm memory");

                Ok(None)
            }
            Some(HostAPI::DeleteObject) => {
                let keys = self.build_keys(&args, "Delete STORAGE".to_string())?;
                println!("Delete SHARED STORAGE KEYS: {keys:?}");
                self.store.remove(&keys);
                Ok(None)
            }
            Some(HostAPI::ReadObjectLength) => {
                let keys = self.build_keys(&args, "GET STORAGE LENGTH".to_string())?;
                println!("GET SHARED STORAGE KEYS: {keys:?}");
                let length = match self.store.get(&keys) {
                    Some(v) => v.len() as i32,
                    None => -1,
                };
                println!("GET PRIVATE STORAGE LENGTH: {length:?}");
                Ok(Some(RuntimeValue::I32(length)))
            }
            Some(HostAPI::GetCallSender) => {
                let dest = args.nth_checked::<u32>(0)?;
                self.vm.memory.set(dest, self.caller.as_bytes()).unwrap();

                Ok(None)
            }
            Some(HostAPI::GetCallSenderLength) => {
                Ok(Some(RuntimeValue::I32(self.caller.len() as i32)))
            }
            Some(HostAPI::GetCallContract) => {
                let dest = args.nth_checked::<u32>(0)?;
                self.vm.memory.set(dest, self.vm.addr.as_bytes()).unwrap();

                Ok(None)
            }
            Some(HostAPI::GetCallGasLeft) => Ok(Some(RuntimeValue::I64(
                self.vm.context.call_gas_left as i64,
            ))),
            Some(HostAPI::GetCallGasLimit) => Ok(Some(RuntimeValue::I64(
                self.vm.context.call_gas_limit as i64,
            ))),
            Some(HostAPI::GetOpContractLength) => {
                Ok(Some(RuntimeValue::I32(self.vm.op_addr.len() as i32)))
            }
            Some(HostAPI::GetOpContract) => {
                let dest = args.nth_checked::<u32>(0)?;
                self.vm
                    .memory
                    .set(dest, self.vm.op_addr.as_bytes())
                    .unwrap();

                Ok(None)
            }
            Some(HostAPI::GetCallContractLength) => {
                Ok(Some(RuntimeValue::I32(self.vm.addr.len() as i32)))
            }
            Some(HostAPI::GetCallArgPack) => {
                let dest = args.nth_checked::<u32>(0)?;
                self.vm.memory.set(dest, &self.vm.input).unwrap();

                Ok(None)
            }
            Some(HostAPI::GetCallArgPackLength) => {
                Ok(Some(RuntimeValue::I32(self.vm.input.len() as i32)))
            }
            Some(HostAPI::SetCallResult) => {
                let src: u32 = args.nth_checked(0)?;
                let len: u32 = args.nth_checked(1)?;

                let mut output = Vec::new();
                output.resize(len as usize, 0);

                self.vm.memory.get_into(src, &mut output).unwrap();

                let try_utf8 = String::from_utf8(output.clone());

                if try_utf8.is_ok() {
                    println!("result: {}", try_utf8.unwrap());
                } else {
                    println!("result: {}", hex::encode(output.clone()));
                }

                self.vm.output = output;

                Ok(None)
            }
            Some(HostAPI::GetBlockNumber) => Ok(Some(RuntimeValue::I64(
                self.vm.context.block_number.try_into().unwrap(),
            ))),
            Some(HostAPI::GetBlockTimestamp) => Ok(Some(RuntimeValue::I64(
                self.vm.context.block_timestamp.try_into().unwrap(),
            ))),
            Some(HostAPI::GetBlockRandomSeed) => {
                let data: u32 = args.nth_checked(0)?;

                self.vm
                    .memory
                    .set(data, &self.vm.context.block_random_seed)
                    .unwrap();

                Ok(None)
            }
            Some(HostAPI::GetTxTimestamp) => Ok(Some(RuntimeValue::I64(
                self.vm.context.timestamp.try_into().unwrap(),
            ))),
            Some(HostAPI::GetTxNonce) => Ok(Some(RuntimeValue::I64(
                self.vm.context.nonce.try_into().unwrap(),
            ))),
            Some(HostAPI::GetTxIndex) => Ok(Some(RuntimeValue::I32(
                self.vm.context.index.try_into().unwrap(),
            ))),
            Some(HostAPI::GetTxHash) => {
                let dest = args.nth_checked::<u32>(0)?;
                self.vm
                    .memory
                    .set(dest, self.vm.context.hash.as_bytes())
                    .unwrap();

                Ok(None)
            }
            Some(HostAPI::GetTxHashLength) => {
                Ok(Some(RuntimeValue::I32(self.vm.context.hash.len() as i32)))
            }
            Some(HostAPI::GetTxSender) => {
                let dest = args.nth_checked::<u32>(0)?;
                self.vm.memory.set(dest, self.caller.as_bytes()).unwrap();
                Ok(None)
            }
            Some(HostAPI::GetTxSenderLength) => {
                Ok(Some(RuntimeValue::I32(self.caller.len() as i32)))
            }
            Some(HostAPI::GetTxGasLimit) => {
                Ok(Some(RuntimeValue::I64(self.vm.context.tx_gas_limit as i64)))
            }
            Some(HostAPI::Abort) => {
                let src: u32 = args.nth_checked(0)?;
                let len: u32 = args.nth_checked(1)?;

                let mut output = Vec::new();
                output.resize(len as usize, 0);
                self.vm.memory.get_into(src, &mut output).unwrap();
                let msg = String::from_utf8(output).unwrap();
                println!("CALL ABORT MESSAGE: {msg}");
                self.abort_msg = Some(msg);
                if self.abort_and_exit {
                    return Err(wasmi::Trap::new(TrapKind::Unreachable));
                }
                Ok(None)
            }
            Some(HostAPI::Revert) => {
                let err_code: u32 = args.nth_checked(0)?;
                let src: u32 = args.nth_checked(1)?;
                let len: u32 = args.nth_checked(2)?;

                let mut output = Vec::new();
                output.resize(len as usize, 0);
                self.vm.memory.get_into(src, &mut output).unwrap();
                let msg = String::from_utf8(output).unwrap();
                println!("CALL REVERT ERROR_CODE {err_code} MESSAGE: {msg}");
                self.abort_msg = Some(msg);
                self.revert_err_code = err_code as i32;
                if self.abort_and_exit {
                    return Err(wasmi::Trap::new(TrapKind::Unreachable));
                }
                Ok(None)
            }
            Some(HostAPI::Println) => {
                let src: u32 = args.nth_checked(0)?;
                let len: u32 = args.nth_checked(1)?;

                let mut output = Vec::new();
                output.resize(len as usize, 0);
                self.vm.memory.get_into(src, &mut output).unwrap();

                let output_result = String::from_utf8(output);
                let output_str = if let Ok(output_result) = output_result {
                    output_result
                } else {
                    println!("println src={src}, len={len}");
                    "invalid println output".to_string()
                };
                println!("CALL PRINTLN: {output_str}");

                let mut tmp: String = self.print_logs.clone();
                tmp += output_str.as_str();
                tmp += "\n";
                self.print_logs = tmp;
                Ok(None)
            }
            Some(HostAPI::Log) => {
                let topics: u32 = args.nth_checked(0)?;
                let topics_len: u32 = args.nth_checked(1)?;
                let topic_len_array: u32 = args.nth_checked(2)?;
                let desc: u32 = args.nth_checked(3)?;
                let desc_len: u32 = args.nth_checked(4)?;

                let mut topic_len_array_o = Vec::new();
                topic_len_array_o.resize(topics_len as usize, 0);
                for i in 0..topics_len {
                    let mut len_bytes: [u8; 4] = [0; 4];
                    self.vm
                        .memory
                        .get_into(topic_len_array + i * 4, &mut len_bytes)
                        .unwrap();
                    // cast [u8; 4] to u32
                    let ptr: *const u8 = len_bytes.as_ptr();
                    let ptr: *const u32 = ptr as *const u32;
                    let len = unsafe { *ptr };
                    topic_len_array_o[i as usize] = len;
                }

                let mut topics_o = Vec::new();
                topics_o.resize(topics_len as usize, Vec::new());

                for i in 0..topics_len {
                    let mut addr_bytes: [u8; 4] = [0; 4];
                    self.vm
                        .memory
                        .get_into(topics + i * 4, &mut addr_bytes)
                        .unwrap();
                    // cast [u8; 4] to u32
                    let ptr: *const u8 = addr_bytes.as_ptr();
                    let ptr: *const u32 = ptr as *const u32;
                    let addr = unsafe { *ptr };
                    let mut topic = Vec::new();
                    topic.resize(topic_len_array_o[i as usize] as usize, 0);
                    self.vm.memory.get_into(addr, &mut topic).unwrap();
                    topics_o[i as usize] = topic;
                }

                let mut desc_o = Vec::new();
                desc_o.resize(desc_len as usize, 0);
                self.vm.memory.get_into(desc, &mut desc_o).unwrap();

                let event = Event {
                    topics: topics_o,
                    data: desc_o.clone(),
                };
                self.events.push(event.clone());

                // If the log name is MyCoverage, export the log content.
                let my_coverage_event_name = "MyCoverage";
                let first_topic = event.topics[0].clone();

                // The first byte is the length of the event string.
                if String::from_utf8(first_topic[1..].to_vec()).unwrap() == *my_coverage_event_name
                {
                    let file_name = "out.mygcna";
                    let mut file =
                        File::create(file_name).expect("mygcna coverage file create failed");
                    // `desc_o` is the length encoded by the data stream, and the header needs to read the length of leb128.
                    let coverage_file_bytes = match nano_leb128::SLEB128::read_from(&desc_o) {
                        Ok((_, len_bytes_len)) => &desc_o[len_bytes_len..],
                        Err(_) => unreachable!(
                            "invalid {} event log data(invalid leb128 length)",
                            my_coverage_event_name
                        ),
                    };
                    file.write_all(coverage_file_bytes)
                        .expect("write coverage file failed");
                    println!("MyCoverage dumped to {file_name:?}");
                }

                Ok(None)
            }
            Some(HostAPI::SHA256) => {
                let msg = args.nth_checked::<u32>(0)?;
                let msg_len = args.nth_checked::<u32>(1)?;
                let value = args.nth_checked::<u32>(2)?;

                let mut output = Vec::new();
                output.resize(msg_len as usize, 0);
                self.vm.memory.get_into(msg, &mut output).unwrap();

                let message = unsafe { output.to_str_unchecked() };
                self.hash = sha256(message);

                self.vm.memory.set(value, &self.hash).unwrap();

                Ok(None)
            }
            Some(HostAPI::SM3) => {
                let msg = args.nth_checked::<u32>(0)?;
                let msg_len = args.nth_checked::<u32>(1)?;
                let value = args.nth_checked::<u32>(2)?;

                let mut output = Vec::new();
                output.resize(msg_len as usize, 0);
                self.vm.memory.get_into(msg, &mut output).unwrap();

                let message = unsafe { output.to_str_unchecked() };
                // use sha256 just in mock runtime
                self.hash = sha256(message);

                self.vm.memory.set(value, &self.hash).unwrap();

                Ok(None)
            }
            Some(HostAPI::KECCAK256) => {
                let msg = args.nth_checked::<u32>(0)?;
                let msg_len = args.nth_checked::<u32>(1)?;
                let value = args.nth_checked::<u32>(2)?;

                let mut output = Vec::new();
                output.resize(msg_len as usize, 0);
                self.vm.memory.get_into(msg, &mut output).unwrap();

                let mut message = output.clone();
                if message.len() < 32 {
                    message.resize(32, 0);
                }
                keccak256(&mut message);
                self.hash = message[0..32].try_into().unwrap();

                self.vm.memory.set(value, &self.hash).unwrap();

                Ok(None)
            }
            Some(HostAPI::EthSecp256k1Recovery) => {
                let hash_offset = args.nth_checked::<u32>(0)?;
                let v_offset = args.nth_checked::<u32>(1)?;
                let r_offset = args.nth_checked::<u32>(2)?;
                let s_offset = args.nth_checked::<u32>(3)?;
                let result_offset = args.nth_checked::<u32>(4)?;

                let mut hash = Vec::new();
                hash.resize(32, 0);
                self.vm.memory.get_into(hash_offset, &mut hash).unwrap();

                let mut v = Vec::new();
                v.resize(32, 0);
                self.vm.memory.get_into(v_offset, &mut v).unwrap();

                let mut r = Vec::new();
                r.resize(32, 0);
                self.vm.memory.get_into(r_offset, &mut r).unwrap();

                let mut s = Vec::new();
                s.resize(32, 0);
                self.vm.memory.get_into(s_offset, &mut s).unwrap();

                println!(
                    "[EthSecp256k1Recovery] r={}, s={}, v={}",
                    r.to_hex(),
                    s.to_hex(),
                    v.to_hex()
                );

                let mut sig_inputs = Vec::new();
                sig_inputs.append(&mut r);
                sig_inputs.append(&mut s);
                let sig = libsecp256k1::Signature::parse_standard(
                    sig_inputs.as_slice().try_into().unwrap(),
                )
                .unwrap();
                let int_v = v[31];
                if int_v > 28 {
                    // too big recovery id for mock implementation
                    self.hash = [0; 32];
                    println!("ETH EC RECOVERY for invalid v {int_v}");

                    self.vm.memory.set(result_offset, &self.hash).unwrap();

                    return Ok(Some(RuntimeValue::I32(1)));
                }
                let recover_msg = libsecp256k1::Message::parse(hash.as_slice().try_into().unwrap());
                let rec_id = int_v - 27;
                let recover_result = libsecp256k1::recover(
                    &recover_msg,
                    &sig,
                    &libsecp256k1::RecoveryId::parse(rec_id).unwrap(),
                )
                .unwrap();
                let pubkey_bytes = recover_result.serialize();
                let pubkey_bytes = &pubkey_bytes[1..pubkey_bytes.len()]; // to 64bytes
                let mut message = pubkey_bytes.to_vec();
                keccak256(&mut message);
                self.hash = message[0..32].try_into().unwrap();

                println!("ETH EC RECOVERY: {}", pubkey_bytes.to_hex());
                println!("ETH EC RECOVERY ADDR(32bytes): {}", self.hash.to_hex());
                println!("ETH EC RECOVERY ADDR: {}", self.hash[12..32].to_hex());

                self.vm.memory.set(result_offset, &self.hash).unwrap();

                Ok(Some(RuntimeValue::I32(1)))
            }

            Some(HostAPI::CoCall) => {
                let arg4 = args.nth_checked::<u32>(4)?;
                let arg5 = args.nth_checked::<u32>(5)?;
                let mut output = Vec::new();
                output.resize(arg5 as usize, 0);
                self.vm.memory.get_into(arg4, &mut output).unwrap();
                self.call_args.push(output.clone());
                Ok(Some(RuntimeValue::I32(0)))
            }
            Some(HostAPI::GetCallResult) => {
                let dest = args.nth_checked::<u32>(0)?;
                self.vm
                    .memory
                    .set(dest, self.call_result.as_bytes())
                    .unwrap();

                Ok(None)
            }
            Some(HostAPI::GetCallResultLength) => {
                Ok(Some(RuntimeValue::I32(self.call_result.len() as i32)))
            }
            _ => panic!("unknown external function {index} "),
        }
    }
}

impl ModuleImportResolver for MockRuntime {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        let index = match field_name {
            "write_object" => HostAPI::WriteObject,
            "read_object" => HostAPI::ReadObject,
            "delete_object" => HostAPI::DeleteObject,
            "read_object_length" => HostAPI::ReadObjectLength,
            "get_call_sender" => HostAPI::GetCallSender,
            "get_call_sender_length" => HostAPI::GetCallSenderLength,
            "get_call_contract" => HostAPI::GetCallContract,
            "get_call_contract_length" => HostAPI::GetCallContractLength,
            "get_call_gas_left" => HostAPI::GetCallGasLeft,
            "get_call_gas_limit" => HostAPI::GetCallGasLimit,
            "get_op_contract" => HostAPI::GetOpContract,
            "get_op_contract_length" => HostAPI::GetOpContractLength,
            "get_call_argpack" => HostAPI::GetCallArgPack,
            "get_call_argpack_length" => HostAPI::GetCallArgPackLength,
            "set_call_result" => HostAPI::SetCallResult,
            "get_block_number" => HostAPI::GetBlockNumber,
            "get_block_timestamp" => HostAPI::GetBlockTimestamp,
            "get_block_random_seed" => HostAPI::GetBlockRandomSeed,
            "get_tx_timestamp" => HostAPI::GetTxTimestamp,
            "get_tx_nonce" => HostAPI::GetTxNonce,
            "get_tx_index" => HostAPI::GetTxIndex,
            "get_tx_hash" => HostAPI::GetTxHash,
            "get_tx_hash_length" => HostAPI::GetTxHashLength,
            "get_tx_sender" => HostAPI::GetTxSender,
            "get_tx_sender_length" => HostAPI::GetTxSenderLength,
            "get_tx_gas_limit" => HostAPI::GetTxGasLimit,
            "abort" => HostAPI::Abort,
            "println" => HostAPI::Println,
            "log" => HostAPI::Log,
            "sha256" => HostAPI::SHA256,
            "sm3" => HostAPI::SM3,
            "keccak256" => HostAPI::KECCAK256,
            "eth_secp256k1_recovery" => HostAPI::EthSecp256k1Recovery,
            "co_call" => HostAPI::CoCall,
            "revert" => HostAPI::Revert,
            "get_call_result_length" => HostAPI::GetCallResultLength,
            "get_call_result" => HostAPI::GetCallResult,
            _ => {
                panic!("{field_name} not implemented");
            }
        };

        Ok(FuncInstance::alloc_host(signature.clone(), index as usize))
    }

    fn resolve_memory(
        &self,
        _field_name: &str,
        _memory_type: &MemoryDescriptor,
    ) -> Result<MemoryRef, Error> {
        Ok(self.vm.memory.clone())
    }
}

impl UnwindSafe for MockRuntime {}

impl MockRuntime {
    /// Input WASM binary code and create a WASM module.
    fn create_or_get_module(&self, code: &[u8]) -> ModuleRef {
        if let Some(module) = self.module.borrow().as_ref() {
            return module.clone();
        }

        let module = Module::from_buffer(code).expect("Failed to parse wasm module");

        let module_ref =
            ModuleInstance::new(&module, &ImportsBuilder::new().with_resolver("env", self))
                .expect("Failed to instantiate module")
                .run_start(&mut NopExternals)
                .expect("Failed to run start function in module");
        {
            let mut self_module = self.module.borrow_mut();
            *self_module = Some(module_ref.clone());
        }
        module_ref
    }

    fn _call(&mut self, name: &str, encoded_params: &[u8]) -> Vec<u8> {
        let module = self.create_or_get_module(&self.vm.code);

        if !self.wasm_start_called {
            // Invoke main and init runtime mainly including the memory management initialization.
            if module.export_by_name("_start").is_some() {
                match module.invoke_export("_start", &[], self) {
                    Err(wasmi::Error::Trap(trap)) => match trap.kind() {
                        TrapKind::Host(host_error) => {
                            assert!(
                                host_error.downcast_ref::<HostCodeFinish>().is_some(),
                                "fail to invoke main: {host_error}"
                            );
                        }
                        _ => panic!("fail to invoke main: {trap}"),
                    },
                    Ok(Some(RuntimeValue::I32(0))) | Ok(None) => {}
                    Ok(Some(RuntimeValue::I32(ret))) => panic!("main returns: {ret}"),
                    Err(e) => panic!("fail to invoke main: {e}"),
                    _ => panic!("fail to invoke main, unknown"),
                }
            }
            if let Some(ExternVal::Memory(memory_ref)) = module.export_by_name("memory") {
                self.vm.memory = memory_ref;
            }
            self.wasm_start_called = true;
        }

        self.vm.input = encoded_params.to_vec();

        match module.invoke_export(name, &[], self) {
            Err(wasmi::Error::Trap(trap)) => match trap.kind() {
                TrapKind::Host(host_error) => {
                    assert!(
                        host_error.downcast_ref::<HostCodeFinish>().is_some(),
                        "fail to invoke {name}: {host_error}"
                    );
                }
                _ => panic!("fail to invoke {name}: {trap}"),
            },
            Ok(Some(RuntimeValue::I32(0))) | Ok(None) => {}
            Ok(Some(RuntimeValue::I32(ret))) => panic!("{name} returns: {ret}"),
            Err(e) => panic!("fail to invoke {name}: {e}"),
            _ => panic!("fail to invoke {name}, unknown err"),
        }

        self.vm.output.clone()
    }
    /// Call contract methods with the method name and parameters
    pub fn call(&mut self, name: &str, encoded_params: &[u8]) -> Vec<u8> {
        self._call(name, encoded_params)
    }

    /// Call contract methods with the method name and parameters
    pub fn call_return_result(&mut self, name: &str, params: &[u8]) -> Result<Vec<u8>, String> {
        self.abort_msg = None;

        let calldata = params.to_vec();

        let module = self.create_or_get_module(&self.vm.code);

        // Invoke main and init runtime mainly including the memory management initialization.
        if module.export_by_name("_start").is_some() {
            match module.invoke_export("_start", &[], self) {
                Err(wasmi::Error::Trap(trap)) => match trap.kind() {
                    TrapKind::Host(host_error) => {
                        if host_error.downcast_ref::<HostCodeFinish>().is_none() {
                            return Err(format!("fail to invoke main: {host_error}"));
                        }
                    }
                    _ => return Err(format!("fail to invoke main: {trap}")),
                },
                Ok(Some(RuntimeValue::I32(0))) | Ok(None) => {}
                Ok(Some(RuntimeValue::I32(ret))) => return Err(format!("main returns: {ret}")),
                Err(e) => return Err(format!("fail to invoke main: {e}")),
                _ => return Err("fail to invoke main, unknown".to_string()),
            }
        }

        self.vm.input = calldata;

        if let Some(ExternVal::Memory(memory_ref)) = module.export_by_name("memory") {
            self.vm.memory = memory_ref;
        }

        match module.invoke_export(name, &[], self) {
            Err(wasmi::Error::Trap(trap)) => match trap.kind() {
                TrapKind::Host(host_error) => {
                    if host_error.downcast_ref::<HostCodeFinish>().is_none() {
                        return Err(format!("fail to invoke {name}: {host_error}"));
                    }
                }
                _ => return Err(format!("fail to invoke {name}: {trap}")),
            },
            Ok(Some(RuntimeValue::I32(0))) | Ok(None) => {}
            Ok(Some(RuntimeValue::I32(ret))) => return Err(format!("{name} returns: {ret}")),
            Err(e) => return Err(format!("fail to invoke {name}: {e}")),
            _ => return Err(format!("fail to invoke {name}, unknown err")),
        }

        let output = self.vm.output.clone();
        Ok(output)
    }

    #[inline]
    pub fn constructor(&mut self, args: &[u8]) {
        self.call("init", args);
    }

    fn events(&self) {
        unimplemented!("events mock test impl")
    }
}

impl MockRuntime {
    fn build_keys(&mut self, args: &RuntimeArgs, fun_name: String) -> Result<Vec<String>, Trap> {
        let key_count: u32 = args.nth_checked(1)?;
        let hints_data_ptr: u32 = args.nth_checked(3)?;
        let hints_count: u32 = args.nth_checked(4)?;

        let mut hints_data_u8_vec = Vec::new();

        hints_data_u8_vec.resize(hints_count as usize * 4, 0);
        self.vm
            .memory
            .get_into(hints_data_ptr, &mut hints_data_u8_vec)
            .unwrap();

        let mut hints: Vec<u32> = vec![];
        for i in 0..hints_count {
            let i = i as usize;
            let hint: u32 = hints_data_u8_vec[i * 4] as u32
                + ((hints_data_u8_vec[i * 4 + 1] as u32) << 8)
                + ((hints_data_u8_vec[i * 4 + 2] as u32) << 16)
                + ((hints_data_u8_vec[i * 4 + 3] as u32) << 24);
            hints.push(hint);
        }

        println!(
            "{fun_name:} KEY COUNT {key_count:?}, hints count {hints_count:?}, hints: {hints:?}"
        );

        self.last_visited_storage_hints = hints.clone();

        let key_datas_ptr: u32 = args.nth_checked(0)?;

        let mut key_ptr_u8_vec = Vec::new();
        key_ptr_u8_vec.resize(key_count as usize * 4, 0);

        self.vm
            .memory
            .get_into(key_datas_ptr, &mut key_ptr_u8_vec)
            .unwrap();

        let key_lengths_ptr: u32 = args.nth_checked(2)?;

        let mut key_ptrs = vec![];

        for i in 0..key_count {
            let i = i as usize;
            let key_ptr: u32 = key_ptr_u8_vec[i * 4] as u32
                + ((key_ptr_u8_vec[i * 4 + 1] as u32) << 8)
                + ((key_ptr_u8_vec[i * 4 + 2] as u32) << 16)
                + ((key_ptr_u8_vec[i * 4 + 3] as u32) << 24);
            key_ptrs.push(key_ptr);
        }

        println!("{fun_name:} KEY POINTS {key_ptrs:?}");

        let mut key_lengths_u8_vec = Vec::new();

        key_lengths_u8_vec.resize(key_count as usize * 4, 0);
        self.vm
            .memory
            .get_into(key_lengths_ptr, &mut key_lengths_u8_vec)
            .unwrap();

        println!("{fun_name:} KEY LENGTHS U8 VEC {key_lengths_u8_vec:?}");
        let mut key_lengths = vec![];
        for i in 0..key_count {
            let i = i as usize;
            let key_length: u32 = key_lengths_u8_vec[i * 4] as u32
                + ((key_lengths_u8_vec[i * 4 + 1] as u32) << 8)
                + ((key_lengths_u8_vec[i * 4 + 2] as u32) << 16)
                + ((key_lengths_u8_vec[i * 4 + 3] as u32) << 24);
            key_lengths.push(key_length);
        }

        println!("{fun_name:} KEY LENGTHS {key_lengths:?}");

        let mut keys = vec![];
        for i in 0..(key_count) {
            let src = key_ptrs[i as usize];
            let len = key_lengths[i as usize];
            let mut output = Vec::new();
            output.resize(len as usize, 0);
            self.vm.memory.get_into(src, &mut output).unwrap();
            if let Ok(key) = String::from_utf8(output.clone()) {
                keys.push(key);
            } else {
                keys.push(output.to_hex().to_string());
            }
        }
        println!("{fun_name:} KEYS {keys:?}");

        for hint in hints {
            if hint > key_count {
                println!("hint {hint:?} > keys count {key_count:?}");
                return Err(Trap::new(TrapKind::Unreachable));
            }
        }

        Ok(keys)
    }
}

fn mock_compile(src: &str) -> IRContext {
    let module = smart_ir::ir::frontend::parser::compile(src);
    let mut ctx = IRContext::default();
    translate_main_module(&mut ctx, &module);
    ctx
}
pub fn build_mock_runtime(src: &str) -> (MockRuntime, IRContractABIMeta) {
    info!("build_mock_runtime at {:?}", Local::now());
    init_mock_runtime();
    let ir_context = mock_compile(src);
    let abi_names = RefCell::new(Vec::new());
    let options = IROptions::default();
    let emit_wasm_bytes =
        smart_ir::ir_codegen::ir_emit_code(&ir_context, abi_names, &options, WASM_IR.to_vec())
            .unwrap();

    let ctx_main_module = ir_context.get_main_module();
    let mut ir_contract_abi_info = IRContractABIMeta::default();
    if let Some(ctx_main_module) = ctx_main_module {
        if let Some(main_contract) = &ctx_main_module.contract {
            // dump contract meta json (IRContractABIMeta)
            ir_contract_abi_info = IRContractABIMeta::from_contract(main_contract);
        }
    }

    let mock_runtime = MockRuntime {
        contract_ir_meta: ir_contract_abi_info.clone(),
        accounts: HashMap::new(),
        vm: VirtualMachine::new(emit_wasm_bytes, Address::from("me")),
        store: HashMap::new(),
        events: vec![],
        caller: rand_address(),
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

    (mock_runtime, ir_contract_abi_info)
}
