// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;
#[test]
fn test_simple_contract_with_break() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "breakTest"
        contract breakTest {
            state {
            }
            pub fn breakTest.breakTest.init()  {
                0:
                    ret()
            }

            pub fn breakTest.breakTest.breakTest(%0: u64, ) -> u64 {
                1:
                    let %1: u64 !ir_debug_location !0 = 0: u64
                    let %2: u64 !ir_debug_location !1 = 0: u64
                    br(bb 2, ) !ir_debug_location !2
                2:
                    br_if(lt(%2: u64, %0: u64, ) , bb 3, bb 4, ) !ir_debug_location !2
                3:
                    %1 !ir_debug_location !3 = add(%1: u64, 5: u64, )
                    %2 !ir_debug_location !4 = add(%2: u64, 1: u64, )
                    br_if(eq(%2: u64, 10: u64, ) , bb 5, bb 6, ) !ir_debug_location !5
                4:
                    ret(%1: u64, ) !ir_debug_location !7
                5:
                    br(bb 4, ) !ir_debug_location !6
                6:
                    br(bb 2, ) !ir_debug_location !6
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 12: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{8: u32, 8: u32, "test.sonar": str, }
        meta !5 = !{9: u32, 11: u32, "test.sonar": str, }
        meta !6 = !{10: u32, 10: u32, "test.sonar": str, }
        meta !7 = !{13: u32, 13: u32, "test.sonar": str, }
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

// #[test]
// fn test_simple_map_for_with_break() {
//     let mut runtime_and_abi = build_mock_runtime(
//         r#" module_name = "SimpleMapContract"
//         contract SimpleMapContract {
//             state {
//             }
//             pub fn SimpleMapContract.SimpleMapContract.init()  {
//                 0:
//                     ret()
//             }
//
//             pub fn SimpleMapContract.SimpleMapContract.simple_map(%0: {str: u64}, ) -> u64 {
//                 1:
//                     call(@ir.map.insert(%0: {str: u64}, "k2": str, 20: u64, ) -> bool, ) !ir_debug_location !0
//                     call(@ir.map.insert(%0: {str: u64}, "k3": str, 30: u64, ) -> bool, ) !ir_debug_location !1
//                     let %1: u64 !ir_debug_location !2 = 0: u64
//                     let %2: %ir.map.iter !ir_debug_location !3 = call(@ir.map.create_iter(%0: {str: u64}, ) -> %ir.map.iter, )
//                     br(bb 2, ) !ir_debug_location !3
//                 2:
//                     br_if(call(@ir.map.get_next(%2: %ir.map.iter, ) -> bool, ) , bb 3, bb 4, ) !ir_debug_location !3
//                 3:
//                     let %3: str !ir_debug_location !3 = call(@ir.map.obj_key(%2: %ir.map.iter, ) -> str, )
//                     let %4: u64 !ir_debug_location !3 = call(@ir.map.obj_value(%2: %ir.map.iter, ) -> u64, )
//                     %1 !ir_debug_location !4 = add(%1: u64, %4: u64, )
//                     br_if(eq(%1: u64, 30: u64, ) , bb 5, bb 6, ) !ir_debug_location !5
//                 4:
//                     ret(%1: u64, ) !ir_debug_location !7
//                 5:
//                     br(bb 4, ) !ir_debug_location !6
//                 6:
//                     br(bb 2, ) !ir_debug_location !6
//             }
//
//         }
//         meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
//         meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
//         meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
//         meta !3 = !{7: u32, 12: u32, "test.sonar": str, }
//         meta !4 = !{8: u32, 8: u32, "test.sonar": str, }
//         meta !5 = !{9: u32, 11: u32, "test.sonar": str, }
//         meta !6 = !{10: u32, 10: u32, "test.sonar": str, }
//         meta !7 = !{13: u32, 13: u32, "test.sonar": str, }
//         "#,
//     );
//     let mut runtime = runtime_and_abi.0;
//     let abi = runtime_and_abi.1;
//     // ABI
//     assert_eq!(abi.methods.len(), 2);
//     assert_eq!(abi.methods[1].inputs.len(), 1);
//     // Deploy contract and call contract constructor.
//     runtime.constructor(hex::decode("00").unwrap().as_slice());
//     // Contract method call.
// }

#[test]
fn test_simple_vector_for_with_break() {
    let mut runtime_and_abi = build_mock_runtime(
        r#" module_name = "SimpleVectorContract"
        contract SimpleVectorContract {
            state {
            }
            pub fn SimpleVectorContract.SimpleVectorContract.init()  {
                0:
                    ret()
            }

            pub fn SimpleVectorContract.SimpleVectorContract.simple_vector(%0: [u64], ) -> u64 {
                1:
                    call(@ir.vector.push(%0: [u64], 3: u64, ) -> void, ) !ir_debug_location !0
                    call(@ir.vector.push(%0: [u64], 4: u64, ) -> void, ) !ir_debug_location !1
                    call(@ir.vector.push(%0: [u64], 5: u64, ) -> void, ) !ir_debug_location !2
                    let %1: u64 !ir_debug_location !3 = 0: u64
                    let %2: %ir.vector.iter !ir_debug_location !4 = call(@ir.vector.create_iter(%0: [u64], ) -> %ir.vector.iter, )
                    br(bb 2, ) !ir_debug_location !4
                2:
                    br_if(call(@ir.vector.get_next(%2: %ir.vector.iter, ) -> bool, ) , bb 3, bb 4, ) !ir_debug_location !4
                3:
                    let %3: u64 !ir_debug_location !4 = call(@ir.vector.obj_value(%2: %ir.vector.iter, ) -> u64, )
                    %1 !ir_debug_location !5 = add(%1: u64, %3: u64, )
                    br_if(eq(%1: u64, 10: u64, ) , bb 5, bb 6, ) !ir_debug_location !6
                4:
                    ret(%1: u64, ) !ir_debug_location !8
                5:
                    br(bb 4, ) !ir_debug_location !7
                6:
                    br(bb 2, ) !ir_debug_location !7
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{8: u32, 13: u32, "test.sonar": str, }
        meta !5 = !{9: u32, 9: u32, "test.sonar": str, }
        meta !6 = !{10: u32, 12: u32, "test.sonar": str, }
        meta !7 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !8 = !{14: u32, 14: u32, "test.sonar": str, }
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
