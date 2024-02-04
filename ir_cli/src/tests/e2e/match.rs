use smart_ir::abi::params::ABIParam;

use crate::vm::build_mock_runtime;

#[test]
fn test_match() {
    let runtime_and_abi = build_mock_runtime(
        r#"module_name = "Match"
        contract Match {
            state {
            }
            pub fn Match.Match.init()  {
                0:
                    ret()
            }
            pub fn Match.Match.match(%0: i32, ) -> u8 {
                1:
                    let %1: u8 = 0: u8
                    match(%0: i32, bb 2, 1: i32, bb 3, 2: i32, bb 4, )
                2:
                    %1 = 3: u8
                    br(bb 5, )
                3:
                    %1 = 1: u8
                    br(bb 5, )
                4:
                    %1 = 2: u8
                    br(bb 5, )
                5:
                    ret (%1: u8)
            }
        
        }
        "#,
    );
    let mut runtime = runtime_and_abi.0;
    runtime.constructor(hex::decode("00").unwrap().as_slice());
    // Contract method call.
    let mut bytes = vec![0_u8];
    bytes.append(&mut ABIParam::I32(1).as_bytes());
    assert_eq!(runtime.call("match", &bytes), [0, 1]);

    let mut bytes = vec![0_u8];
    bytes.append(&mut ABIParam::I32(2).as_bytes());
    assert_eq!(runtime.call("match", &bytes), [0, 2]);

    let mut bytes = vec![0_u8];
    bytes.append(&mut ABIParam::I32(3).as_bytes());
    assert_eq!(runtime.call("match", &bytes), [0,3]);
}
