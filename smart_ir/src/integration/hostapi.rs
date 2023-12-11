// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use num_derive::FromPrimitive;

/// Default Host APIs.
#[derive(FromPrimitive)]
pub enum HostAPI {
    /// Set storage with the keys and values.
    ///
    /// ```no_check
    /// void write_object(const uint8_t** immut_comps,
    ///     uint32_t immut_comps_d1_length,
    ///     const uint32_t* immut_comps_d2_length,
    ///     const uint8_t** mut_comps,
    ///     uint32_t mut_comps_d1_length,
    ///     const uint32_t* mut_comps_d2_length,
    ///     const uint8_t* value,
    ///     uint32_t value_length);
    /// ```
    WriteObject = 1,
    /// Get storage with the keys and values.
    ///
    /// ```no_check
    /// void read_object(const uint8_t** immut_comps,
    ///     uint32_t immut_comps_d1_length,
    ///     const uint32_t* immut_comps_d2_length,
    ///     const uint8_t** mut_comps,
    ///     uint32_t mut_comps_d1_length,
    ///     const uint32_t* mut_comps_d2_length,
    ///     uint8_t* value);
    /// ```
    ReadObject,
    /// Delete storage with the keys.
    ///
    /// ```no_check
    /// void delete_object(const uint8_t** immut_comps,
    ///     uint32_t immut_comps_d1_length,
    ///     const uint32_t* immut_comps_d2_length,
    ///     const uint8_t** mut_comps,
    ///     uint32_t mut_comps_d1_length,
    ///     const uint32_t* mut_comps_d2_length);
    /// ```
    DeleteObject,
    /// Get the storage length.
    ///
    /// ```no_check
    /// int32_t read_object_length(const uint8_t** immut_comps,
    ///     uint32_t immut_comps_d1_length,
    ///     const uint32_t* immut_comps_d2_length,
    ///     const uint8_t** mut_comps,
    ///     uint32_t mut_comps_d1_length,
    ///     const uint32_t* mut_comps_d2_length,
    /// );  
    /// ```
    ReadObjectLength,
    /// Get call.sender u8 ptr.
    ///
    /// ```no_check
    /// void get_tx_sender(uint8_t* data);
    /// ```
    GetCallSender,
    /// Get call.sender length.
    ///
    /// ```no_check
    /// int32_t get_tx_sender_length();
    /// ```
    GetCallSenderLength,
    /// Get call.this_contract u8 ptr.
    ///
    /// ```no_check
    /// void get_call_contract(uint8_t* data);
    /// ```
    GetCallContract,
    /// Get call.this_contract length.
    ///
    /// ```no_check
    /// int32_t get_call_contract_length();
    /// ```
    GetCallContractLength,
    /// Get call gas left
    ///
    /// ```no_check
    /// uint64_t get_call_gas_left();
    /// ```
    GetCallGasLeft,
    /// Get call gas limit
    ///
    /// ```no_check
    /// uint64_t get_call_gas_limit();
    /// ```
    GetCallGasLimit,
    /// Get call.op_contract u8 ptr.
    ///
    /// ```no_check
    /// void get_op_contract(uint8_t* data);
    /// ```
    GetOpContract,
    /// Get call.op_contract length.
    ///
    /// ```no_check
    /// int32_t get_op_contract_length();
    /// ```
    GetOpContractLength,
    /// Get contract method parameter pointer.
    ///
    /// ```no_check
    /// void get_call_argpack(uint8_t* args);
    /// ```
    GetCallArgPack,
    /// Get contract method parameter length.
    ///
    /// ```no_check
    /// uint32_t get_call_argpack_length();
    /// ```
    GetCallArgPackLength,
    /// Set contract method call return result.
    ///
    /// ```no_check
    /// void set_call_result(const uint8_t* data, uint32_t length);
    /// ```
    SetCallResult,
    /// Get the current block number.
    ///
    /// ```no_check
    /// uint64_t get_block_number();
    /// ```
    GetBlockNumber,
    /// Get the current block timestamp.
    ///
    /// ```no_check
    /// uint64_t get_block_timestamp();
    /// ```
    GetBlockTimestamp,
    /// Get random seed of block, need 32 bytes buffer
    ///
    /// ```no_check
    /// void get_block_random_seed(uint8_t* data);
    /// ```
    GetBlockRandomSeed,
    /// Get the transaction timestamp.
    ///
    /// ```no_check
    /// uint64_t get_tx_timestamp();
    /// ```
    GetTxTimestamp,
    /// Get the transaction nonce.
    ///
    /// ```no_check
    /// uint64_t get_tx_nonce();
    /// ```
    GetTxNonce,
    /// Get the transaction index in block.
    ///
    /// ```no_check
    /// uint32_t get_tx_index();
    /// ```
    GetTxIndex,
    /// Get tx.hash u8 ptr.
    ///
    /// ```no_check
    /// void get_tx_hash(char *hash32);
    /// ```
    GetTxHash,
    /// Get tx.hash length.
    ///
    /// ```no_check
    /// int32_t get_tx_hash_length();
    /// ```
    GetTxHashLength,
    /// Get tx.sender u8 ptr.
    ///
    /// ```no_check
    /// void get_call_sender(uint8_t* data);
    /// ```
    GetTxSender,
    /// Get tx.sender length.
    ///
    /// ```no_check
    /// int32_t get_call_sender_length();
    /// ```
    GetTxSenderLength,
    /// Get tx gas limit
    ///
    /// ```no_check
    /// uint64_t get_tx_gas_limit();
    /// ```
    GetTxGasLimit,
    /// Contract abort with messages.
    ///
    /// ```no_check
    /// void abort(const uint8_t *msg, uint32_t msg_length);
    /// ```
    Abort,
    /// Contract debug print API.
    ///
    /// ```no_check
    /// void println(const uint8_t* data, uint32_t length);
    /// ```
    Println,
    /// Contract log messages.
    ///
    /// ```no_check
    /// void log(const char** topics,
    ///     uint32_t topics_d1_length,
    ///     const uint32_t* topics_length,
    ///     const char* desc,
    ///     uint32_t desc_length);
    /// ``
    Log,
    /// HASH: SHA256 algorithm
    ///
    /// ```no_check
    /// void sha256(const char* msg,
    ///     uint32_t msg_length,
    ///     char* value);
    /// ``
    SHA256,
    /// HASH: SM3 algorithm
    ///
    /// ```no_check
    /// void sm3(const char* msg,
    ///     uint32_t msg_length,
    ///     char* value);
    /// ``
    SM3,
    /// HASH: keccak256 algorithm
    ///
    /// ```no_check
    /// void keccak256(const char* msg,
    ///     uint32)t msg_lenght,
    ///     char* value);
    /// ```
    KECCAK256,
    /// Recovery eth address from sign and msg
    ///
    /// ```no_check
    /// uint32_t eth_secp256k1_recovery(const char* hash,
    ///     const char* v,
    ///     const char* r,
    ///     const char* s,
    ///     char* addr);
    /// ```
    EthSecp256k1Recovery,
    /// Initiate a contract call.
    ///
    /// ```no_check
    /// int32_t co_call(const char *contract,
    ///     uint32_t contract_length,
    ///     const char *method,
    ///     uint32_t method_length,
    ///     const char *argpack,
    ///     uint32_t argpack_length);
    /// ```
    CoCall,
    /// Get the return value of the contract call.
    ///
    /// ```no_check
    /// void get_call_result(char *result);
    /// ```
    GetCallResult,
    /// Get the length of the return value of the contract call
    ///
    /// ```no_check
    /// int32_t get_call_result_length();
    /// ```
    GetCallResultLength,
    /// Revert the call(not the whole transaction)
    /// when child contract revert, execution return to the parent contract call with error code
    /// ```no_check
    /// void revert(int32_t error_code, const char* error_msg, uint32_t error_msg_len);
    /// ```
    Revert,
}

