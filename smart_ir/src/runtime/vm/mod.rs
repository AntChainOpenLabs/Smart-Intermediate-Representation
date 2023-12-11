// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::Rng;
use rsa::{Hash, PaddingScheme, PublicKey, RSAPublicKey};
use std::iter::repeat;
use wasmi::memory_units::Pages;
use wasmi::{MemoryInstance, MemoryRef};

// Address abstraction.
pub type Address = String;

pub const MAX_ADDRESS_LENGTH: usize = 28;

/// VirtualMachine is mock contract WASM code with input contract method inputs and outputs
/// for the mock test.
#[derive(Debug, Clone)]
pub struct VirtualMachine {
    /// Contract address
    pub addr: Address,
    pub op_addr: Address,
    pub memory: MemoryRef,
    pub code: Vec<u8>,
    pub input: Vec<u8>,
    pub output: Vec<u8>,
    pub context: VMContext,
}

#[derive(Debug, Clone)]
pub struct VMContext {
    pub block_number: u64,
    pub block_timestamp: u64,
    pub timestamp: u64,
    pub nonce: u64,
    pub index: u32,
    pub hash: String,
    pub call_gas_left: u64,
    pub call_gas_limit: u64,
    pub tx_gas_limit: u64,
    pub block_random_seed: [u8; 32],
}

impl VirtualMachine {
    pub fn new(code: Vec<u8>, addr: Address) -> Self {
        let mut rng = rand::thread_rng();
        let block_number: u64 = rng.gen::<u32>() as u64;
        let block_random_seed = sha256(&block_number.to_string());
        VirtualMachine {
            memory: MemoryInstance::alloc(Pages(2), Some(Pages(2))).unwrap(),
            input: vec![],
            output: vec![],
            context: VMContext {
                block_number,
                block_timestamp: rng.gen::<u32>() as u64,
                timestamp: rng.gen::<u32>() as u64,
                nonce: rng.gen::<u32>() as u64,
                index: rng.gen::<u16>() as u32,
                hash: rand_hash(),
                call_gas_left: rng.gen(),
                call_gas_limit: rng.gen(),
                tx_gas_limit: rng.gen(),
                block_random_seed,
            },
            code,
            addr,
            op_addr: Address::from("opration address"),
        }
    }
}

/// New a random address
pub fn rand_address() -> Address {
    rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(MAX_ADDRESS_LENGTH)
        .collect()
}

/// New a random hash
fn rand_hash() -> String {
    rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(32)
        .collect()
}

/// sha256
pub fn sha256(msg: &str) -> [u8; 32] {
    // create a Sha256 object
    let mut hasher = Sha256::new();
    // write input message
    hasher.input_str(msg);
    // Save the hash digest to buf
    let mut buf: Vec<u8> = repeat(0).take((hasher.output_bits() + 7) / 8).collect();
    hasher.result(&mut buf);
    buf.to_vec().try_into().unwrap()
}

/// verify the signature using rsa algorithm
pub fn rsa_verify_sign(pub_k: Vec<u8>, sig: Vec<u8>, digest: Vec<u8>) -> bool {
    let pk = RSAPublicKey::from_pkcs1(&pub_k).expect("failed to parse key");
    pk.verify(
        PaddingScheme::PKCS1v15Sign {
            hash: Option::from(Hash::SHA2_256),
        },
        &digest,
        &sig,
    )
    .is_ok()
}
