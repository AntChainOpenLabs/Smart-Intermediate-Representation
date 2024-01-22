// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_if_stmt() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Test"
        contract Test {
            state {
            }
            pub fn Test.Test.init()  {
                0:
                    ret()
            }

            pub fn Test.Test.test(%0: bool, ) -> bool {
                1:
                    br_if(true: bool, bb 2, bb 4, ) !ir_debug_location !0
                2:
                    ret(true: bool, ) !ir_debug_location !1
                3:
                    ret(false: bool, ) !ir_debug_location !3
                4:
                    ret(false: bool, ) !ir_debug_location !2
            }

        }
        meta !0 = !{4: u32, 8: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !3 = !{9: u32, 9: u32, "test.sonar": str, }
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
fn test_simple_contract_with_ifelse() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "IfElseContract"
        contract IfElseContract {
            state {
            }
            pub fn IfElseContract.IfElseContract.init()  {
                0:
                    ret()
            }

            pub fn IfElseContract.IfElseContract.ifTest(%0: u64, %1: u64, ) -> u64 {
                1:
                    br_if(gt(%0: u64, %1: u64, ) , bb 2, bb 4, ) !ir_debug_location !0
                2:
                    ret(%0: u64, ) !ir_debug_location !1
                3:
                    ret() !ir_debug_location !4
                4:
                    br_if(eq(%0: u64, %1: u64, ) , bb 5, bb 7, ) !ir_debug_location !2
                5:
                    ret(add(%0: u64, %1: u64, ) , ) !ir_debug_location !3
                6:
                    br(bb 3, ) !ir_debug_location !4
                7:
                    ret(%1: u64, ) !ir_debug_location !4
            }

        }
        meta !0 = !{4: u32, 10: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 10: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{9: u32, 9: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 2);
    assert_eq!(abi.methods[1].inputs.len(), 2);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_simple_contract_with_conditional_expr() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "ifExprTest"
        contract ifExprTest {
            state {
            }
            pub fn ifExprTest.ifExprTest.init()  {
                0:
                    ret()
            }

            pub fn ifExprTest.ifExprTest.ifTest() -> i32 {
                1:
                    let %0: i32 !ir_debug_location !0 = 10: i32
                    let %1: i32 !ir_debug_location !1 = 20: i32
                    let %2: i32 !ir_debug_location !2
                    br_if(lt(%0: i32, %1: i32, ) , bb 2, bb 3, ) !ir_debug_location !2
                2:
                    %2 !ir_debug_location !2 = %0: i32
                    br(bb 4, ) !ir_debug_location !2
                3:
                    %2 !ir_debug_location !2 = %1: i32
                    br(bb 4, ) !ir_debug_location !2
                4:
                    let %3: i32 !ir_debug_location !2 = %2: i32
                    ret(%3: i32, ) !ir_debug_location !3
            }

            pub fn ifExprTest.ifExprTest.ifTest2() -> str {
                5:
                    let %0: str !ir_debug_location !4 = "aaa": str
                    let %1: str !ir_debug_location !5 = "bbb": str
                    let %2: i32 !ir_debug_location !6 = -3: i32
                    let %3: str !ir_debug_location !7
                    br_if(lt(%2: i32, 0: i32, ) , bb 6, bb 7, ) !ir_debug_location !7
                6:
                    %3 !ir_debug_location !7 = %1: str
                    br(bb 8, ) !ir_debug_location !7
                7:
                    %3 !ir_debug_location !7 = "ccc": str
                    br(bb 8, ) !ir_debug_location !7
                8:
                    let %4: str !ir_debug_location !7
                    br_if(gt(%2: i32, 0: i32, ) , bb 9, bb 10, ) !ir_debug_location !7
                9:
                    %4 !ir_debug_location !7 = %0: str
                    br(bb 11, ) !ir_debug_location !7
                10:
                    %4 !ir_debug_location !7 = %3: str
                    br(bb 11, ) !ir_debug_location !7
                11:
                    let %5: str !ir_debug_location !7 = %4: str
                    ret(%5: str, ) !ir_debug_location !8
            }

            pub fn ifExprTest.ifExprTest.ifTest3() -> i32 {
                12:
                    let %0: i32 !ir_debug_location !9 = 10: i32
                    let %1: i32 !ir_debug_location !10 = 20: i32
                    let %2: i32 !ir_debug_location !11
                    br_if(eq(%0: i32, %1: i32, ) , bb 13, bb 14, ) !ir_debug_location !11
                13:
                    %2 !ir_debug_location !11 = -50: i32
                    br(bb 15, ) !ir_debug_location !11
                14:
                    %2 !ir_debug_location !11 = -60: i32
                    br(bb 15, ) !ir_debug_location !11
                15:
                    let %3: i32 !ir_debug_location !11
                    br_if(gt(%0: i32, %1: i32, ) , bb 16, bb 17, ) !ir_debug_location !11
                16:
                    %3 !ir_debug_location !11 = -40: i32
                    br(bb 18, ) !ir_debug_location !11
                17:
                    %3 !ir_debug_location !11 = %2: i32
                    br(bb 18, ) !ir_debug_location !11
                18:
                    let %4: i32 !ir_debug_location !11
                    br_if(gt(%0: i32, %1: i32, ) , bb 19, bb 20, ) !ir_debug_location !11
                19:
                    %4 !ir_debug_location !11 = -30: i32
                    br(bb 21, ) !ir_debug_location !11
                20:
                    %4 !ir_debug_location !11 = %3: i32
                    br(bb 21, ) !ir_debug_location !11
                21:
                    let %5: i32 !ir_debug_location !11 = %4: i32
                    ret(%5: i32, ) !ir_debug_location !12
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{10: u32, 10: u32, "test.sonar": str, }
        meta !5 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !6 = !{12: u32, 12: u32, "test.sonar": str, }
        meta !7 = !{13: u32, 13: u32, "test.sonar": str, }
        meta !8 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !9 = !{17: u32, 17: u32, "test.sonar": str, }
        meta !10 = !{18: u32, 18: u32, "test.sonar": str, }
        meta !11 = !{19: u32, 19: u32, "test.sonar": str, }
        meta !12 = !{20: u32, 20: u32, "test.sonar": str, }
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
