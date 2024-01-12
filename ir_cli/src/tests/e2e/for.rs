// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_for_stmt() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Test"
        contract Test {
            state {
            }
            pub fn Test.Test.init()  {
                0:
                    ret()
            }

            pub fn Test.Test.test() -> u64 {
                1:
                    let %0: [u64; 3] !ir_debug_location !0 = malloc([u64; 3], )
                    call(@ir.vector.push(%0: [u64; 3], 3: u64, ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [u64; 3], 4: u64, ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [u64; 3], 5: u64, ) -> void, ) !ir_debug_location !0
                    let %1: [u64] !ir_debug_location !0 = %0: [u64; 3]
                    let %2: u64 !ir_debug_location !1 = 0: u64
                    let %3: %ir.vector.iter !ir_debug_location !2 = call(@ir.vector.create_iter(%1: [u64], ) -> %ir.vector.iter, )
                    br(bb 2, ) !ir_debug_location !2
                2:
                    br_if(call(@ir.vector.get_next(%3: %ir.vector.iter, ) -> bool, ) , bb 3, bb 4, ) !ir_debug_location !2
                3:
                    let %4: u64 !ir_debug_location !2 = call(@ir.vector.obj_value(%3: %ir.vector.iter, ) -> u64, )
                    %2 !ir_debug_location !3 = add(%2: u64, %4: u64, )
                    br(bb 2, ) !ir_debug_location !3
                4:
                    ret(%2: u64, ) !ir_debug_location !4
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 8: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{9: u32, 9: u32, "test.sonar": str, }
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
fn test_for_stmt_index() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Test"
        contract Test {
            state {
            }
            pub fn Test.Test.init()  {
                0:
                    ret()
            }

            pub fn Test.Test.test() -> u64 {
                1:
                    let %0: [u64; 3] !ir_debug_location !0 = malloc([u64; 3], )
                    call(@ir.vector.push(%0: [u64; 3], 3: u64, ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [u64; 3], 4: u64, ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [u64; 3], 5: u64, ) -> void, ) !ir_debug_location !0
                    let %1: [u64] !ir_debug_location !0 = %0: [u64; 3]
                    let %2: u64 !ir_debug_location !1 = 0: u64
                    let %3: %ir.vector.iter !ir_debug_location !2 = call(@ir.vector.create_iter(%1: [u64], ) -> %ir.vector.iter, )
                    br(bb 2, ) !ir_debug_location !2
                2:
                    br_if(call(@ir.vector.get_next(%3: %ir.vector.iter, ) -> bool, ) , bb 3, bb 4, ) !ir_debug_location !2
                3:
                    let %4: u64 !ir_debug_location !2 = call(@ir.vector.obj_value(%3: %ir.vector.iter, ) -> u64, )
                    let %5: u32 !ir_debug_location !2 = call(@ir.vector.obj_key(%3: %ir.vector.iter, ) -> u32, )
                    %2 !ir_debug_location !3 = add(%2: u64, %4: u64, )
                    br(bb 2, ) !ir_debug_location !3
                4:
                    ret(%2: u64, ) !ir_debug_location !4
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 8: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{9: u32, 9: u32, "test.sonar": str, }
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
