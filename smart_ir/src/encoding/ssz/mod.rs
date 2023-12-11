// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

pub trait SSZDecode: Sized {
    fn from_ssz_bytes(bytes: &[u8]) -> Self;
}

pub trait SSZEncode {
    fn as_ssz_bytes(v: &Self) -> Vec<u8>;
}
impl SSZDecode for u128 {
    fn from_ssz_bytes(bytes: &[u8]) -> u128 {
        let mut array: [u8; 16] = std::default::Default::default();
        array.clone_from_slice(bytes);
        u128::from_le_bytes(array)
    }
}

impl SSZEncode for u128 {
    fn as_ssz_bytes(v: &u128) -> Vec<u8> {
        let mut array = vec![];
        for byte in v.to_le_bytes() {
            array.push(byte);
        }
        array
    }
}
