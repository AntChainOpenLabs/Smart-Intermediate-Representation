// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_literal_vec() {
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

            pub fn Test.Test.test() -> u32  {
                1:
                    let %0: [u64; 3] !ir_debug_location !0 = malloc([u64; 3], )
                    call(@ir.vector.push(%0: [u64; 3], 3: u64, ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [u64; 3], 4: u64, ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [u64; 3], 5: u64, ) -> void, ) !ir_debug_location !0
                    let %1: [u64] !ir_debug_location !0 = %0: [u64; 3]
                    let %2: u32 !ir_debug_location !0 = call(@ir.vector.len(%1: [u64], ) -> u32, )
                    ret(%2: u32, ) !ir_debug_location !0
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
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
    assert_eq!(
        runtime.call("test", hex::decode("00").unwrap().as_slice()),
        encode(vec![ABIParam::U32(3)].as_slice(), VERSION)
    );
}

#[test]
fn test_literal_repeat_vec() {
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
                    let %0: [u64; 10] !ir_debug_location !0 = malloc([u64; 10], )
                    let %1: i32 !ir_debug_location !0 = 0: i32
                    br(bb 2, ) !ir_debug_location !0
                2:
                    br_if(ne(%1: i32, 10: u32, ) , bb 3, bb 4, ) !ir_debug_location !0
                3:
                    call(@ir.vector.push(%0: [u64; 10], 1: u64, ) -> void, ) !ir_debug_location !0
                    %1 !ir_debug_location !0 = add(%1: i32, 1: i32, )
                    br(bb 2, ) !ir_debug_location !0
                4:
                    let %2: [u64] !ir_debug_location !0 = %0: [u64; 10]
                    ret() !ir_debug_location !0
            }
        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
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
