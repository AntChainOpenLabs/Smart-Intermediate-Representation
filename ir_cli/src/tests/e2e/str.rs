// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_str_intrinsic() {
    // Construct a mock runtime with the textual ir code
    let mut runtime_and_abi = build_mock_runtime(
        r#"
        module_name = "Test"
        contract Test {
            state {
            }
            pub fn Test.Test.init()  {
                0:
                    ret()
            }

            pub fn Test.Test.test()  {
                1:
                    let %0: str !ir_debug_location !0 = "hello world": str
                    let %1: [str] !ir_debug_location !1 = call(@ir.str.split(%0: str, "o": str, ) -> [str], )
                    let %2: u32 !ir_debug_location !2 = call(@ir.str.len(%0: str, ) -> u32, )
                    let %3: str !ir_debug_location !3 = call(@ir.str.lower(%0: str, ) -> str, )
                    let %4: str !ir_debug_location !4 = call(@ir.str.upper(%0: str, ) -> str, )
                    let %5: u8 !ir_debug_location !5 = call(@ir.str.at(%0: str, 0: u32, ) -> u8, )
                    let %6: u32 !ir_debug_location !6 = call(@ir.str.count(%0: str, "o": str, 0: u32, 10: u32, ) -> u32, )
                    let %7: bool !ir_debug_location !7 = call(@ir.str.startswith(%0: str, "hello": str, 0: u32, 10: u32, ) -> bool, )
                    let %8: bool !ir_debug_location !8 = call(@ir.str.endswith(%0: str, "hello": str, 0: u32, 10: u32, ) -> bool, )
                    let %9: bool !ir_debug_location !9 = call(@ir.str.isalnum(%0: str, ) -> bool, )
                    let %10: bool !ir_debug_location !10 = call(@ir.str.isalpha(%0: str, ) -> bool, )
                    let %11: bool !ir_debug_location !11 = call(@ir.str.isdigit(%0: str, ) -> bool, )
                    let %12: bool !ir_debug_location !12 = call(@ir.str.islower(%0: str, ) -> bool, )
                    let %13: bool !ir_debug_location !13 = call(@ir.str.isupper(%0: str, ) -> bool, )
                    let %14: bool !ir_debug_location !14 = call(@ir.str.isspace(%0: str, ) -> bool, )
                    let %15: str !ir_debug_location !15 = call(@ir.str.strip(%0: str, "o": str, ) -> str, )
                    let %16: str !ir_debug_location !16 = call(@ir.str.lstrip(%0: str, "o": str, ) -> str, )
                    let %17: str !ir_debug_location !17 = call(@ir.str.rstrip(%0: str, "o": str, ) -> str, )
                    let %18: [str; 1] !ir_debug_location !18 = malloc([str; 1], )
                    call(@ir.vector.push(%18: [str; 1], "o": str, ) -> void, ) !ir_debug_location !18
                    let %19: str !ir_debug_location !18 = call(@ir.str.join(%0: str, %18: [str; 1], ) -> str, )
                    let %20: str !ir_debug_location !19 = call(@ir.str.concat(%0: str, "o": str, ) -> str, )
                    let %21: str !ir_debug_location !20 = call(@ir.str.replace(%0: str, "o": str, "a": str, 1: u32, ) -> str, )
                    let %22: u32 !ir_debug_location !21 = call(@ir.str.find(%0: str, "0": str, 0: u32, 10: u32, ) -> u32, )
                    let %23: str !ir_debug_location !22 = call(@ir.str.substr(%0: str, 0: u32, 5: u32, ) -> str, )
                    let %24: str !ir_debug_location !23 = call(@ir.str.insert(%0: str, "o": str, 5: u32, ) -> str, )
                    let %25: [u8] !ir_debug_location !24 = call(@ir.str.to_bytes(%0: str, ) -> [u8], )
                    ret() !ir_debug_location !24
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{8: u32, 8: u32, "test.sonar": str, }
        meta !5 = !{9: u32, 9: u32, "test.sonar": str, }
        meta !6 = !{10: u32, 10: u32, "test.sonar": str, }
        meta !7 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !8 = !{12: u32, 12: u32, "test.sonar": str, }
        meta !9 = !{13: u32, 13: u32, "test.sonar": str, }
        meta !10 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !11 = !{15: u32, 15: u32, "test.sonar": str, }
        meta !12 = !{16: u32, 16: u32, "test.sonar": str, }
        meta !13 = !{17: u32, 17: u32, "test.sonar": str, }
        meta !14 = !{18: u32, 18: u32, "test.sonar": str, }
        meta !15 = !{19: u32, 19: u32, "test.sonar": str, }
        meta !16 = !{20: u32, 20: u32, "test.sonar": str, }
        meta !17 = !{21: u32, 21: u32, "test.sonar": str, }
        meta !18 = !{22: u32, 22: u32, "test.sonar": str, }
        meta !19 = !{23: u32, 23: u32, "test.sonar": str, }
        meta !20 = !{24: u32, 24: u32, "test.sonar": str, }
        meta !21 = !{25: u32, 25: u32, "test.sonar": str, }
        meta !22 = !{26: u32, 26: u32, "test.sonar": str, }
        meta !23 = !{27: u32, 27: u32, "test.sonar": str, }
        meta !24 = !{28: u32, 28: u32, "test.sonar": str, }
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
    runtime.call("test", hex::decode("00").unwrap().as_slice());
}

#[test]
fn test_simple_str_declaration() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"
        module_name = "SimpleBuiltinContract"
        contract SimpleBuiltinContract {
            state {
            }
            pub fn SimpleBuiltinContract.SimpleBuiltinContract.init()  {
                0:
                    ret()
            }

            pub fn SimpleBuiltinContract.SimpleBuiltinContract.test() -> str {
                1:
                    let %0: str !ir_debug_location !0 = "hello": str
                    let %1: str !ir_debug_location !1 = %0: str
                    ret(%1: str, ) !ir_debug_location !2
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
    runtime.call("test", hex::decode("00").unwrap().as_slice());
}
