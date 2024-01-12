// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_json_encode_base_type_test() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "JSONTest"
        contract JSONTest {
            state {
            }
            pub fn JSONTest.JSONTest.init()  {
                0:
                    ret()
            }

            pub fn JSONTest.JSONTest.json_encode_bool() -> str {
                1:
                    let %0: bool !ir_debug_location !0 = true: bool
                    ret(call(@ir.json.encode(%0: bool, ) -> str, ) , ) !ir_debug_location !1
            }

            pub fn JSONTest.JSONTest.json_encode_u8() -> str {
                2:
                    let %0: u8 !ir_debug_location !2 = 1: u8
                    ret(call(@ir.json.encode(%0: u8, ) -> str, ) , ) !ir_debug_location !3
            }

        }
        meta !0 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !1 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !2 = !{9: u32, 9: u32, "test.sonar": str, }
        meta !3 = !{10: u32, 10: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 3);
    assert_eq!(abi.methods[1].inputs.len(), 0);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

// #[test]
// fn test_json_encode_test() {
//     let mut runtime_and_abi = build_mock_runtime(
//         r#" module_name = "JSONTest"
//         type struct.JSONTest.School = {name: str, }
//         type struct.JSONTest.Person = {school: %struct.JSONTest.School*, arr: [%struct.JSONTest.School*], map: {str: %struct.JSONTest.School*}, }
//         contract JSONTest {
//             state {
//             }
//             pub fn JSONTest.JSONTest.init()  {
//                 0:
//                     ret()
//             }
//
//             pub fn JSONTest.JSONTest.json_encode_struct() -> str {
//                 1:
//                     let %0: %struct.JSONTest.School* !ir_debug_location !0 = malloc(%struct.JSONTest.School, )
//                     set_field(%0: %struct.JSONTest.School*, "aaa": str, 0: i32, ) !ir_debug_location !0
//                     let %1: %struct.JSONTest.School* !ir_debug_location !0 = %0: %struct.JSONTest.School*
//                     let %2: %struct.JSONTest.Person* !ir_debug_location !1 = malloc(%struct.JSONTest.Person, )
//                     set_field(%2: %struct.JSONTest.Person*, %1: %struct.JSONTest.School*, 0: i32, ) !ir_debug_location !1
//                     let %3: [%struct.JSONTest.School*; 1] !ir_debug_location !1 = malloc([%struct.JSONTest.School*; 1], )
//                     call(@ir.vector.push(%3: [%struct.JSONTest.School*; 1], %1: %struct.JSONTest.School*, ) -> void, ) !ir_debug_location !1
//                     set_field(%2: %struct.JSONTest.Person*, %3: [%struct.JSONTest.School*; 1], 1: i32, ) !ir_debug_location !1
//                     let %4: {str: %struct.JSONTest.School*} !ir_debug_location !1 = malloc({str: %struct.JSONTest.School*}, )
//                     call(@ir.map.set(%4: {str: %struct.JSONTest.School*}, "a": str, %1: %struct.JSONTest.School*, ) -> void, ) !ir_debug_location !1
//                     set_field(%2: %struct.JSONTest.Person*, %4: {str: %struct.JSONTest.School*}, 2: i32, ) !ir_debug_location !1
//                     let %5: %struct.JSONTest.Person* !ir_debug_location !1 = %2: %struct.JSONTest.Person*
//                     call(@ir.builtin.print(call(@ir.json.encode(%5: %struct.JSONTest.Person*, ) -> str, ) , ) -> void, ) !ir_debug_location !2
//                     ret(call(@ir.json.encode(%5: %struct.JSONTest.Person*, ) -> str, ) , ) !ir_debug_location !3
//             }
//
//         }
//         meta !0 = !{15: u32, 17: u32, "test.sonar": str, }
//         meta !1 = !{18: u32, 23: u32, "test.sonar": str, }
//         meta !2 = !{24: u32, 24: u32, "test.sonar": str, }
//         meta !3 = !{25: u32, 25: u32, "test.sonar": str, }
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
