// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

mod e2e;
mod examples;

use smart_ir::abi::params::ABIParam;

#[allow(unused)]
const VERSION: u8 = 0;
type Bytes = Vec<u8>;
pub fn encode(params: &[ABIParam], version: u8) -> Bytes {
    let mut bytes = vec![version];
    for param in params {
        bytes.append(&mut param.as_bytes());
    }
    bytes
}
