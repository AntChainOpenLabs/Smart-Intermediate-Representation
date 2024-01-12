// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_ssz() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"
        module_name = "SSZ"
        contract SSZ {
            state {
            }
            pub fn SSZ.SSZ.init()  {
                0:
                    ret()
            }

            pub fn SSZ.SSZ.encode_u8() -> [u8] {
                1:
                    ret(call(@ir.ssz.encode(int_cast(1: u8, ) -> u8 , ) -> [u8], ) , ) !ir_debug_location !0
            }

            pub fn SSZ.SSZ.decode_i8() -> i8 {
                2:
                    let %0: [i8; 1] !ir_debug_location !1 = malloc([i8; 1], )
                    call(@ir.vector.push(%0: [i8; 1], 1: i8, ) -> void, ) !ir_debug_location !1
                    ret(call(@ir.ssz.decode(%0: [i8; 1], ) -> i8, ) , ) !ir_debug_location !1
            }

        }
        meta !0 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !1 = !{9: u32, 9: u32, "test.sonar": str, }
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
    // runtime.call("encode_u8", hex::decode("00").unwrap().as_slice());
}
