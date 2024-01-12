// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

// #[test]
// fn test_builtin_func_call() {
//     let mut runtime_and_abi = build_mock_runtime(
//         r#" module_name = "Test"
//         contract Test {
//             state {
//                 u8_arr: [u8],
//             }
//             pub fn Test.Test.init()  {
//                 0:
//                     ret()
//             }
//
//             pub fn Test.Test.test()  {
//                 1:
//                     let %0: [u8; 1] !ir_debug_location !0 = malloc([u8; 1], )
//                     call(@ir.vector.push(%0: [u8; 1], 1: u8, ) -> void, ) !ir_debug_location !0
//                     let %1: [u8] !ir_debug_location !0 = %0: [u8; 1]
//                     call(@ir.vector.push(%1: [u8], 1: u8, ) -> void, ) !ir_debug_location !1
//                     let %2: {str: u8} !ir_debug_location !2 = malloc({str: u8}, )
//                     let %3: {str: u8} !ir_debug_location !2 = %2: {str: u8}
//                     call(@ir.map.insert(%3: {str: u8}, "a": str, 1: u8, ) -> bool, ) !ir_debug_location !3
//                     call(@ir.storage.push(get_storage_path("u8_arr": str, ) !ir_storage_path_extra_args !5 , 1: u8, ) -> void, ) !ir_debug_location !4
//                     call(@ir.builtin.print("a": str, ) -> void, ) !ir_debug_location !6
//                     ret() !ir_debug_location !6
//             }
//
//         }
//         meta !0 = !{8: u32, 8: u32, "test.sonar": str, }
//         meta !1 = !{9: u32, 9: u32, "test.sonar": str, }
//         meta !2 = !{10: u32, 10: u32, "test.sonar": str, }
//         meta !3 = !{11: u32, 11: u32, "test.sonar": str, }
//         meta !4 = !{12: u32, 12: u32, "test.sonar": str, }
//         meta !5 = !{1: u32, }
//         meta !6 = !{13: u32, 13: u32, "test.sonar": str, }
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

