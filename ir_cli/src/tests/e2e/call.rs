// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_contract_inner_func_call() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "Test"
        contract Test {
            state {
            }
            pub fn Test.Test.init()  {
                0:
                    ret()
            }

            fn Test.Test.foo()  {
                1:
                    ret()
            }

            fn Test.Test.foo1() -> u8 {
                2:
                    ret(1: u8, ) !ir_debug_location !0
            }

            pub fn Test.Test.test()  {
                3:
                    call(@Test.Test.foo() -> void, ) !ir_debug_location !1
                    call(@Test.Test.foo1() -> u8, ) !ir_debug_location !2
                    let %0: u8 !ir_debug_location !3 = call(@Test.Test.foo1() -> u8, )
                    ret() !ir_debug_location !3
            }

        }
        meta !0 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !1 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !2 = !{12: u32, 12: u32, "test.sonar": str, }
        meta !3 = !{13: u32, 13: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 4);
    assert_eq!(abi.methods[1].inputs.len(), 0);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

// #[test]
// fn test_cocall() {
//     let mut runtime_and_abi = build_mock_runtime(
//         r#"module_name = "Test"
//         contract Test {
//             state {
//             }
//             pub fn Test.Test.init()  {
//                 0:
//                     ret()
//             }
//
//             pub fn Test.Test.transfer1(%0: str, %1: u64, )  {
//                 1:
//                     let %2: str !ir_debug_location !0 = "inst.ERC20.co": str
//                     call(@ir.builtin.co_call(%2: str, "foo": str, %0: str, %1: u64, ) -> void, ) !ir_debug_location !1
//                     call(@ir.builtin.co_call("inst.ERC20.co": str, "foo": str, %0: str, %1: u64, ) -> void, ) !ir_debug_location !2
//                     ret() !ir_debug_location !2
//             }
//
//         }
//         meta !0 = !{8: u32, 8: u32, "test.sonar": str, }
//         meta !1 = !{9: u32, 9: u32, "test.sonar": str, }
//         meta !2 = !{10: u32, 10: u32, "test.sonar": str, }
//         "#,
//     );
//     let mut runtime = runtime_and_abi.0;
//     let abi = runtime_and_abi.1;
//     // ABI
//     assert_eq!(abi.methods.len(), 2);
//     assert_eq!(abi.methods[1].inputs.len(), 2);
//     // Deploy contract and call contract constructor.
//     runtime.constructor(hex::decode("00").unwrap().as_slice());
//     // Contract method call.
// }

