// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn declare_const_int_outside_function() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "simpleContract"
        contract simpleContract {
            state {
            }
            pub fn simpleContract.simpleContract.init()  {
                0:
                    ret()
            }

            pub fn simpleContract.simpleContract.test() -> i32 {
                1:
                    ret(123: i32, ) !ir_debug_location !0
            }

        }
        meta !0 = !{5: u32, 5: u32, "test.sonar": str, }
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
fn declare_const_str_outside_function() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "simpleContract"
        contract simpleContract {
            state {
            }
            pub fn simpleContract.simpleContract.init()  {
                0:
                    ret()
            }

            pub fn simpleContract.simpleContract.test() -> str {
                1:
                    ret("abc": str, ) !ir_debug_location !0
            }

        }
        meta !0 = !{5: u32, 5: u32, "test.sonar": str, }
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
