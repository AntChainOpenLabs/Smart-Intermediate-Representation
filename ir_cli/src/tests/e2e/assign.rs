// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::tests::{encode, VERSION};
use crate::vm::build_mock_runtime;
use smart_ir::abi::params::ABIParam;
use smart_ir::encoding::datastream::ParamType;

#[test]
fn test_simple_assign_ops_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"
        module_name = "SimpleAssignOps"
        contract SimpleAssignOps {
            state {
            }
            pub fn SimpleAssignOps.SimpleAssignOps.init()  {
                0:
                    ret()
            }

            pub fn SimpleAssignOps.SimpleAssignOps.str_assign(%0: str, ) -> str {
                1:
                    let %1: str !ir_debug_location !0 = %0: str
                    let %2: str !ir_debug_location !1 = %1: str
                    let %3: str !ir_debug_location !2 = "123": str
                    %3 !ir_debug_location !3 = "456": str
                    ret(%2: str, ) !ir_debug_location !4
            }

            pub fn SimpleAssignOps.SimpleAssignOps.int_assign() -> u8 {
                2:
                    let %0: u8 !ir_debug_location !5 = 1: u8
                    let %1: u8 !ir_debug_location !6 = %0: u8
                    let %2: u8 !ir_debug_location !7 = add(%0: u8, %1: u8, )
                    ret(%2: u8, ) !ir_debug_location !8
            }

            pub fn SimpleAssignOps.SimpleAssignOps.aug_assign(%0: u64, ) -> u64 {
                3:
                    let %1: u64 !ir_debug_location !9 = %0: u64
                    %1 !ir_debug_location !10 = mod(%1: u64, 2: u64, )
                    %1 !ir_debug_location !11 = shl(%1: u64, 1: u64, )
                    %1 !ir_debug_location !12 = shr(%1: u64, 1: u64, )
                    %1 !ir_debug_location !13 = bit_and(%1: u64, 1: u64, )
                    %1 !ir_debug_location !14 = bit_or(%1: u64, 1: u64, )
                    %1 !ir_debug_location !15 = bit_xor(%1: u64, 1: u64, )
                    ret(%1: u64, ) !ir_debug_location !16
            }

        }
        meta !0 = !{4: u32, 4: u32, "test.sonar": str, }
        meta !1 = !{5: u32, 5: u32, "test.sonar": str, }
        meta !2 = !{6: u32, 6: u32, "test.sonar": str, }
        meta !3 = !{7: u32, 7: u32, "test.sonar": str, }
        meta !4 = !{8: u32, 8: u32, "test.sonar": str, }
        meta !5 = !{11: u32, 11: u32, "test.sonar": str, }
        meta !6 = !{12: u32, 12: u32, "test.sonar": str, }
        meta !7 = !{13: u32, 13: u32, "test.sonar": str, }
        meta !8 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !9 = !{17: u32, 17: u32, "test.sonar": str, }
        meta !10 = !{18: u32, 18: u32, "test.sonar": str, }
        meta !11 = !{19: u32, 19: u32, "test.sonar": str, }
        meta !12 = !{20: u32, 20: u32, "test.sonar": str, }
        meta !13 = !{21: u32, 21: u32, "test.sonar": str, }
        meta !14 = !{22: u32, 22: u32, "test.sonar": str, }
        meta !15 = !{23: u32, 23: u32, "test.sonar": str, }
        meta !16 = !{24: u32, 24: u32, "test.sonar": str, }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    let abi = runtime_and_abi.1;
    // ABI
    assert_eq!(abi.methods.len(), 4);
    assert_eq!(abi.methods[1].inputs.len(), 1);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_simple_add_assign_contract() {
    let mut runtime_and_abi = build_mock_runtime(
    r#"
        module_name = "SimpleStorage"
        contract SimpleStorage {
            state {
            }
            pub fn SimpleStorage.SimpleStorage.init()  {
                0:
                    ret()
            }

            pub fn SimpleStorage.SimpleStorage.add(%0: u64, %1: u64, ) -> u64 {
                1:
                    let %2: u64 !ir_debug_location !0 = %0: u64
                    %2 !ir_debug_location !1 = add(%2: u64, %1: u64, )
                    ret(%2: u64, ) !ir_debug_location !2
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
    assert_eq!(abi.methods[1].inputs.len(), 2);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_simple_mul_assign_contract() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"
        module_name = "SimpleStorage"
        contract SimpleStorage {
            state {
            }
            pub fn SimpleStorage.SimpleStorage.init()  {
                0:
                    ret()
            }

            pub fn SimpleStorage.SimpleStorage.mul(%0: u64, %1: u64, ) -> u64 {
                1:
                    let %2: u64 !ir_debug_location !0 = %0: u64
                    %2 !ir_debug_location !1 = mul(%2: u64, %1: u64, )
                    ret(%2: u64, ) !ir_debug_location !2
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
    assert_eq!(abi.methods[1].inputs.len(), 2);
    // Deploy contract and call contract constructor.
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
}

#[test]
fn test_struct_elem_copy() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"
        module_name = "struct_elem_copy"
        type struct.struct_elem_copy.Config = {key: str, value: str, }
        contract struct_elem_copy {
            state {
            }
            pub fn struct_elem_copy.struct_elem_copy.init()  {
                0:
                    ret()
            }

            pub fn struct_elem_copy.struct_elem_copy.test() -> str {
                1:
                    let %0: %struct.struct_elem_copy.Config* !ir_debug_location !0 = malloc(%struct.struct_elem_copy.Config, )
                    set_field(%0: %struct.struct_elem_copy.Config*, "aaa": str, 0: i32, ) !ir_debug_location !0
                    set_field(%0: %struct.struct_elem_copy.Config*, "bbb": str, 1: i32, ) !ir_debug_location !0
                    let %1: %struct.struct_elem_copy.Config* !ir_debug_location !0 = %0: %struct.struct_elem_copy.Config*
                    let %2: str !ir_debug_location !1 = get_field(%1: %struct.struct_elem_copy.Config*, 0: i32, ) -> str
                    let %3: str !ir_debug_location !2 = get_field(%1: %struct.struct_elem_copy.Config*, 1: i32, ) -> str
                    ret(%2: str, ) !ir_debug_location !3
            }

        }
        meta !0 = !{9: u32, 12: u32, "test.sonar": str, }
        meta !1 = !{13: u32, 13: u32, "test.sonar": str, }
        meta !2 = !{14: u32, 14: u32, "test.sonar": str, }
        meta !3 = !{15: u32, 15: u32, "test.sonar": str, }
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
fn test_point_ty_field_assign() {
    let mut runtime_and_abi = build_mock_runtime(
        r#"
        module_name = "SimpleStorage"
        type struct.SimpleStorage.School = {name: str, }
        type struct.SimpleStorage.Person = {name: str, age: u8, school: %struct.SimpleStorage.School*, }
        contract SimpleStorage {
            state {
            }
            pub fn SimpleStorage.SimpleStorage.init()  {
                0:
                    ret()
            }

            pub fn SimpleStorage.SimpleStorage.test() -> str {
                1:
                    let %0: %struct.SimpleStorage.Person* !ir_debug_location !0 = malloc(%struct.SimpleStorage.Person, )
                    set_field(%0: %struct.SimpleStorage.Person*, "alice": str, 0: i32, ) !ir_debug_location !0
                    set_field(%0: %struct.SimpleStorage.Person*, 1: u8, 1: i32, ) !ir_debug_location !0
                    let %1: %struct.SimpleStorage.School* !ir_debug_location !0 = malloc(%struct.SimpleStorage.School, )
                    set_field(%1: %struct.SimpleStorage.School*, "s1": str, 0: i32, ) !ir_debug_location !0
                    set_field(%0: %struct.SimpleStorage.Person*, %1: %struct.SimpleStorage.School*, 2: i32, ) !ir_debug_location !0
                    let %2: %struct.SimpleStorage.Person* !ir_debug_location !0 = %0: %struct.SimpleStorage.Person*
                    ret(get_field(%2: %struct.SimpleStorage.Person*, 2: i32, 0: i32, ) -> str , ) !ir_debug_location !1
            }

            pub fn SimpleStorage.SimpleStorage.test1() -> str {
                2:
                    let %0: %struct.SimpleStorage.Person* !ir_debug_location !2 = malloc(%struct.SimpleStorage.Person, )
                    set_field(%0: %struct.SimpleStorage.Person*, "alice": str, 0: i32, ) !ir_debug_location !2
                    set_field(%0: %struct.SimpleStorage.Person*, 1: u8, 1: i32, ) !ir_debug_location !2
                    let %1: %struct.SimpleStorage.School* !ir_debug_location !2 = malloc(%struct.SimpleStorage.School, )
                    set_field(%1: %struct.SimpleStorage.School*, "s1": str, 0: i32, ) !ir_debug_location !2
                    set_field(%0: %struct.SimpleStorage.Person*, %1: %struct.SimpleStorage.School*, 2: i32, ) !ir_debug_location !2
                    let %2: %struct.SimpleStorage.Person* !ir_debug_location !2 = %0: %struct.SimpleStorage.Person*
                    let %3: %struct.SimpleStorage.School* !ir_debug_location !3 = malloc(%struct.SimpleStorage.School, )
                    set_field(%3: %struct.SimpleStorage.School*, "s2": str, 0: i32, ) !ir_debug_location !3
                    let %4: %struct.SimpleStorage.School* !ir_debug_location !3 = %3: %struct.SimpleStorage.School*
                    set_field(%2: %struct.SimpleStorage.Person*, %4: %struct.SimpleStorage.School*, 2: i32, ) !ir_debug_location !4
                    ret(get_field(%2: %struct.SimpleStorage.Person*, 2: i32, 0: i32, ) -> str , ) !ir_debug_location !5
            }

        }
        meta !0 = !{13: u32, 19: u32, "test.sonar": str, }
        meta !1 = !{20: u32, 20: u32, "test.sonar": str, }
        meta !2 = !{24: u32, 30: u32, "test.sonar": str, }
        meta !3 = !{31: u32, 33: u32, "test.sonar": str, }
        meta !4 = !{34: u32, 34: u32, "test.sonar": str, }
        meta !5 = !{35: u32, 35: u32, "test.sonar": str, }
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
