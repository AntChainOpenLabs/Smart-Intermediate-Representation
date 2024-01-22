// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_bigint() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "BigIntContract"
        contract BigIntContract {
            state {
            }
            pub fn BigIntContract.BigIntContract.init()  {
                0:
                    ret()
            }

            pub fn BigIntContract.BigIntContract.u128Test(%0: u128, ) -> u128 {
                1:
                    let %1: u128 !ir_debug_location !0 = 10: u128
                    ret(add(%0: u128, %1: u128, ) , ) !ir_debug_location !1
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
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
