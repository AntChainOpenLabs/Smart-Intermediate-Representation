// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

// #[test]
// fn test_select() {
//     let mut runtime_and_abi = build_mock_runtime(
//         r#"module_name = "Test"
//         type struct.Test.Person = {name: str, age: u8, }
//         contract Test {
//             state {
//             }
//             pub fn Test.Test.init()  {
//                 0:
//                     ret()
//             }
//
//             pub fn Test.Test.test()  {
//                 1:
//                     let %0: %struct.Test.Person* !ir_debug_location !0 = malloc(%struct.Test.Person, )
//                     set_field(%0: %struct.Test.Person*, "alice": str, 0: i32, ) !ir_debug_location !0
//                     set_field(%0: %struct.Test.Person*, 1: u8, 1: i32, ) !ir_debug_location !0
//                     let %1: %struct.Test.Person* !ir_debug_location !0 = %0: %struct.Test.Person*
//                     let %2: str !ir_debug_location !1 = get_field(%1: %struct.Test.Person*, 0: i32, ) -> str
//                     let %3: u8 !ir_debug_location !2 = get_field(%1: %struct.Test.Person*, 1: i32, ) -> u8
//                     ret() !ir_debug_location !2
//             }
//
//         }
//         meta !0 = !{8: u32, 11: u32, "test.sonar": str, }
//         meta !1 = !{12: u32, 12: u32, "test.sonar": str, }
//         meta !2 = !{13: u32, 13: u32, "test.sonar": str, }
//         "#,
//     );
//     let mut runtime = runtime_and_abi.0;
//     let abi = runtime_and_abi.1;
//     // ABI
//     assert_eq!(abi.methods.len(), 2);
//     assert_eq!(abi.methods[1].inputs.len(), 0);
//     // Deploy contract and call contract constructor.
//     runtime.constructor(hex::decode("00").unwrap().as_slice());
//     // Contract method call.
// }