#[test]
fn test_simple_contract_internal_call() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Contract"
        contract Contract {
            state {
            }
            pub fn Contract.Contract.init()  {
                0:
                    ret()
            }

            pub fn Contract.Contract.u8_add(%0: u8, %1: u8, ) -> u8 {
                1:
                    ret(add(%0: u8, %1: u8, ) , ) !ir_debug_location !0
            }

            pub fn Contract.Contract.u8_sub(%0: u8, %1: u8, ) -> u8 {
                2:
                    ret(sub(%0: u8, %1: u8, ) , ) !ir_debug_location !1
            }

            pub fn Contract.Contract.calculation(%0: u8, %1: u8, ) -> u8 {
                3:
                    let %2: u8 !ir_debug_location !2 = call(@Contract.Contract.u8_add(%0: u8, %1: u8, ) -> u8, )
                    let %3: u8 !ir_debug_location !3 = call(@Contract.Contract.u8_sub(%0: u8, %1: u8, ) -> u8, )
                    ret(add(%2: u8, %3: u8, ) , ) !ir_debug_location !4
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !2 = !{10: u32, 10: u32, "test.sonar": str, }
        meta !3 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !4 = !{12: u32, 12: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 4);
    assert_eq!(abi.methods[1].inputs.len(), 2);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_complex_contract_internal_call() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Contract"
        contract Contract {
            state {
            }
            pub fn Contract.Contract.init()  {
                0:
                    ret()
            }

            fn Contract.Contract.add(%0: u64, %1: u64, ) -> u64 {
                1:
                    ret(add(%0: u64, %1: u64, ) , ) !ir_debug_location !0
            }

            fn Contract.Contract.add_twice(%0: u64, %1: u64, ) -> u64 {
                2:
                    ret(mul(call(@Contract.Contract.add(%0: u64, %1: u64, ) -> u64, ) , 2: u64, ) , ) !ir_debug_location !1
            }

            pub fn Contract.Contract.calculation(%0: u64, %1: u64, ) -> u64 {
                3:
                    ret(add(call(@Contract.Contract.add(%0: u64, call(@Contract.Contract.add_twice(%0: u64, %1: u64, ) -> u64, ) , ) -> u64, ) , call(@Contract.Contract.add_twice(%0: u64, call(@Contract.Contract.add(%0: u64, %1: u64, ) -> u64, ) , ) -> u64, ) , ) , ) !ir_debug_location !2
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !2 = !{10: u32, 10: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 4);
    assert_eq!(abi.methods[1].inputs.len(), 2);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_simple_contract_recursive_call() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Contract"
        contract Contract {
            state {
            }
            pub fn Contract.Contract.init()  {
                0:
                    ret()
            }

            fn Contract.Contract.inner_recursive(%0: u32, ) -> u32 {
                1:
                    br_if(eq(%0: u32, 1: u32, ) , bb 2, bb 3, ) !ir_debug_location !0
                2:
                    ret(1: u32, ) !ir_debug_location !1
                3:
                    ret(add(call(@Contract.Contract.inner_recursive(sub(%0: u32, 1: u32, ) , ) -> u32, ) , 1: u32, ) , ) !ir_debug_location !2
            }

            pub fn Contract.Contract.recursive(%0: u32, ) -> u32 {
                4:
                    ret(call(@Contract.Contract.inner_recursive(%0: u32, ) -> u32, ) , ) !ir_debug_location !3
            }

        }
        meta !0 = !{4: u32, 6: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !3 = !{10: u32, 10: u32, "test.sonar": str, }
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
fn test_simple_contract_call_empty_arr() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Contract"
        contract Contract {
            state {
            }
            pub fn Contract.Contract.init()  {
                0:
                    ret()
            }

            pub fn Contract.Contract.func(%0: [u64], ) -> u64 {
                1:
                    ret(123: u64, ) !ir_debug_location !0
            }

            pub fn Contract.Contract.callFun() -> u64 {
                2:
                    let %0: [u64; 0] !ir_debug_location !1 = malloc([u64; 0], )
                    let %1: u64 !ir_debug_location !1 = call(@Contract.Contract.func(%0: [u64; 0], ) -> u64, )
                    ret(%1: u64, ) !ir_debug_location !2
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{8: u32, 8: u32, "test.sonar": str, }
        meta !2 = !{9: u32, 9: u32, "test.sonar": str, }
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
fn test_simple_contract_call_map() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Contract"
        contract Contract {
            state {
            }
            pub fn Contract.Contract.init()  {
                0:
                    ret()
            }

            pub fn Contract.Contract.func(%0: {str: u64}, ) -> u64 {
                1:
                    ret(123: u64, ) !ir_debug_location !0
            }

            pub fn Contract.Contract.callFun() -> u64 {
                2:
                    let %0: {str: u64} !ir_debug_location !1 = malloc({str: u64}, )
                    let %1: u64 !ir_debug_location !1 = call(@Contract.Contract.func(%0: {str: u64}, ) -> u64, )
                    ret(%1: u64, ) !ir_debug_location !2
            }

            pub fn Contract.Contract.callFun2() -> u64 {
                3:
                    let %0: {str: u64} !ir_debug_location !3 = malloc({str: u64}, )
                    call(@ir.map.set(%0: {str: u64}, "aaa": str, 12: u64, ) -> void, ) !ir_debug_location !3
                    let %1: u64 !ir_debug_location !3 = call(@Contract.Contract.func(%0: {str: u64}, ) -> u64, )
                    ret(%1: u64, ) !ir_debug_location !4
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{8: u32, 8: u32, "test.sonar": str, }
        meta !2 = !{9: u32, 9: u32, "test.sonar": str, }
        meta !3 = !{13: u32, 13: u32, "test.sonar": str, }
        meta !4 = !{14: u32, 14: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 4);
    assert_eq!(abi.methods[1].inputs.len(), 1);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_func_order_independent() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "FuncOrderIndependent"
        contract FuncOrderIndependent {
            state {
            }
            pub fn FuncOrderIndependent.FuncOrderIndependent.init()  {
                0:
                    ret()
            }

            pub fn FuncOrderIndependent.FuncOrderIndependent.call1(%0: str, ) -> str {
                1:
                    ret(call(@FuncOrderIndependent.FuncOrderIndependent.call2(%0: str, ) -> str, ) , ) !ir_debug_location !0
            }

            pub fn FuncOrderIndependent.FuncOrderIndependent.call2(%0: str, ) -> str {
                2:
                    ret(%0: str, ) !ir_debug_location !1
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
