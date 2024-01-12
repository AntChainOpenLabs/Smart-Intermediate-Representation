// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;

#[test]
fn simple_hello_world_contract() {
    // Construct a mock runtime with the textual ir code
    let mut runtime_and_abi = build_mock_runtime(
        r#"
        module_name = "HelloWorld"
        contract HelloWorld {
            state {
            }
            pub fn HelloWorld.HelloWorld.init()  {
                0:
                    call(@ir.builtin.add_coverage_counter(0: u32, ) -> void, )
                    ret()
            }

            pub fn HelloWorld.HelloWorld.greeting() -> str {
                1:
                    call(@ir.builtin.add_coverage_counter(1: u32, ) -> void, )
                    call(@ir.builtin.print("Hello Smart Intermediate Representation": str, ) -> void, ) !ir_debug_location !0
                    ret("Hello world!": str, ) !ir_debug_location !1
            }

            pub fn HelloWorld.HelloWorld.greeting2(%0: str, ) -> str {
                1:
                    call(@ir.builtin.add_coverage_counter(1: u32, ) -> void, )
                    call(@ir.builtin.print("Hello Smart Intermediate Representation": str, ) -> void, ) !ir_debug_location !0
                    ret(%0: str, ) !ir_debug_location !1
            }

        }
        meta !0 = !{3: u32, 3: u32, "examples/hello_world.ir": str, }
        meta !1 = !{4: u32, 4: u32, "examples/hello_world.ir": str, }
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
    assert_eq!(
        runtime.call("greeting", hex::decode("00").unwrap().as_slice()),
        encode(
            vec![ABIParam::Str("Hello world!".into())].as_slice(),
            VERSION
        )
    );
}
