// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_simple_caculation() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "SimpleCalculation"
        contract SimpleCalculation {
            state {
            }
            pub fn SimpleCalculation.SimpleCalculation.init()  {
                0:
                    ret()
            }

            pub fn SimpleCalculation.SimpleCalculation.calculate(%0: u64, ) -> u64 {
                1:
                    let %1: u64 !ir_debug_location !0 = %0: u64
                    %1 !ir_debug_location !1 = add(%1: u64, 2: u64, )
                    %1 !ir_debug_location !2 = mul(%1: u64, 2: u64, )
                    %1 !ir_debug_location !3 = sub(%1: u64, 1: u64, )
                    %1 !ir_debug_location !4 = div(%1: u64, 1: u64, )
                    let %2: u64 !ir_debug_location !5 = add(%1: u64, div(mul(2: u64, 2: u64, ) , 2: u64, ) , )
                    call(@ir.builtin.require(eq(add(sub(mul(add(%0: u64, 2: u64, ) , 2: u64, ) , 1: u64, ) , div(mul(2: u64, 2: u64, ) , 2: u64, ) , ) , %2: u64, ) , "Calculated failed": str, ) -> void, ) !ir_debug_location !6
                    ret(%2: u64, ) !ir_debug_location !7
            }

            pub fn SimpleCalculation.SimpleCalculation.compare() -> bool {
                2:
                    let %0: u64 !ir_debug_location !8 = 1: u64
                    let %1: bool !ir_debug_location !9 = gt(%0: u64, 1: u64, )
                    let %2: bool !ir_debug_location !10 = ge(%0: u64, 1: u64, )
                    let %3: bool !ir_debug_location !11 = lt(%0: u64, 1: u64, )
                    let %4: bool !ir_debug_location !12 = le(%0: u64, 1: u64, )
                    call(@ir.builtin.require(eq(%1: bool, false: bool, ) , "result1 require failed": str, ) -> void, ) !ir_debug_location !13
                    call(@ir.builtin.require(eq(%2: bool, true: bool, ) , "result2 require failed": str, ) -> void, ) !ir_debug_location !14
                    call(@ir.builtin.require(eq(%3: bool, false: bool, ) , "result3 require failed": str, ) -> void, ) !ir_debug_location !15
                    call(@ir.builtin.require(eq(%4: bool, true: bool, ) , "result4 require failed": str, ) -> void, ) !ir_debug_location !16
                    call(@ir.builtin.require(ne(%1: bool, true: bool, ) , "result1 require failed": str, ) -> void, ) !ir_debug_location !17
                    call(@ir.builtin.require(ne(%2: bool, false: bool, ) , "result2 require failed": str, ) -> void, ) !ir_debug_location !18
                    call(@ir.builtin.require(ne(%3: bool, true: bool, ) , "result3 require failed": str, ) -> void, ) !ir_debug_location !19
                    call(@ir.builtin.require(ne(%4: bool, false: bool, ) , "result4 require failed": str, ) -> void, ) !ir_debug_location !20
                    let %5: bool !ir_debug_location !21 = or(or(%1: bool, and(%2: bool, %3: bool, ) , ) , %4: bool, )
                    ret(%5: bool, ) !ir_debug_location !22
            }

            pub fn SimpleCalculation.SimpleCalculation.shift() -> u64 {
                3:
                    let %0: u64 !ir_debug_location !23 = shl(1: u64, 1: u64, )
                    let %1: u64 !ir_debug_location !24 = shr(2: u64, 1: u64, )
                    call(@ir.builtin.require(eq(%0: u64, 2: u64, ) , "result1 require failed": str, ) -> void, ) !ir_debug_location !25
                    call(@ir.builtin.require(eq(%1: u64, 1: u64, ) , "result2 require failed": str, ) -> void, ) !ir_debug_location !26
                    ret(add(%0: u64, %1: u64, ) , ) !ir_debug_location !27
            }

            pub fn SimpleCalculation.SimpleCalculation.logic() -> bool {
                4:
                    let %0: bool !ir_debug_location !28 = true: bool
                    let %1: bool !ir_debug_location !29 = false: bool
                    let %2: bool !ir_debug_location !30 = and(%0: bool, true: bool, )
                    let %3: bool !ir_debug_location !31 = or(%0: bool, true: bool, )
                    let %4: bool !ir_debug_location !32 = and(%0: bool, false: bool, )
                    let %5: bool !ir_debug_location !33 = or(%0: bool, false: bool, )
                    let %6: bool !ir_debug_location !34 = not(%0: bool, )
                    let %7: bool !ir_debug_location !35 = not(%1: bool, )
                    call(@ir.builtin.require(eq(%2: bool, true: bool, ) , "result1 require failed": str, ) -> void, ) !ir_debug_location !36
                    call(@ir.builtin.require(eq(%3: bool, true: bool, ) , "result2 require failed": str, ) -> void, ) !ir_debug_location !37
                    call(@ir.builtin.require(eq(%4: bool, false: bool, ) , "result3 require failed": str, ) -> void, ) !ir_debug_location !38
                    call(@ir.builtin.require(eq(%5: bool, true: bool, ) , "result4 require failed": str, ) -> void, ) !ir_debug_location !39
                    call(@ir.builtin.require(eq(%6: bool, false: bool, ) , "result5 require failed": str, ) -> void, ) !ir_debug_location !40
                    call(@ir.builtin.require(eq(%7: bool, true: bool, ) , "result6 require failed": str, ) -> void, ) !ir_debug_location !41
                    call(@ir.builtin.require(eq(bit_or(bit_and(int_cast(1: u32, ) -> u32 , 2: u32, ) , bit_xor(3: u32, 4: u32, ) , ) , 7: u32, ) , "bit logic require failed": str, ) -> void, ) !ir_debug_location !42
                    ret(and(%2: bool, %3: bool, ) , ) !ir_debug_location !43
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !2 = !{8: u32, 8: u32, "test.sonar": str, }
        meta !3 = !{10: u32, 10: u32, "test.sonar": str, }
        meta !4 = !{12: u32, 12: u32, "test.sonar": str, }
        meta !5 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !6 = !{15: u32, 15: u32, "test.sonar": str, }
        meta !7 = !{16: u32, 16: u32, "test.sonar": str, }
        meta !8 = !{20: u32, 20: u32, "test.sonar": str, }
        meta !9 = !{21: u32, 21: u32, "test.sonar": str, }
        meta !10 = !{22: u32, 22: u32, "test.sonar": str, }
        meta !11 = !{23: u32, 23: u32, "test.sonar": str, }
        meta !12 = !{24: u32, 24: u32, "test.sonar": str, }
        meta !13 = !{25: u32, 25: u32, "test.sonar": str, }
        meta !14 = !{26: u32, 26: u32, "test.sonar": str, }
        meta !15 = !{27: u32, 27: u32, "test.sonar": str, }
        meta !16 = !{28: u32, 28: u32, "test.sonar": str, }
        meta !17 = !{30: u32, 30: u32, "test.sonar": str, }
        meta !18 = !{31: u32, 31: u32, "test.sonar": str, }
        meta !19 = !{32: u32, 32: u32, "test.sonar": str, }
        meta !20 = !{33: u32, 33: u32, "test.sonar": str, }
        meta !21 = !{35: u32, 35: u32, "test.sonar": str, }
        meta !22 = !{36: u32, 36: u32, "test.sonar": str, }
        meta !23 = !{40: u32, 40: u32, "test.sonar": str, }
        meta !24 = !{41: u32, 41: u32, "test.sonar": str, }
        meta !25 = !{42: u32, 42: u32, "test.sonar": str, }
        meta !26 = !{43: u32, 43: u32, "test.sonar": str, }
        meta !27 = !{44: u32, 44: u32, "test.sonar": str, }
        meta !28 = !{48: u32, 48: u32, "test.sonar": str, }
        meta !29 = !{49: u32, 49: u32, "test.sonar": str, }
        meta !30 = !{50: u32, 50: u32, "test.sonar": str, }
        meta !31 = !{51: u32, 51: u32, "test.sonar": str, }
        meta !32 = !{52: u32, 52: u32, "test.sonar": str, }
        meta !33 = !{53: u32, 53: u32, "test.sonar": str, }
        meta !34 = !{54: u32, 54: u32, "test.sonar": str, }
        meta !35 = !{55: u32, 55: u32, "test.sonar": str, }
        meta !36 = !{56: u32, 56: u32, "test.sonar": str, }
        meta !37 = !{57: u32, 57: u32, "test.sonar": str, }
        meta !38 = !{58: u32, 58: u32, "test.sonar": str, }
        meta !39 = !{59: u32, 59: u32, "test.sonar": str, }
        meta !40 = !{60: u32, 60: u32, "test.sonar": str, }
        meta !41 = !{61: u32, 61: u32, "test.sonar": str, }
        meta !42 = !{62: u32, 62: u32, "test.sonar": str, }
        meta !43 = !{63: u32, 63: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 5);
    assert_eq!(abi.methods[1].inputs.len(), 1);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}