#[test]
fn test_tx_context() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "Test"
        contract Test {
            state {
            }
            pub fn Test.Test.init()  {
                0:
                    ret()
            }

            pub fn Test.Test.test()  {
                1:
                    let %0: u64 !ir_debug_location !0 = call(@ir.builtin.block_number() -> u64, )
                    let %1: u64 !ir_debug_location !1 = call(@ir.builtin.block_timestamp() -> u64, )
                    let %2: str !ir_debug_location !2 = call(@ir.builtin.tx_hash() -> str, )
                    let %3: str !ir_debug_location !3 = call(@ir.builtin.tx_sender() -> str, )
                    let %4: u64 !ir_debug_location !4 = call(@ir.builtin.tx_timestamp() -> u64, )
                    let %5: u64 !ir_debug_location !5 = call(@ir.builtin.tx_nonce() -> u64, )
                    let %6: u32 !ir_debug_location !6 = call(@ir.builtin.tx_index() -> u32, )
                    let %7: str !ir_debug_location !7 = call(@ir.builtin.call_sender() -> str, )
                    let %8: str !ir_debug_location !8 = call(@ir.builtin.call_this_contract() -> str, )
                    let %9: str !ir_debug_location !9 = call(@ir.builtin.call_op_contract() -> str, )
                    ret() !ir_debug_location !9
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !3 = !{8: u32, 8: u32, "test.sonar": str, }
        meta !4 = !{9: u32, 9: u32, "test.sonar": str, }
        meta !5 = !{10: u32, 10: u32, "test.sonar": str, }
        meta !6 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !7 = !{13: u32, 13: u32, "test.sonar": str, }
        meta !8 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !9 = !{15: u32, 15: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 2);
    assert_eq!(abi.methods[1].inputs.len(), 0);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_simple_base64_builtin_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "SimpleBuiltinContract"
        contract SimpleBuiltinContract {
            state {
            }
            pub fn SimpleBuiltinContract.SimpleBuiltinContract.init()  {
                0:
                    ret()
            }

            pub fn SimpleBuiltinContract.SimpleBuiltinContract.str_encode_base64(%0: [u8], ) -> str {
                1:
                    ret(call(@ir.builtin.encode_base64(%0: [u8], ) -> str, ) , ) !ir_debug_location !0
            }

            pub fn SimpleBuiltinContract.SimpleBuiltinContract.str_decode_base64(%0: str, ) -> [u8] {
                2:
                    ret(call(@ir.builtin.decode_base64(%0: str, ) -> [u8], ) , ) !ir_debug_location !1
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{7: u32, 7: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 3);
    assert_eq!(abi.methods[1].inputs.len(), 1);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_simple_hex_builtin_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "SimpleBuiltinContract"
        contract SimpleBuiltinContract {
            state {
            }
            pub fn SimpleBuiltinContract.SimpleBuiltinContract.init()  {
                0:
                    ret()
            }

            pub fn SimpleBuiltinContract.SimpleBuiltinContract.str_encode_hex(%0: [u8], ) -> str {
                1:
                    ret(call(@ir.builtin.encode_hex(%0: [u8], ) -> str, ) , ) !ir_debug_location !0
            }

            pub fn SimpleBuiltinContract.SimpleBuiltinContract.str_decode_hex(%0: str, ) -> [u8] {
                2:
                    ret(call(@ir.builtin.decode_hex(%0: str, ) -> [u8], ) , ) !ir_debug_location !1
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{7: u32, 7: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 3);
    assert_eq!(abi.methods[1].inputs.len(), 1);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_builtin_call_log_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "SimpleBuiltinContract"
        contract SimpleBuiltinContract {
            state {
            }
            pub fn SimpleBuiltinContract.SimpleBuiltinContract.init()  {
                0:
                    ret()
            }

            pub fn SimpleBuiltinContract.SimpleBuiltinContract.log_test()  {
                1:
                    let %0: [[u8]; 2] !ir_debug_location !0 = malloc([[u8]; 2], )
                    call(@ir.vector.push(%0: [[u8]; 2], call(@ir.str.to_bytes("hello": str, ) -> [u8], ) , ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [[u8]; 2], call(@ir.str.to_bytes("world": str, ) -> [u8], ) , ) -> void, ) !ir_debug_location !0
                    let %1: [[u8]] !ir_debug_location !0 = %0: [[u8]; 2]
                    let %2: [u8] !ir_debug_location !1 = call(@ir.str.to_bytes("sss": str, ) -> [u8], )
                    call(@ir.builtin.call_log(%1: [[u8]], %2: [u8], ) -> void, ) !ir_debug_location !2
                    ret() !ir_debug_location !2
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 2);
    assert_eq!(abi.methods[1].inputs.len(), 0);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

// #[test]
// fn test_print_type() {
//     let mut runtime_and_abi = build_mock_runtime(
//         r#" module_name = "PrintTypeTest"
//         type struct.PrintTypeTest.School = {name: str, }
//         type struct.PrintTypeTest.Person = {name: str, age: u128, schools: [%struct.PrintTypeTest.School*; 2], map: {str: bool}, vec: [u8], }
//         contract PrintTypeTest {
//             state {
//             }
//             pub fn PrintTypeTest.PrintTypeTest.init()  {
//                 0:
//                     ret()
//             }
//
//             pub fn PrintTypeTest.PrintTypeTest.test_print_i8() -> bool {
//                 1:
//                     call(@ir.builtin.print_type(int_cast(12: i8, ) -> i8 , ) -> void, ) !ir_debug_location !0
//                     ret(true: bool, ) !ir_debug_location !1
//             }
//
//             pub fn PrintTypeTest.PrintTypeTest.test_print_struct() -> bool {
//                 2:
//                     let %0: %struct.PrintTypeTest.Person* !ir_debug_location !2 = malloc(%struct.PrintTypeTest.Person, )
//                     set_field(%0: %struct.PrintTypeTest.Person*, "p1": str, 0: i32, ) !ir_debug_location !2
//                     set_field(%0: %struct.PrintTypeTest.Person*, 123: u128, 1: i32, ) !ir_debug_location !2
//                     let %1: [%struct.PrintTypeTest.School*; 2] !ir_debug_location !2 = malloc([%struct.PrintTypeTest.School*; 2], )
//                     let %2: %struct.PrintTypeTest.School* !ir_debug_location !2 = malloc(%struct.PrintTypeTest.School, )
//                     set_field(%2: %struct.PrintTypeTest.School*, "s1": str, 0: i32, ) !ir_debug_location !2
//                     call(@ir.vector.push(%1: [%struct.PrintTypeTest.School*; 2], %2: %struct.PrintTypeTest.School*, ) -> void, ) !ir_debug_location !2
//                     let %3: %struct.PrintTypeTest.School* !ir_debug_location !2 = malloc(%struct.PrintTypeTest.School, )
//                     set_field(%3: %struct.PrintTypeTest.School*, "s2": str, 0: i32, ) !ir_debug_location !2
//                     call(@ir.vector.push(%1: [%struct.PrintTypeTest.School*; 2], %3: %struct.PrintTypeTest.School*, ) -> void, ) !ir_debug_location !2
//                     set_field(%0: %struct.PrintTypeTest.Person*, %1: [%struct.PrintTypeTest.School*; 2], 2: i32, ) !ir_debug_location !2
//                     let %4: {str: bool} !ir_debug_location !2 = malloc({str: bool}, )
//                     call(@ir.map.set(%4: {str: bool}, "k1": str, true: bool, ) -> void, ) !ir_debug_location !2
//                     set_field(%0: %struct.PrintTypeTest.Person*, %4: {str: bool}, 3: i32, ) !ir_debug_location !2
//                     let %5: [u8; 1] !ir_debug_location !2 = malloc([u8; 1], )
//                     call(@ir.vector.push(%5: [u8; 1], 1: u8, ) -> void, ) !ir_debug_location !2
//                     set_field(%0: %struct.PrintTypeTest.Person*, %5: [u8; 1], 4: i32, ) !ir_debug_location !2
//                     let %6: %struct.PrintTypeTest.Person* !ir_debug_location !2 = %0: %struct.PrintTypeTest.Person*
//                     call(@ir.builtin.print_type(%6: %struct.PrintTypeTest.Person*, ) -> void, ) !ir_debug_location !3
//                     ret(true: bool, ) !ir_debug_location !4
//             }
//
//         }
//         meta !0 = !{14: u32, 14: u32, "test.sonar": str, }
//         meta !1 = !{15: u32, 15: u32, "test.sonar": str, }
//         meta !2 = !{18: u32, 27: u32, "test.sonar": str, }
//         meta !3 = !{28: u32, 28: u32, "test.sonar": str, }
//         meta !4 = !{29: u32, 29: u32, "test.sonar": str, }
//         "#,
//     );
//     let mut runtime = runtime_and_abi.0;
//     let abi = runtime_and_abi.1;
//     // ABI
//     assert_eq!(abi.methods.len(), 3);
//     assert_eq!(abi.methods[1].inputs.len(), 0);
//     // Deploy contract and call contract constructor.
//     runtime.constructor(hex::decode("00").unwrap().as_slice());
//     // Contract method call.
// }

// #[test]
// fn test_builtin_sha256_contract() {
//     let mut runtime_and_abi = build_mock_runtime(
//         r#" module_name = "SimpleBuiltinContract"
//         contract SimpleBuiltinContract {
//             state {
//             }
//             pub fn SimpleBuiltinContract.SimpleBuiltinContract.init()  {
//                 0:
//                     ret()
//             }
//
//             pub fn SimpleBuiltinContract.SimpleBuiltinContract.sha256_test(%0: [u8], ) -> [u8; 32] {
//                 1:
//                     ret(call(@ir.builtin.sha256(%0: [u8], ) -> [u8; 32], ) , ) !ir_debug_location !0
//             }
//
//         }
//         meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
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

// #[test]
// fn test_builtin_sm3_contract() {
//     let mut runtime_and_abi = build_mock_runtime(
//         r#" module_name = "SimpleBuiltinContract"
//         contract SimpleBuiltinContract {
//             state {
//             }
//             pub fn SimpleBuiltinContract.SimpleBuiltinContract.init()  {
//                 0:
//                     ret()
//             }
//
//             pub fn SimpleBuiltinContract.SimpleBuiltinContract.sm3_test(%0: [u8], ) -> [u8; 32] {
//                 1:
//                     ret(call(@ir.builtin.sm3(%0: [u8], ) -> [u8; 32], ) , ) !ir_debug_location !0
//             }
//
//         }
//         meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
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

#[test]
fn test_builtin_keccak256_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "SimpleBuiltinContract"
        contract SimpleBuiltinContract {
            state {
            }
            pub fn SimpleBuiltinContract.SimpleBuiltinContract.init()  {
                0:
                    ret()
            }

            pub fn SimpleBuiltinContract.SimpleBuiltinContract.keccak256_test(%0: [u8], ) -> [u8; 32] {
                1:
                    ret(call(@ir.builtin.keccak256(%0: [u8], ) -> [u8; 32], ) , ) !ir_debug_location !0
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 2);
    assert_eq!(abi.methods[1].inputs.len(), 1);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_print_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "PrintTest"
        contract PrintTest {
            state {
            }
            pub fn PrintTest.PrintTest.init()  {
                0:
                    ret()
            }

            pub fn PrintTest.PrintTest.print_func()  {
                1:
                    let %0: str !ir_debug_location !0 = "aaa": str
                    call(@ir.builtin.print(%0: str, ) -> void, ) !ir_debug_location !1
                    call(@ir.builtin.print("bbb": str, ) -> void, ) !ir_debug_location !2
                    ret() !ir_debug_location !2
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 2);
    assert_eq!(abi.methods[1].inputs.len(), 0);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}
