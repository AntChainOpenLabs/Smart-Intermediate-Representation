// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_literal_map() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "Test"
        contract Test {
            state {
            }
            pub fn Test.Test.init()  {
                0:
                    ret()
            }

            pub fn Test.Test.test()  {
                1:
                    let %0: {str: i32} !ir_debug_location !0 = malloc({str: i32}, )
                    call(@ir.map.set(%0: {str: i32}, "alice": str, 1: i32, ) -> void, ) !ir_debug_location !0
                    call(@ir.map.set(%0: {str: i32}, "bob": str, 2: i32, ) -> void, ) !ir_debug_location !0
                    let %1: {str: i32} !ir_debug_location !0 = %0: {str: i32}
                    ret() !ir_debug_location !0
            }

        }
        meta !0 = !{4: u32, 7: u32, "test.sonar": str, }
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
