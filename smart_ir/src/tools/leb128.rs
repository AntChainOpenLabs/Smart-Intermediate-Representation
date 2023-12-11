// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use nano_leb128::LEB128DecodeError;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

pub fn leb128_encode(v: &BigInt) -> Vec<u8> {
    let v_encode = nano_leb128::SLEB128::from(v.to_i64().unwrap());
    let mut buf = [0; 8];
    match v_encode.write_into(&mut buf) {
        Ok(len) => buf.to_vec()[..len].to_vec(),
        Err(e) => unreachable!("{:?}", e),
    }
}

pub fn leb128_decode(input: &[u8]) -> Result<BigInt, LEB128DecodeError> {
    match nano_leb128::SLEB128::read_from(input) {
        Ok((leb128_value, _)) => Ok(BigInt::from(i64::from(leb128_value))),
        Err(e) => Err(e),
    }
}