impl HostAPI {
    pub fn name(&self) -> &'static str {
        match self {
            HostAPI::WriteObject => "write_object",
            HostAPI::ReadObject => "read_object",
            HostAPI::DeleteObject => "delete_object",
            HostAPI::ReadObjectLength => "read_object_length",
            HostAPI::GetCallSender => "get_call_sender",
            HostAPI::GetCallSenderLength => "get_call_sender_length",
            HostAPI::GetCallContract => "get_call_contract",
            HostAPI::GetCallContractLength => "get_call_contract_length",
            HostAPI::GetCallGasLeft => "get_call_gas_left",
            HostAPI::GetCallGasLimit => "get_call_gas_limit",
            HostAPI::GetOpContract => "get_op_contract",
            HostAPI::GetOpContractLength => "get_op_contract_length",
            HostAPI::GetCallArgPack => "get_call_argpack",
            HostAPI::GetCallArgPackLength => "get_call_argpack_length",
            HostAPI::SetCallResult => "set_call_result",
            HostAPI::GetBlockNumber => "get_block_number",
            HostAPI::GetBlockTimestamp => "get_block_timestamp",
            HostAPI::GetBlockRandomSeed => "get_block_random_seed",
            HostAPI::GetTxTimestamp => "get_tx_timestamp",
            HostAPI::GetTxNonce => "get_tx_nonce",
            HostAPI::GetTxIndex => "get_tx_index",
            HostAPI::GetTxHash => "get_tx_hash",
            HostAPI::GetTxHashLength => "get_tx_hash_length",
            HostAPI::GetTxSender => "get_tx_sender",
            HostAPI::GetTxSenderLength => "get_tx_sender_length",
            HostAPI::GetTxGasLimit => "get_tx_gas_limit",
            HostAPI::Abort => "abort",
            HostAPI::Println => "println",
            HostAPI::Log => "log",
            HostAPI::SHA256 => "sha256",
            HostAPI::SM3 => "sm3",
            HostAPI::KECCAK256 => "keccak256",
            HostAPI::EthSecp256k1Recovery => "eth_secp256k1_recovery",
            HostAPI::CoCall => "co_call",
            HostAPI::GetCallResult => "get_call_result",
            HostAPI::GetCallResultLength => "get_call_result_length",
            HostAPI::Revert => "revert",
        }
    }

    /// Get all host api names
    pub fn all_names() -> Vec<String> {
        vec![
            HostAPI::WriteObject.name().to_string(),
            HostAPI::ReadObject.name().to_string(),
            HostAPI::DeleteObject.name().to_string(),
            HostAPI::ReadObjectLength.name().to_string(),
            HostAPI::GetCallSender.name().to_string(),
            HostAPI::GetCallSenderLength.name().to_string(),
            HostAPI::GetCallContract.name().to_string(),
            HostAPI::GetCallContractLength.name().to_string(),
            HostAPI::GetCallGasLeft.name().to_string(),
            HostAPI::GetCallGasLimit.name().to_string(),
            HostAPI::GetOpContract.name().to_string(),
            HostAPI::GetOpContractLength.name().to_string(),
            HostAPI::GetCallArgPack.name().to_string(),
            HostAPI::GetCallArgPackLength.name().to_string(),
            HostAPI::SetCallResult.name().to_string(),
            HostAPI::GetBlockNumber.name().to_string(),
            HostAPI::GetBlockTimestamp.name().to_string(),
            HostAPI::GetBlockRandomSeed.name().to_string(),
            HostAPI::GetTxTimestamp.name().to_string(),
            HostAPI::GetTxNonce.name().to_string(),
            HostAPI::GetTxIndex.name().to_string(),
            HostAPI::GetTxHash.name().to_string(),
            HostAPI::GetTxHashLength.name().to_string(),
            HostAPI::GetTxSender.name().to_string(),
            HostAPI::GetTxSenderLength.name().to_string(),
            HostAPI::GetTxGasLimit.name().to_string(),
            HostAPI::Abort.name().to_string(),
            HostAPI::Println.name().to_string(),
            HostAPI::Log.name().to_string(),
            HostAPI::SHA256.name().to_string(),
            HostAPI::SM3.name().to_string(),
            HostAPI::KECCAK256.name().to_string(),
            HostAPI::EthSecp256k1Recovery.name().to_string(),
            HostAPI::CoCall.name().to_string(),
            HostAPI::GetCallResult.name().to_string(),
            HostAPI::GetCallResultLength.name().to_string(),
            HostAPI::Revert.name().to_string(),
        ]
    }
}
