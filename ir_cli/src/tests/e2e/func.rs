// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_func_param_def() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"module_name = "SimpleVectorContract"
        contract SimpleVectorContract {
            state {
            }
            pub fn SimpleVectorContract.SimpleVectorContract.init()  {
                0:
                    ret()
            }

            pub fn SimpleVectorContract.SimpleVectorContract.simple_vector_lit() -> [str] {
                1:
                    let %0: [str; 3] !ir_debug_location !0 = malloc([str; 3], )
                    call(@ir.vector.push(%0: [str; 3], "a": str, ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [str; 3], "b": str, ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [str; 3], "c": str, ) -> void, ) !ir_debug_location !0
                    ret(%0: [str; 3], ) !ir_debug_location !0
            }

            pub fn SimpleVectorContract.SimpleVectorContract.simple_para_vector(%0: [str], ) -> [str] {
                2:
                    let %1: str !ir_debug_location !1 = "1": str
                    ret(%0: [str], ) !ir_debug_location !2
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !2 = !{8: u32, 8: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 3);
    assert_eq!(abi.methods[1].inputs.len(), 0);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}
