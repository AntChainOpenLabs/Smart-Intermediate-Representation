// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn math_pow_unsigned_pow_calculation_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Pow"
        contract Pow {
            state {
            }
            pub fn Pow.Pow.init()  {
                0:
                    ret()
            }

            pub fn Pow.Pow.augpow8() -> bool {
                1:
                    let %0: u8 !ir_debug_location !0 = 2: u8
                    let %1: u8 !ir_debug_location !1 = call(@ir.math.pow(%0: u8, 3: u8, ) -> u8, )
                    %0 !ir_debug_location !2 = call(@ir.math.pow(%0: u8, 3: u8, ) -> u8, )
                    ret(eq(%0: u8, %1: u8, ) , ) !ir_debug_location !3
            }

            pub fn Pow.Pow.pow32() -> u32 {
                2:
                    let %0: u32 !ir_debug_location !4 = 2: u32
                    let %1: u32 !ir_debug_location !5 = call(@ir.math.pow(%0: u32, 10: u32, ) -> u32, )
                    %0 !ir_debug_location !6 = call(@ir.math.pow(%0: u32, 10: u32, ) -> u32, )
                    call(@ir.builtin.require(eq(%0: u32, %1: u32, ) , "test failed": str, ) -> void, ) !ir_debug_location !7
                    ret(%1: u32, ) !ir_debug_location !8
            }

            pub fn Pow.Pow.pow64() -> u64 {
                3:
                    let %0: u64 !ir_debug_location !9 = 2: u64
                    let %1: u64 !ir_debug_location !10 = call(@ir.math.pow(%0: u64, 10: u64, ) -> u64, )
                    %0 !ir_debug_location !11 = call(@ir.math.pow(%0: u64, 10: u64, ) -> u64, )
                    call(@ir.builtin.require(eq(%0: u64, %1: u64, ) , "test failed": str, ) -> void, ) !ir_debug_location !12
                    ret(%1: u64, ) !ir_debug_location !13
            }

            pub fn Pow.Pow.pow128() -> u128 {
                4:
                    let %0: u128 !ir_debug_location !14 = 2: u128
                    let %1: u128 !ir_debug_location !15 = call(@ir.math.pow(%0: u128, 10: u128, ) -> u128, )
                    %0 !ir_debug_location !16 = call(@ir.math.pow(%0: u128, 10: u128, ) -> u128, )
                    call(@ir.builtin.require(eq(%0: u128, %1: u128, ) , "test failed": str, ) -> void, ) !ir_debug_location !17
                    ret(%1: u128, ) !ir_debug_location !18
            }

            pub fn Pow.Pow.powlit() -> u64 {
                5:
                    ret(call(@ir.math.pow(int_cast(2: u64, ) -> u64 , 3: u64, ) -> u64, ) , ) !ir_debug_location !19
            }

            pub fn Pow.Pow.pow(%0: u32, ) -> u32 {
                6:
                    ret(call(@ir.math.pow(int_cast(2: u32, ) -> u32 , %0: u32, ) -> u32, ) , ) !ir_debug_location !20
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !5 = !{12: u32, 12: u32, "test.sonar": str, }
        meta !6 = !{13: u32, 13: u32, "test.sonar": str, }
        meta !7 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !8 = !{15: u32, 15: u32, "test.sonar": str, }
        meta !9 = !{19: u32, 19: u32, "test.sonar": str, }
        meta !10 = !{20: u32, 20: u32, "test.sonar": str, }
        meta !11 = !{21: u32, 21: u32, "test.sonar": str, }
        meta !12 = !{22: u32, 22: u32, "test.sonar": str, }
        meta !13 = !{23: u32, 23: u32, "test.sonar": str, }
        meta !14 = !{27: u32, 27: u32, "test.sonar": str, }
        meta !15 = !{28: u32, 28: u32, "test.sonar": str, }
        meta !16 = !{29: u32, 29: u32, "test.sonar": str, }
        meta !17 = !{30: u32, 30: u32, "test.sonar": str, }
        meta !18 = !{31: u32, 31: u32, "test.sonar": str, }
        meta !19 = !{35: u32, 35: u32, "test.sonar": str, }
        meta !20 = !{39: u32, 39: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 7);
    assert_eq!(abi.methods[1].inputs.len(), 0);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn math_pow_signed_pow_calculation_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Pow"
        contract Pow {
            state {
            }
            pub fn Pow.Pow.init()  {
                0:
                    ret()
            }

            pub fn Pow.Pow.augpow8() -> bool {
                1:
                    let %0: i8 !ir_debug_location !0 = -2: i8
                    let %1: i8 !ir_debug_location !1 = call(@ir.math.pow(%0: i8, 3: i8, ) -> i8, )
                    %0 !ir_debug_location !2 = call(@ir.math.pow(%0: i8, 3: i8, ) -> i8, )
                    ret(eq(%0: i8, %1: i8, ) , ) !ir_debug_location !3
            }

            pub fn Pow.Pow.pow32() -> i32 {
                2:
                    let %0: i32 !ir_debug_location !4 = -2: i32
                    let %1: i32 !ir_debug_location !5 = call(@ir.math.pow(%0: i32, 10: i32, ) -> i32, )
                    %0 !ir_debug_location !6 = call(@ir.math.pow(%0: i32, 10: i32, ) -> i32, )
                    call(@ir.builtin.require(eq(%0: i32, %1: i32, ) , "test failed": str, ) -> void, ) !ir_debug_location !7
                    ret(%1: i32, ) !ir_debug_location !8
            }

            pub fn Pow.Pow.pow64() -> i64 {
                3:
                    let %0: i64 !ir_debug_location !9 = -2: i64
                    let %1: i64 !ir_debug_location !10 = call(@ir.math.pow(%0: i64, 10: i64, ) -> i64, )
                    %0 !ir_debug_location !11 = call(@ir.math.pow(%0: i64, 10: i64, ) -> i64, )
                    call(@ir.builtin.require(eq(%0: i64, %1: i64, ) , "test failed": str, ) -> void, ) !ir_debug_location !12
                    ret(%1: i64, ) !ir_debug_location !13
            }

            pub fn Pow.Pow.pow128() -> i128 {
                4:
                    let %0: i128 !ir_debug_location !14 = -2: i128
                    let %1: i128 !ir_debug_location !15 = call(@ir.math.pow(%0: i128, 10: i128, ) -> i128, )
                    %0 !ir_debug_location !16 = call(@ir.math.pow(%0: i128, 10: i128, ) -> i128, )
                    call(@ir.builtin.require(eq(%0: i128, %1: i128, ) , "test failed": str, ) -> void, ) !ir_debug_location !17
                    ret(%1: i128, ) !ir_debug_location !18
            }

            pub fn Pow.Pow.powlit() -> i64 {
                5:
                    ret(call(@ir.math.pow(int_cast(-2: i64, ) -> i64 , 4: i64, ) -> i64, ) , ) !ir_debug_location !19
            }

            pub fn Pow.Pow.pow(%0: i32, ) -> i32 {
                6:
                    ret(sub(0: i32, call(@ir.math.pow(int_cast(2: i32, ) -> i32 , %0: i32, ) -> i32, ) , ) , ) !ir_debug_location !20
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !5 = !{12: u32, 12: u32, "test.sonar": str, }
        meta !6 = !{13: u32, 13: u32, "test.sonar": str, }
        meta !7 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !8 = !{15: u32, 15: u32, "test.sonar": str, }
        meta !9 = !{19: u32, 19: u32, "test.sonar": str, }
        meta !10 = !{20: u32, 20: u32, "test.sonar": str, }
        meta !11 = !{21: u32, 21: u32, "test.sonar": str, }
        meta !12 = !{22: u32, 22: u32, "test.sonar": str, }
        meta !13 = !{23: u32, 23: u32, "test.sonar": str, }
        meta !14 = !{27: u32, 27: u32, "test.sonar": str, }
        meta !15 = !{28: u32, 28: u32, "test.sonar": str, }
        meta !16 = !{29: u32, 29: u32, "test.sonar": str, }
        meta !17 = !{30: u32, 30: u32, "test.sonar": str, }
        meta !18 = !{31: u32, 31: u32, "test.sonar": str, }
        meta !19 = !{35: u32, 35: u32, "test.sonar": str, }
        meta !20 = !{39: u32, 39: u32, "test.sonar": str, }

        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 7);
    assert_eq!(abi.methods[1].inputs.len(), 0);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn math_mixed_precedence_calculation_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Calculation"
        contract Calculation {
            state {
            }
            pub fn Calculation.Calculation.init()  {
                0:
                    ret()
            }

            pub fn Calculation.Calculation.mix32() -> u32 {
                1:
                    let %0: u32 !ir_debug_location !0 = 2: u32
                    let %1: u32 !ir_debug_location !1 = sub(add(call(@ir.math.pow(int_cast(0: u32, ) -> u32 , %0: u32, ) -> u32, ) , mul(%0: u32, 10: u32, ) , ) , div(4: u32, 2: u32, ) , )
                    let %2: u32 !ir_debug_location !2 = sub(add(call(@ir.math.pow(%0: u32, 0: u32, ) -> u32, ) , mul(%0: u32, 10: u32, ) , ) , div(4: u32, 2: u32, ) , )
                    ret(%1: u32, ) !ir_debug_location !3
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
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
fn math_itoa_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Calculation"
        contract Calculation {
            state {
            }
            pub fn Calculation.Calculation.init()  {
                0:
                    ret()
            }

            pub fn Calculation.Calculation.i8_to_str(%0: i8, %1: i32, ) -> str {
                1:
                    let %2: str !ir_debug_location !0 = call(@ir.math.itoa(%0: i8, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !1
                    ret(%2: str, ) !ir_debug_location !2
            }

            pub fn Calculation.Calculation.i16_to_str(%0: i16, %1: i32, ) -> str {
                2:
                    let %2: str !ir_debug_location !3 = call(@ir.math.itoa(%0: i16, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !4
                    ret(%2: str, ) !ir_debug_location !5
            }

            pub fn Calculation.Calculation.i32_to_str(%0: i32, %1: i32, ) -> str {
                3:
                    let %2: str !ir_debug_location !6 = call(@ir.math.itoa(%0: i32, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !7
                    ret(%2: str, ) !ir_debug_location !8
            }

            pub fn Calculation.Calculation.i64_to_str(%0: i64, %1: i32, ) -> str {
                4:
                    let %2: str !ir_debug_location !9 = call(@ir.math.itoa(%0: i64, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !10
                    ret(%2: str, ) !ir_debug_location !11
            }

            pub fn Calculation.Calculation.i128_to_str(%0: i128, %1: i32, ) -> str {
                5:
                    let %2: str !ir_debug_location !12 = call(@ir.math.itoa(%0: i128, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !13
                    ret(%2: str, ) !ir_debug_location !14
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !3 = !{9: u32, 9: u32, "test.sonar": str, }
        meta !4 = !{10: u32, 10: u32, "test.sonar": str, }
        meta !5 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !6 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !7 = !{15: u32, 15: u32, "test.sonar": str, }
        meta !8 = !{16: u32, 16: u32, "test.sonar": str, }
        meta !9 = !{19: u32, 19: u32, "test.sonar": str, }
        meta !10 = !{20: u32, 20: u32, "test.sonar": str, }
        meta !11 = !{21: u32, 21: u32, "test.sonar": str, }
        meta !12 = !{24: u32, 24: u32, "test.sonar": str, }
        meta !13 = !{25: u32, 25: u32, "test.sonar": str, }
        meta !14 = !{26: u32, 26: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 6);
    assert_eq!(abi.methods[1].inputs.len(), 2);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn math_uitoa_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Calculation"
        contract Calculation {
            state {
            }
            pub fn Calculation.Calculation.init()  {
                0:
                    ret()
            }

            pub fn Calculation.Calculation.u8_to_str(%0: u8, %1: i32, ) -> str {
                1:
                    let %2: str !ir_debug_location !0 = call(@ir.math.itoa(%0: u8, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !1
                    ret(%2: str, ) !ir_debug_location !2
            }

            pub fn Calculation.Calculation.u16_to_str(%0: u16, %1: i32, ) -> str {
                2:
                    let %2: str !ir_debug_location !3 = call(@ir.math.itoa(%0: u16, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !4
                    ret(%2: str, ) !ir_debug_location !5
            }

            pub fn Calculation.Calculation.u32_to_str(%0: u32, %1: i32, ) -> str {
                3:
                    let %2: str !ir_debug_location !6 = call(@ir.math.itoa(%0: u32, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !7
                    ret(%2: str, ) !ir_debug_location !8
            }

            pub fn Calculation.Calculation.u64_to_str(%0: u64, %1: i32, ) -> str {
                4:
                    let %2: str !ir_debug_location !9 = call(@ir.math.itoa(%0: u64, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !10
                    ret(%2: str, ) !ir_debug_location !11
            }

            pub fn Calculation.Calculation.u128_to_str(%0: u128, %1: i32, ) -> str {
                5:
                    let %2: str !ir_debug_location !12 = call(@ir.math.itoa(%0: u128, %1: i32, ) -> str, )
                    call(@ir.builtin.print(%2: str, ) -> void, ) !ir_debug_location !13
                    ret(%2: str, ) !ir_debug_location !14
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !3 = !{9: u32, 9: u32, "test.sonar": str, }
        meta !4 = !{10: u32, 10: u32, "test.sonar": str, }
        meta !5 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !6 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !7 = !{15: u32, 15: u32, "test.sonar": str, }
        meta !8 = !{16: u32, 16: u32, "test.sonar": str, }
        meta !9 = !{19: u32, 19: u32, "test.sonar": str, }
        meta !10 = !{20: u32, 20: u32, "test.sonar": str, }
        meta !11 = !{21: u32, 21: u32, "test.sonar": str, }
        meta !12 = !{24: u32, 24: u32, "test.sonar": str, }
        meta !13 = !{25: u32, 25: u32, "test.sonar": str, }
        meta !14 = !{26: u32, 26: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 6);
    assert_eq!(abi.methods[1].inputs.len(), 2);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}
