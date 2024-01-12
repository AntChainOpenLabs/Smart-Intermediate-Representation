// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

// TODO: Failed to parse wasm module: Validation("Function #45 reading/validation error: At instruction Call(50)(@10): Expected value of type Specific(I64) on top of stack. Got Specific(I32)")

// #[test]
// fn parampack_encode_test() {
//     let mut runtime_and_abi = build_mock_runtime(
//         r#"module_name = "EthEntrypoint"
//         contract EthEntrypoint {
//             state {
//                 chain_id: u64,
//             }
//             pub fn EthEntrypoint.EthEntrypoint.init(%0: u64, )  {
//                 0:
//                     storage_store(get_storage_path("chain_id": str, ) !ir_storage_path_extra_args !1 , %0: u64, ) !ir_debug_location !0
//                     ret() !ir_debug_location !0
//             }
//
//             pub fn EthEntrypoint.EthEntrypoint.create_eoa_account(%0: str, )  {
//                 1:
//                     let %1: %ir.builtin.parampack !ir_debug_location !2 = call(@ir.builtin.encode_params(%0: str, storage_load(get_storage_path("chain_id": str, ) !ir_storage_path_extra_args !3 , ) -> u64 , ) -> %ir.builtin.parampack, )
//                     ret() !ir_debug_location !2
//             }
//
//         }
//         meta !0 = !{8: u32, 8: u32, "test.sonar": str, }
//         meta !1 = !{0: u32, }
//         meta !2 = !{12: u32, 12: u32, "test.sonar": str, }
//         meta !3 = !{1: u32, }
//         "#,
//     );
//     let mut runtime = runtime_and_abi.0;
//     let abi = runtime_and_abi.1;
//     // ABI
//     assert_eq!(abi.methods.len(), 2);
//     assert_eq!(abi.methods[1].inputs.len(), 1);
//     // Deploy contract and call contract constructor.
//     runtime.constructor(hex::decode("00").unwrap().as_slice());
//     // Contract method call.
// }
