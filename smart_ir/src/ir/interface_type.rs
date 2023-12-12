// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::builder::MetaDataId;
use crate::ir_codegen::common::global::{get_extend_context, has_extend_context};

pub trait PartialFuncNameBehavior {
    fn apply_name(&self) -> String;
}

impl PartialFuncNameBehavior for () {
    fn apply_name(&self) -> String {
        unreachable!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PartialFuncNameKind<T: PartialFuncNameBehavior, U: PartialFuncNameBehavior> {
    UserDefFunc(String),
    Intrinsic(T),
    HostAPI(U),
    Otherwise,
}

/// Specify the specification of HostAPI here
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DefaultHostAPI {}

/// Specify the specification of Intrinsic functions here
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum IntrinsicFuncName {
    // Vector
    IR_VECTOR_SET,
    IR_VECTOR_GET,
    IR_VECTOR_CREATE_ITER,
    IR_VECTOR_GET_NEXT,
    IR_VECTOR_OBJ_KEY,
    IR_VECTOR_OBJ_VALUE,
    IR_VECTOR_PUSH,
    IR_VECTOR_POP,
    IR_VECTOR_INSERT,
    IR_VECTOR_DELETE,
    IR_VECTOR_LEN,
    IR_VECTOR_CLEAR,
    IR_VECTOR_REVERSE,
    IR_VECTOR_AT,
    IR_VECTOR_SLICE,
    IR_VECTOR_TO_STR,

    // Map
    IR_MAP_SET,
    IR_MAP_GET,
    IR_MAP_CREATE_ITER,
    IR_MAP_GET_NEXT,
    IR_MAP_OBJ_KEY,
    IR_MAP_OBJ_VALUE,
    IR_MAP_CONTAINS_KEY,
    IR_MAP_INSERT,
    IR_MAP_DELETE,
    IR_MAP_LEN,
    IR_MAP_CLEAR,

    // Storage
    IR_STORAGE_PUSH,
    IR_STORAGE_PUSH_EMPTY,
    IR_STORAGE_MULTIARRAY_PUSH_EMPTY,
    IR_STORAGE_POP,
    IR_STORAGE_LEN,
    IR_STORAGE_VERIFY_INDEX,
    IR_STORAGE_CONTAINS_KEY,
    IR_STORAGE_CONTAINS_ASSET,
    IR_STORAGE_DELETE,
    IR_STORAGE_MINT,
    IR_STORAGE_BURN,
    IR_STORAGE_DESTROY,
    IR_STORAGE_GET_BALANCE,
    IR_STORAGE_GET_TAG,
    IR_STORAGE_TRANSFER,
    IR_STORAGE_SET_BSS,
    IR_STORAGE_GET_BSS,
    IR_STORAGE_PATH_JOIN,

    // Builtin function
    IR_BUILTIN_ABORT,
    IR_BUILTIN_ASSERT,
    IR_BUILTIN_REQUIRE,
    IR_BUILTIN_PRINT,
    IR_BUILTIN_PRINT_TYPE,
    IR_BUILTIN_ADDRESS,
    IR_BUILTIN_ENCODE_BASE64,
    IR_BUILTIN_DECODE_BASE64,
    IR_BUILTIN_ENCODE_HEX,
    IR_BUILTIN_DECODE_HEX,
    IR_BUILTIN_ENCODE_PARAMS,
    IR_BUILTIN_CALL_LOG,
    IR_BUILTIN_SHA256,
    IR_BUILTIN_SM3,
    IR_BUILTIN_KECCAK256,
    IR_BUILTIN_VERIFY_MYCRYPTO_SIGNATURE,
    IR_BUILTIN_ETH_SECP256K1_RECOVERY,
    IR_BUILTIN_GET_ACCOUT,
    IR_BUILTIN_GET_CONTRACT,
    IR_BUILTIN_GET_ARTIFACT,
    IR_BUILTIN_ADD_COVERAGE_COUNTER,
    IR_BUILTIN_COCALL, // co_call by let a: Contract = Contract("xxx"); a.fib(...)
    IR_BUILTIN_CO_CALL_DIRECTLY, // co_call directly by let err: i32 = co_call("xxx","fib",...)
    IR_BUILTIN_REVERT,
    IR_BUILTIN_GET_CALL_RESULT,

    // Builtin variable
    IR_BUILTIN_BLOCK_NUMBER,
    IR_BUILTIN_BLOCK_TIMESTAMP,
    IR_BUILTIN_BLOCK_RANDOM_SEED,
    IR_BUILTIN_BLOCK_VERSION,
    IR_BUILTIN_BLOCK_FLAGS,

    IR_BUILTIN_TX_SENDER,
    IR_BUILTIN_TX_HASH,
    IR_BUILTIN_TX_INDEX,
    IR_BUILTIN_TX_GAS_LIMIT,
    IR_BUILTIN_TX_TIMESTAMP,
    IR_BUILTIN_TX_NONCE,

    IR_BUILTIN_CALL_SENDER,
    IR_BUILTIN_CALL_THIS_CONTRACT,
    IR_BUILTIN_CALL_OP_CONTRACT,
    IR_BUILTIN_CALL_GAS_LEFT,
    IR_BUILTIN_CALL_GAS_LIMIT,

    // STR
    IR_STR_SPLIT,
    IR_STR_LEN,
    IR_STR_LOWER,
    IR_STR_UPPER,
    IR_STR_AT,
    IR_STR_COUNT,
    IR_STR_STARTSWITH,
    IR_STR_ENDSWITH,
    IR_STR_ISALNUM,
    IR_STR_ISALPHA,
    IR_STR_ISDIGIT,
    IR_STR_ISLOWER,
    IR_STR_ISUPPER,
    IR_STR_ISSPACE,
    IR_STR_STRIP,
    IR_STR_LSTRIP,
    IR_STR_RSTRIP,
    IR_STR_JOIN,
    IR_STR_CONCAT,
    IR_STR_REPLACE,
    IR_STR_FIND,
    IR_STR_SUBSTR,
    IR_STR_INSERT,
    IR_STR_TO_BYTES,
    IR_STR_TO_I128,
    IR_STR_TO_U128,

    // builtin module func

    // Base64
    IR_BASE64_ENCODE,
    IR_BASE64_DECODE,

    // Hex
    IR_HEX_ENCODE,
    IR_HEX_DECODE,

    // SSZ
    IR_SSZ_ENCODE,
    IR_SSZ_DECODE,

    // JSON
    IR_JSON_ENCODE,
    IR_JSON_DECODE,

    // rlp
    IR_RLP_ENCODE,
    IR_RLP_DECODE,

    // Datastream
    IR_DATASTREAM_ENCODE_BOOL,
    IR_DATASTREAM_ENCODE_U8,
    IR_DATASTREAM_ENCODE_U16,
    IR_DATASTREAM_ENCODE_U32,
    IR_DATASTREAM_ENCODE_U64,
    IR_DATASTREAM_ENCODE_U128,
    IR_DATASTREAM_ENCODE_I8,
    IR_DATASTREAM_ENCODE_I16,
    IR_DATASTREAM_ENCODE_I32,
    IR_DATASTREAM_ENCODE_I64,
    IR_DATASTREAM_ENCODE_I128,
    IR_DATASTREAM_ENCODE_STR,
    IR_DATASTREAM_ENCODE_BOOLARRAY,
    IR_DATASTREAM_ENCODE_U8ARRAY,
    IR_DATASTREAM_ENCODE_U16ARRAY,
    IR_DATASTREAM_ENCODE_U32ARRAY,
    IR_DATASTREAM_ENCODE_U64ARRAY,
    IR_DATASTREAM_ENCODE_U128ARRAY,
    IR_DATASTREAM_ENCODE_I8ARRAY,
    IR_DATASTREAM_ENCODE_I16ARRAY,
    IR_DATASTREAM_ENCODE_I32ARRAY,
    IR_DATASTREAM_ENCODE_I64ARRAY,
    IR_DATASTREAM_ENCODE_I128ARRAY,
    IR_DATASTREAM_ENCODE_STRARRAY,
    IR_DATASTREAM_ENCODE_STRBOOLMAP,
    IR_DATASTREAM_ENCODE_STRU8MAP,
    IR_DATASTREAM_ENCODE_STRU16MAP,
    IR_DATASTREAM_ENCODE_STRU32MAP,
    IR_DATASTREAM_ENCODE_STRU64MAP,
    IR_DATASTREAM_ENCODE_STRU128MAP,
    IR_DATASTREAM_ENCODE_STRI8MAP,
    IR_DATASTREAM_ENCODE_STRI16MAP,
    IR_DATASTREAM_ENCODE_STRI32MAP,
    IR_DATASTREAM_ENCODE_STRI64MAP,
    IR_DATASTREAM_ENCODE_STRI128MAP,
    IR_DATASTREAM_ENCODE_STRSTRMAP,
    IR_DATASTREAM_DECODE,

    // Math
    IR_MATH_POW,
    IR_MATH_ITOA,

    // Twisted Elgamal
    IR_TWISTED_ELGAMAL_ADD,
    IR_TWISTED_ELGAMAL_SUB,
    IR_TWISTED_ELGAMAL_SCALAR_MULTIPLY,
    IR_TWISTED_ELGAMAL_VERIFY_PUBLICKEY,
    IR_TWISTED_ELGAMAL_VERIFY_CIPHER,
    IR_TWISTED_ELGAMAL_VERIFY_EQUAL_AMOUNT,
    IR_TWISTED_ELGAMAL_VERIFY_ZERO_AMOUNT,
    IR_TWISTED_ELGAMAL_VERIFY_AMOUNT_RANGE,
    IR_TWISTED_ELGAMAL_VERIFY_AMOUNT_PROPORTION,
    IR_TWISTED_ELGAMAL_VERIFY_AMOUNT_GREATER_EQUAL,

    // Rand
    IR_RAND_GEN_U64,
    IR_RAND_GEN_U64_FROM_SEED,
    IR_RAND_GEN_U64_ARRAY,
    IR_RAND_GEN_U64_ARRAY_FROM_SEED,
}

/// Default name of Intrinsic functions
/// OVERRIDE `apply_name` to concrete exact function name in later codegen stage
impl PartialFuncNameBehavior for IntrinsicFuncName {
    fn apply_name(&self) -> String {
        match self {
            IntrinsicFuncName::IR_VECTOR_SET => "ir.vector.set".to_string(),
            IntrinsicFuncName::IR_VECTOR_GET => "ir.vector.get".to_string(),
            IntrinsicFuncName::IR_VECTOR_CREATE_ITER => "ir.vector.create_iter".to_string(),
            IntrinsicFuncName::IR_VECTOR_GET_NEXT => "ir.vector.get_next".to_string(),
            IntrinsicFuncName::IR_VECTOR_OBJ_KEY => "ir.vector.obj_key".to_string(),
            IntrinsicFuncName::IR_VECTOR_OBJ_VALUE => "ir.vector.obj_value".to_string(),
            IntrinsicFuncName::IR_VECTOR_PUSH => "ir.vector.push".to_string(),
            IntrinsicFuncName::IR_VECTOR_POP => "ir.vector.pop".to_string(),
            IntrinsicFuncName::IR_VECTOR_INSERT => "ir.vector.insert".to_string(),
            IntrinsicFuncName::IR_VECTOR_DELETE => "ir.vector.delete".to_string(),
            IntrinsicFuncName::IR_VECTOR_LEN => "ir.vector.len".to_string(),
            IntrinsicFuncName::IR_VECTOR_CLEAR => "ir.vector.clear".to_string(),
            IntrinsicFuncName::IR_VECTOR_REVERSE => "ir.vector.reverse".to_string(),
            IntrinsicFuncName::IR_VECTOR_AT => "ir.vector.at".to_string(),
            IntrinsicFuncName::IR_VECTOR_SLICE => "ir.vector.slice".to_string(),
            IntrinsicFuncName::IR_VECTOR_TO_STR => "ir.vector.to_str".to_string(),

            IntrinsicFuncName::IR_MAP_CONTAINS_KEY => "ir.map.contains_key".to_string(),
            IntrinsicFuncName::IR_STORAGE_CONTAINS_ASSET => "ir.map.contains_asset".to_string(),
            IntrinsicFuncName::IR_MAP_INSERT => "ir.map.insert".to_string(),
            IntrinsicFuncName::IR_MAP_DELETE => "ir.map.delete".to_string(),
            IntrinsicFuncName::IR_MAP_LEN => "ir.map.len".to_string(),
            IntrinsicFuncName::IR_MAP_CLEAR => "ir.map.clear".to_string(),
            IntrinsicFuncName::IR_MAP_SET => "ir.map.set".to_string(),
            IntrinsicFuncName::IR_MAP_GET => "ir.map.get".to_string(),
            IntrinsicFuncName::IR_MAP_CREATE_ITER => "ir.map.create_iter".to_string(),
            IntrinsicFuncName::IR_MAP_GET_NEXT => "ir.map.get_next".to_string(),
            IntrinsicFuncName::IR_MAP_OBJ_KEY => "ir.map.obj_key".to_string(),
            IntrinsicFuncName::IR_MAP_OBJ_VALUE => "ir.map.obj_value".to_string(),

            IntrinsicFuncName::IR_STORAGE_PUSH => "ir.storage.push".to_string(),
            IntrinsicFuncName::IR_STORAGE_PUSH_EMPTY => "ir.storage.push_empty".to_string(),
            IntrinsicFuncName::IR_STORAGE_MULTIARRAY_PUSH_EMPTY => {
                "ir.storage.multiarray_push_empty".to_string()
            }
            IntrinsicFuncName::IR_STORAGE_POP => "ir.storage.pop".to_string(),
            IntrinsicFuncName::IR_STORAGE_LEN => "ir.storage.len".to_string(),
            IntrinsicFuncName::IR_STORAGE_VERIFY_INDEX => "ir.storage.verify_index".to_string(),
            IntrinsicFuncName::IR_STORAGE_CONTAINS_KEY => "ir.storage.contains_key".to_string(),
            IntrinsicFuncName::IR_STORAGE_DELETE => "ir.storage.delete".to_string(),
            IntrinsicFuncName::IR_STORAGE_MINT => "ir.storage.mint".to_string(),
            IntrinsicFuncName::IR_STORAGE_BURN => "ir.storage.burn".to_string(),
            IntrinsicFuncName::IR_STORAGE_DESTROY => "ir.storage.destroy".to_string(),
            IntrinsicFuncName::IR_STORAGE_GET_BALANCE => "ir.storage.get_balance".to_string(),
            IntrinsicFuncName::IR_STORAGE_GET_TAG => "ir.storage.get_tag".to_string(),
            IntrinsicFuncName::IR_STORAGE_TRANSFER => "ir.storage.transfer".to_string(),
            IntrinsicFuncName::IR_STORAGE_SET_BSS => "ir.storage.set_bss".to_string(),
            IntrinsicFuncName::IR_STORAGE_GET_BSS => "ir.storage.get_bss".to_string(),
            IntrinsicFuncName::IR_STORAGE_PATH_JOIN => "ir.storage.path_join".to_string(),

            IntrinsicFuncName::IR_BUILTIN_ABORT => "ir.builtin.abort".to_string(),
            IntrinsicFuncName::IR_BUILTIN_ASSERT => "ir.builtin.assert".to_string(),
            IntrinsicFuncName::IR_BUILTIN_REQUIRE => "ir.builtin.require".to_string(),
            IntrinsicFuncName::IR_BUILTIN_PRINT => "ir.builtin.print".to_string(),
            IntrinsicFuncName::IR_BUILTIN_PRINT_TYPE => "ir.builtin.print_type".to_string(),
            IntrinsicFuncName::IR_BUILTIN_ADDRESS => "ir.builtin.address".to_string(),
            IntrinsicFuncName::IR_BUILTIN_ENCODE_BASE64 => "ir.builtin.encode_base64".to_string(),
            IntrinsicFuncName::IR_BUILTIN_DECODE_BASE64 => "ir.builtin.decode_base64".to_string(),
            IntrinsicFuncName::IR_BUILTIN_ENCODE_HEX => "ir.builtin.encode_hex".to_string(),
            IntrinsicFuncName::IR_BUILTIN_DECODE_HEX => "ir.builtin.decode_hex".to_string(),
            IntrinsicFuncName::IR_BUILTIN_ENCODE_PARAMS => "ir.builtin.encode_params".to_string(),
            IntrinsicFuncName::IR_BUILTIN_CALL_LOG => "ir.builtin.call_log".to_string(),
            IntrinsicFuncName::IR_BUILTIN_GET_CALL_RESULT => {
                "ir.builtin.get_call_result".to_string()
            }
            IntrinsicFuncName::IR_BUILTIN_SHA256 => "ir.builtin.sha256".to_string(),
            IntrinsicFuncName::IR_BUILTIN_SM3 => "ir.builtin.sm3".to_string(),
            IntrinsicFuncName::IR_BUILTIN_KECCAK256 => "ir.builtin.keccak256".to_string(),
            IntrinsicFuncName::IR_BUILTIN_VERIFY_MYCRYPTO_SIGNATURE => {
                "ir.builtin.verify_mycrypto_signature".to_string()
            }
            IntrinsicFuncName::IR_BUILTIN_ETH_SECP256K1_RECOVERY => {
                "ir.builtin.eth_secp256k1_recovery".to_string()
            }
            IntrinsicFuncName::IR_BUILTIN_REVERT => "ir.builtin.revert".to_string(),
            IntrinsicFuncName::IR_BUILTIN_GET_ACCOUT => "ir.builtin.get_account".to_string(),
            IntrinsicFuncName::IR_BUILTIN_GET_CONTRACT => "ir.builtin.get_contract".to_string(),
            IntrinsicFuncName::IR_BUILTIN_GET_ARTIFACT => "ir.builtin.get_artifact".to_string(),
            IntrinsicFuncName::IR_BUILTIN_ADD_COVERAGE_COUNTER => {
                "ir.builtin.add_coverage_counter".to_string()
            }
            IntrinsicFuncName::IR_BUILTIN_COCALL => "ir.builtin.cocall".to_string(),
            IntrinsicFuncName::IR_BUILTIN_CO_CALL_DIRECTLY => {
                "ir.builtin.co_call_directly".to_string()
            }
            IntrinsicFuncName::IR_BUILTIN_BLOCK_NUMBER => "ir.builtin.block_number".to_string(),
            IntrinsicFuncName::IR_BUILTIN_BLOCK_TIMESTAMP => {
                "ir.builtin.block_timestamp".to_string()
            }
            IntrinsicFuncName::IR_BUILTIN_BLOCK_RANDOM_SEED => {
                "ir.builtin.block_random_seed".to_string()
            }
            IntrinsicFuncName::IR_BUILTIN_BLOCK_VERSION => "ir.builtin.block_version".to_string(),
            IntrinsicFuncName::IR_BUILTIN_TX_SENDER => "ir.builtin.tx_sender".to_string(),
            IntrinsicFuncName::IR_BUILTIN_TX_HASH => "ir.builtin.tx_hash".to_string(),
            IntrinsicFuncName::IR_BUILTIN_TX_INDEX => "ir.builtin.tx_index".to_string(),
            IntrinsicFuncName::IR_BUILTIN_TX_GAS_LIMIT => "ir.builtin.tx_gas_limit".to_string(),
            IntrinsicFuncName::IR_BUILTIN_TX_TIMESTAMP => "ir.builtin.tx_timestamp".to_string(),
            IntrinsicFuncName::IR_BUILTIN_TX_NONCE => "ir.builtin.tx_nonce".to_string(),
            IntrinsicFuncName::IR_BUILTIN_CALL_SENDER => "ir.builtin.call_sender".to_string(),
            IntrinsicFuncName::IR_BUILTIN_CALL_THIS_CONTRACT => {
                "ir.builtin.call_this_contract".to_string()
            }
            IntrinsicFuncName::IR_BUILTIN_CALL_OP_CONTRACT => {
                "ir.builtin.call_op_contract".to_string()
            }
            IntrinsicFuncName::IR_BUILTIN_CALL_GAS_LIMIT => "ir.builtin.call_gas_limit".to_string(),
            IntrinsicFuncName::IR_BUILTIN_CALL_GAS_LEFT => "ir.builtin.call_gas_left".to_string(),

            IntrinsicFuncName::IR_STR_SPLIT => "ir.str.split".to_string(),
            IntrinsicFuncName::IR_STR_LEN => "ir.str.len".to_string(),
            IntrinsicFuncName::IR_STR_LOWER => "ir.str.lower".to_string(),
            IntrinsicFuncName::IR_STR_UPPER => "ir.str.upper".to_string(),
            IntrinsicFuncName::IR_STR_AT => "ir.str.at".to_string(),
            IntrinsicFuncName::IR_STR_COUNT => "ir.str.count".to_string(),
            IntrinsicFuncName::IR_STR_STARTSWITH => "ir.str.startswith".to_string(),
            IntrinsicFuncName::IR_STR_ENDSWITH => "ir.str.endswith".to_string(),
            IntrinsicFuncName::IR_STR_ISALNUM => "ir.str.isalnum".to_string(),
            IntrinsicFuncName::IR_STR_ISALPHA => "ir.str.isalpha".to_string(),
            IntrinsicFuncName::IR_STR_ISDIGIT => "ir.str.isdigit".to_string(),
            IntrinsicFuncName::IR_STR_ISLOWER => "ir.str.islower".to_string(),
            IntrinsicFuncName::IR_STR_ISUPPER => "ir.str.isupper".to_string(),
            IntrinsicFuncName::IR_STR_ISSPACE => "ir.str.isspace".to_string(),
            IntrinsicFuncName::IR_STR_STRIP => "ir.str.strip".to_string(),
            IntrinsicFuncName::IR_STR_LSTRIP => "ir.str.lstrip".to_string(),
            IntrinsicFuncName::IR_STR_RSTRIP => "ir.str.rstrip".to_string(),
            IntrinsicFuncName::IR_STR_JOIN => "ir.str.join".to_string(),
            IntrinsicFuncName::IR_STR_CONCAT => "ir.str.concat".to_string(),
            IntrinsicFuncName::IR_STR_REPLACE => "ir.str.replace".to_string(),
            IntrinsicFuncName::IR_STR_FIND => "ir.str.find".to_string(),
            IntrinsicFuncName::IR_STR_SUBSTR => "ir.str.substr".to_string(),
            IntrinsicFuncName::IR_STR_INSERT => "ir.str.insert".to_string(),
            IntrinsicFuncName::IR_STR_TO_BYTES => "ir.str.to_bytes".to_string(),
            IntrinsicFuncName::IR_STR_TO_I128 => "ir.str.to_i128".to_string(),
            IntrinsicFuncName::IR_STR_TO_U128 => "ir.str.to_u128".to_string(),

            IntrinsicFuncName::IR_BASE64_ENCODE => "ir.base64.encode".to_string(),
            IntrinsicFuncName::IR_BASE64_DECODE => "ir.base64.decode".to_string(),
            IntrinsicFuncName::IR_HEX_ENCODE => "ir.hex.encode".to_string(),
            IntrinsicFuncName::IR_HEX_DECODE => "ir.hex.decode".to_string(),
            IntrinsicFuncName::IR_SSZ_ENCODE => "ir.ssz.encode".to_string(),
            IntrinsicFuncName::IR_SSZ_DECODE => "ir.ssz.decode".to_string(),
            IntrinsicFuncName::IR_JSON_ENCODE => "ir.json.encode".to_string(),
            IntrinsicFuncName::IR_JSON_DECODE => "ir.json.decode".to_string(),
            IntrinsicFuncName::IR_RLP_ENCODE => "ir.rlp.encode".to_string(),
            IntrinsicFuncName::IR_RLP_DECODE => "ir.rlp.decode".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_BOOL => {
                "ir.data_stream.encode_bool".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U8 => "ir.data_stream.encode_u8".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U16 => "ir.data_stream.encode_u16".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U32 => "ir.data_stream.encode_u32".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U64 => "ir.data_stream.encode_u64".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U128 => {
                "ir.data_stream.encode_u128".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I8 => "ir.data_stream.encode_i8".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I16 => "ir.data_stream.encode_i16".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I32 => "ir.data_stream.encode_i32".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I64 => "ir.data_stream.encode_i64".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I128 => {
                "ir.data_stream.encode_i128".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STR => "ir.data_stream.encode_str".to_string(),
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_BOOLARRAY => {
                "ir.data_stream.encode_boolarray".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U8ARRAY => {
                "ir.data_stream.encode_u8array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U16ARRAY => {
                "ir.data_stream.encode_u16array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U32ARRAY => {
                "ir.data_stream.encode_u32array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U64ARRAY => {
                "ir.data_stream.encode_u64array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_U128ARRAY => {
                "ir.data_stream.encode_u128array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I8ARRAY => {
                "ir.data_stream.encode_i8array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I16ARRAY => {
                "ir.data_stream.encode_i16array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I32ARRAY => {
                "ir.data_stream.encode_i32array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I64ARRAY => {
                "ir.data_stream.encode_i64array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_I128ARRAY => {
                "ir.data_stream.encode_i128array".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRARRAY => {
                "ir.data_stream.encode_strarray".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRBOOLMAP => {
                "ir.data_stream.encode_strboolmap".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU8MAP => {
                "ir.data_stream.encode_stru8map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU16MAP => {
                "ir.data_stream.encode_stru16map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU32MAP => {
                "ir.data_stream.encode_stru32map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU64MAP => {
                "ir.data_stream.encode_stru64map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU128MAP => {
                "ir.data_stream.encode_stru128map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI8MAP => {
                "ir.data_stream.encode_stri8map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI16MAP => {
                "ir.data_stream.encode_stri16map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI32MAP => {
                "ir.data_stream.encode_stri32map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI64MAP => {
                "ir.data_stream.encode_stri64map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI128MAP => {
                "ir.data_stream.encode_stri128map".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRSTRMAP => {
                "ir.data_stream.encode_strstrmap".to_string()
            }
            IntrinsicFuncName::IR_DATASTREAM_DECODE => "ir.data_stream.decode".to_string(),
            IntrinsicFuncName::IR_MATH_POW => "ir.math.power".to_string(),
            IntrinsicFuncName::IR_MATH_ITOA => "ir.math.itoa".to_string(),
            _ => {
                if has_extend_context() {
                    let ext_ctx = get_extend_context();
                    let func_intrinsics = ext_ctx.get_ir_func_intrinsics();
                    for intr_info in func_intrinsics {
                        if intr_info.func_name == self.clone() {
                            return intr_info.ir_func_name.to_string();
                        }
                    }
                }
                unreachable!("not found intrinsic");
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartialFuncName {
    pub kind: PartialFuncNameKind<IntrinsicFuncName, ()>,
    pub metadata: Option<MetaDataId>,
}

impl Default for PartialFuncName {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialFuncName {
    pub fn new() -> Self {
        Self {
            kind: PartialFuncNameKind::Otherwise,
            metadata: None,
        }
    }

    pub fn get_name(&self) -> String {
        match &self.kind {
            PartialFuncNameKind::UserDefFunc(str) => str.clone(),
            PartialFuncNameKind::Intrinsic(intrinsic) => intrinsic.apply_name(),
            PartialFuncNameKind::HostAPI(_) => unimplemented!(),
            PartialFuncNameKind::Otherwise => unreachable!(),
        }
    }
}

impl From<String> for PartialFuncName {
    fn from(val: String) -> Self {
        let mut p = PartialFuncName::new();
        p.kind = PartialFuncNameKind::UserDefFunc(val);
        p
    }
}

impl From<IntrinsicFuncName> for PartialFuncName {
    fn from(val: IntrinsicFuncName) -> Self {
        let mut p = PartialFuncName::new();
        p.kind = PartialFuncNameKind::Intrinsic(val);
        p
    }
}
impl From<String> for IntrinsicFuncName {
    fn from(s: String) -> IntrinsicFuncName {
        match parse_intrinsic_func_name(&s) {
            Some(intrinsic) => intrinsic,
            None => unimplemented!("api {} unimplemented", s),
        }
    }
}

pub(crate) fn parse_intrinsic_func_name(func_name: &str) -> Option<IntrinsicFuncName> {
    match func_name {
        "ir.vector.set" => Some(IntrinsicFuncName::IR_VECTOR_SET),
        "ir.vector.get" => Some(IntrinsicFuncName::IR_VECTOR_GET),
        "ir.vector.create_iter" => Some(IntrinsicFuncName::IR_VECTOR_CREATE_ITER),
        "ir.vector.get_next" => Some(IntrinsicFuncName::IR_VECTOR_GET_NEXT),
        "ir.vector.obj_key" => Some(IntrinsicFuncName::IR_VECTOR_OBJ_KEY),
        "ir.vector.obj_value" => Some(IntrinsicFuncName::IR_VECTOR_OBJ_VALUE),
        "ir.vector.push" => Some(IntrinsicFuncName::IR_VECTOR_PUSH),
        "ir.vector.pop" => Some(IntrinsicFuncName::IR_VECTOR_POP),
        "ir.vector.insert" => Some(IntrinsicFuncName::IR_VECTOR_INSERT),
        "ir.vector.delete" => Some(IntrinsicFuncName::IR_VECTOR_DELETE),
        "ir.vector.len" => Some(IntrinsicFuncName::IR_VECTOR_LEN),
        "ir.vector.clear" => Some(IntrinsicFuncName::IR_VECTOR_CLEAR),
        "ir.vector.reverse" => Some(IntrinsicFuncName::IR_VECTOR_REVERSE),
        "ir.vector.at" => Some(IntrinsicFuncName::IR_VECTOR_AT),
        "ir.vector.slice" => Some(IntrinsicFuncName::IR_VECTOR_SLICE),
        "ir.vector.to_str" => Some(IntrinsicFuncName::IR_VECTOR_TO_STR),

        "ir.map.contains_key" => Some(IntrinsicFuncName::IR_MAP_CONTAINS_KEY),
        "ir.map.insert" => Some(IntrinsicFuncName::IR_MAP_INSERT),
        "ir.map.delete" => Some(IntrinsicFuncName::IR_MAP_DELETE),
        "ir.map.len" => Some(IntrinsicFuncName::IR_MAP_LEN),
        "ir.map.clear" => Some(IntrinsicFuncName::IR_MAP_CLEAR),
        "ir.map.set" => Some(IntrinsicFuncName::IR_MAP_SET),
        "ir.map.get" => Some(IntrinsicFuncName::IR_MAP_GET),
        "ir.map.create_iter" => Some(IntrinsicFuncName::IR_MAP_CREATE_ITER),
        "ir.map.get_next" => Some(IntrinsicFuncName::IR_MAP_GET_NEXT),
        "ir.map.obj_key" => Some(IntrinsicFuncName::IR_MAP_OBJ_KEY),
        "ir.map.obj_value" => Some(IntrinsicFuncName::IR_MAP_OBJ_VALUE),

        "ir.str.split" => Some(IntrinsicFuncName::IR_STR_SPLIT),
        "ir.str.len" => Some(IntrinsicFuncName::IR_STR_LEN),
        "ir.str.lower" => Some(IntrinsicFuncName::IR_STR_LOWER),
        "ir.str.upper" => Some(IntrinsicFuncName::IR_STR_UPPER),
        "ir.str.at" => Some(IntrinsicFuncName::IR_STR_AT),
        "ir.str.count" => Some(IntrinsicFuncName::IR_STR_COUNT),
        "ir.str.startswith" => Some(IntrinsicFuncName::IR_STR_STARTSWITH),
        "ir.str.endswith" => Some(IntrinsicFuncName::IR_STR_ENDSWITH),
        "ir.str.isalnum" => Some(IntrinsicFuncName::IR_STR_ISALNUM),
        "ir.str.isalpha" => Some(IntrinsicFuncName::IR_STR_ISALPHA),
        "ir.str.isdigit" => Some(IntrinsicFuncName::IR_STR_ISDIGIT),
        "ir.str.islower" => Some(IntrinsicFuncName::IR_STR_ISLOWER),
        "ir.str.isupper" => Some(IntrinsicFuncName::IR_STR_ISUPPER),
        "ir.str.isspace" => Some(IntrinsicFuncName::IR_STR_ISSPACE),
        "ir.str.strip" => Some(IntrinsicFuncName::IR_STR_STRIP),
        "ir.str.lstrip" => Some(IntrinsicFuncName::IR_STR_LSTRIP),
        "ir.str.rstrip" => Some(IntrinsicFuncName::IR_STR_RSTRIP),
        "ir.str.join" => Some(IntrinsicFuncName::IR_STR_JOIN),
        "ir.str.concat" => Some(IntrinsicFuncName::IR_STR_CONCAT),
        "ir.str.replace" => Some(IntrinsicFuncName::IR_STR_REPLACE),
        "ir.str.find" => Some(IntrinsicFuncName::IR_STR_FIND),
        "ir.str.substr" => Some(IntrinsicFuncName::IR_STR_SUBSTR),
        "ir.str.insert" => Some(IntrinsicFuncName::IR_STR_INSERT),
        "ir.str.to_bytes" => Some(IntrinsicFuncName::IR_STR_TO_BYTES),
        "ir.str.to_i128" => Some(IntrinsicFuncName::IR_STR_TO_I128),
        "ir.str.to_u128" => Some(IntrinsicFuncName::IR_STR_TO_U128),

        "ir.builtin.abort" => Some(IntrinsicFuncName::IR_BUILTIN_ABORT),
        "ir.builtin.assert" => Some(IntrinsicFuncName::IR_BUILTIN_ASSERT),
        "ir.builtin.require" => Some(IntrinsicFuncName::IR_BUILTIN_REQUIRE),
        "ir.builtin.print" => Some(IntrinsicFuncName::IR_BUILTIN_PRINT),
        "ir.builtin.print_type" => Some(IntrinsicFuncName::IR_BUILTIN_PRINT_TYPE),
        "ir.builtin.address" => Some(IntrinsicFuncName::IR_BUILTIN_ADDRESS),
        "ir.builtin.encode_base64" => Some(IntrinsicFuncName::IR_BUILTIN_ENCODE_BASE64),
        "ir.builtin.decode_base64" => Some(IntrinsicFuncName::IR_BUILTIN_DECODE_BASE64),
        "ir.builtin.encode_hex" => Some(IntrinsicFuncName::IR_BUILTIN_ENCODE_HEX),
        "ir.builtin.decode_hex" => Some(IntrinsicFuncName::IR_BUILTIN_DECODE_HEX),
        "ir.builtin.encode_params" => Some(IntrinsicFuncName::IR_BUILTIN_ENCODE_PARAMS),
        "ir.builtin.call_log" => Some(IntrinsicFuncName::IR_BUILTIN_CALL_LOG),
        "ir.builtin.get_call_result" => Some(IntrinsicFuncName::IR_BUILTIN_GET_CALL_RESULT),
        "ir.builtin.sha256" => Some(IntrinsicFuncName::IR_BUILTIN_SHA256),
        "ir.builtin.sm3" => Some(IntrinsicFuncName::IR_BUILTIN_SM3),
        "ir.builtin.keccak256" => Some(IntrinsicFuncName::IR_BUILTIN_KECCAK256),
        "ir.builtin.verify_mycrypto_signature" => {
            Some(IntrinsicFuncName::IR_BUILTIN_VERIFY_MYCRYPTO_SIGNATURE)
        }
        "ir.builtin.eth_secp256k1_recovery" => {
            Some(IntrinsicFuncName::IR_BUILTIN_ETH_SECP256K1_RECOVERY)
        }
        "ir.builtin.get_account" => Some(IntrinsicFuncName::IR_BUILTIN_GET_ACCOUT),
        "ir.builtin.get_contract" => Some(IntrinsicFuncName::IR_BUILTIN_GET_CONTRACT),
        "ir.builtin.get_artifact" => Some(IntrinsicFuncName::IR_BUILTIN_GET_ARTIFACT),
        "ir.builtin.add_coverage_counter" => {
            Some(IntrinsicFuncName::IR_BUILTIN_ADD_COVERAGE_COUNTER)
        }
        "ir.builtin.cocall" => Some(IntrinsicFuncName::IR_BUILTIN_COCALL),
        "ir.builtin.co_call" => Some(IntrinsicFuncName::IR_BUILTIN_CO_CALL_DIRECTLY), // also co_call directly
        "ir.builtin.co_call_directly" => Some(IntrinsicFuncName::IR_BUILTIN_CO_CALL_DIRECTLY),
        "ir.builtin.block_number" => Some(IntrinsicFuncName::IR_BUILTIN_BLOCK_NUMBER),
        "ir.builtin.block_timestamp" => Some(IntrinsicFuncName::IR_BUILTIN_BLOCK_TIMESTAMP),
        "ir.builtin.block_random_seed" => Some(IntrinsicFuncName::IR_BUILTIN_BLOCK_RANDOM_SEED),
        "ir.builtin.block_version" => Some(IntrinsicFuncName::IR_BUILTIN_BLOCK_VERSION),
        "ir.builtin.block_flags" => Some(IntrinsicFuncName::IR_BUILTIN_BLOCK_FLAGS),
        "ir.builtin.tx_sender" => Some(IntrinsicFuncName::IR_BUILTIN_TX_SENDER),
        "ir.builtin.tx_hash" => Some(IntrinsicFuncName::IR_BUILTIN_TX_HASH),
        "ir.builtin.tx_index" => Some(IntrinsicFuncName::IR_BUILTIN_TX_INDEX),
        "ir.builtin.tx_gas_limit" => Some(IntrinsicFuncName::IR_BUILTIN_TX_GAS_LIMIT),
        "ir.builtin.tx_timestamp" => Some(IntrinsicFuncName::IR_BUILTIN_TX_TIMESTAMP),
        "ir.builtin.tx_nonce" => Some(IntrinsicFuncName::IR_BUILTIN_TX_NONCE),
        "ir.builtin.call_sender" => Some(IntrinsicFuncName::IR_BUILTIN_CALL_SENDER),
        "ir.builtin.call_this_contract" => Some(IntrinsicFuncName::IR_BUILTIN_CALL_THIS_CONTRACT),
        "ir.builtin.call_op_contract" => Some(IntrinsicFuncName::IR_BUILTIN_CALL_OP_CONTRACT),
        "ir.builtin.call_gas_limit" => Some(IntrinsicFuncName::IR_BUILTIN_CALL_GAS_LIMIT),
        "ir.builtin.call_gas_left" => Some(IntrinsicFuncName::IR_BUILTIN_CALL_GAS_LEFT),

        "ir.storage.push" => Some(IntrinsicFuncName::IR_STORAGE_PUSH),
        "ir.storage.push_empty" => Some(IntrinsicFuncName::IR_STORAGE_PUSH_EMPTY),
        "ir.storage.multiarray_push_empty" => {
            Some(IntrinsicFuncName::IR_STORAGE_MULTIARRAY_PUSH_EMPTY)
        }
        "ir.storage.pop" => Some(IntrinsicFuncName::IR_STORAGE_POP),
        "ir.storage.len" => Some(IntrinsicFuncName::IR_STORAGE_LEN),
        "ir.storage.verify_index" => Some(IntrinsicFuncName::IR_STORAGE_VERIFY_INDEX),
        "ir.storage.contains_key" => Some(IntrinsicFuncName::IR_STORAGE_CONTAINS_KEY),
        "ir.storage.contains_asset" => Some(IntrinsicFuncName::IR_STORAGE_CONTAINS_ASSET),
        "ir.storage.delete" => Some(IntrinsicFuncName::IR_STORAGE_DELETE),
        "ir.storage.mint" => Some(IntrinsicFuncName::IR_STORAGE_MINT),
        "ir.storage.burn" => Some(IntrinsicFuncName::IR_STORAGE_BURN),
        "ir.storage.destroy" => Some(IntrinsicFuncName::IR_STORAGE_DESTROY),
        "ir.storage.get_balance" => Some(IntrinsicFuncName::IR_STORAGE_GET_BALANCE),
        "ir.storage.get_tag" => Some(IntrinsicFuncName::IR_STORAGE_GET_TAG),
        "ir.storage.transfer" => Some(IntrinsicFuncName::IR_STORAGE_TRANSFER),
        "ir.storage.set_bss" => Some(IntrinsicFuncName::IR_STORAGE_SET_BSS),
        "ir.storage.get_bss" => Some(IntrinsicFuncName::IR_STORAGE_GET_BSS),
        "ir.storage.path_join" => Some(IntrinsicFuncName::IR_STORAGE_PATH_JOIN),

        "ir.data_stream.encode_bool" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_BOOL),
        "ir.data_stream.encode_u8" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U8),
        "ir.data_stream.encode_u16" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U16),
        "ir.data_stream.encode_u32" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U32),
        "ir.data_stream.encode_u64" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U64),
        "ir.data_stream.encode_u128" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U128),
        "ir.data_stream.encode_i8" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I8),
        "ir.data_stream.encode_i16" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I16),
        "ir.data_stream.encode_i32" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I32),
        "ir.data_stream.encode_i64" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I64),
        "ir.data_stream.encode_i128" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I128),
        "ir.data_stream.encode_str" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STR),
        "ir.data_stream.encode_boolarray" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_BOOLARRAY)
        }
        "ir.data_stream.encode_u8array" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U8ARRAY),
        "ir.data_stream.encode_u16array" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U16ARRAY),
        "ir.data_stream.encode_u32array" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U32ARRAY),
        "ir.data_stream.encode_u64array" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U64ARRAY),
        "ir.data_stream.encode_u128array" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_U128ARRAY)
        }
        "ir.data_stream.encode_i8array" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I8ARRAY),
        "ir.data_stream.encode_i16array" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I16ARRAY),
        "ir.data_stream.encode_i32array" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I32ARRAY),
        "ir.data_stream.encode_i64array" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I64ARRAY),
        "ir.data_stream.encode_i128array" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_I128ARRAY)
        }
        "ir.data_stream.encode_strarray" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRARRAY),
        "ir.data_stream.encode_strboolmap" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRBOOLMAP)
        }
        "ir.data_stream.encode_stru8map" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU8MAP),
        "ir.data_stream.encode_stru16map" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU16MAP)
        }
        "ir.data_stream.encode_stru32map" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU32MAP)
        }
        "ir.data_stream.encode_stru64map" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU64MAP)
        }
        "ir.data_stream.encode_stru128map" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRU128MAP)
        }
        "ir.data_stream.encode_stri8map" => Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI8MAP),
        "ir.data_stream.encode_stri16map" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI16MAP)
        }
        "ir.data_stream.encode_stri32map" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI32MAP)
        }
        "ir.data_stream.encode_stri64map" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI64MAP)
        }
        "ir.data_stream.encode_stri128map" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRI128MAP)
        }
        "ir.data_stream.encode_strstrmap" => {
            Some(IntrinsicFuncName::IR_DATASTREAM_ENCODE_STRSTRMAP)
        }
        "ir.data_stream.decode" => Some(IntrinsicFuncName::IR_DATASTREAM_DECODE),
        "ir.base64.encode" => Some(IntrinsicFuncName::IR_BASE64_ENCODE),
        "ir.base64.decode" => Some(IntrinsicFuncName::IR_BASE64_DECODE),
        "ir.hex.encode" => Some(IntrinsicFuncName::IR_HEX_ENCODE),
        "ir.hex.decode" => Some(IntrinsicFuncName::IR_HEX_DECODE),
        "ir.ssz.encode" => Some(IntrinsicFuncName::IR_SSZ_ENCODE),
        "ir.ssz.decode" => Some(IntrinsicFuncName::IR_SSZ_DECODE),
        "ir.json.encode" => Some(IntrinsicFuncName::IR_JSON_ENCODE),
        "ir.json.decode" => Some(IntrinsicFuncName::IR_JSON_DECODE),
        "ir.rlp.encode" => Some(IntrinsicFuncName::IR_RLP_ENCODE),
        "ir.rlp.decode" => Some(IntrinsicFuncName::IR_RLP_DECODE),

        "ir.math.pow" => Some(IntrinsicFuncName::IR_MATH_POW),
        "ir.math.itoa" => Some(IntrinsicFuncName::IR_MATH_ITOA),
        "ir.builtin.revert" => Some(IntrinsicFuncName::IR_BUILTIN_REVERT),
        "ir.twisted_elgamal.add" => Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_ADD),
        "ir.twisted_elgamal.sub" => Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_SUB),
        "ir.twisted_elgamal.scalar_multiply" => {
            Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_SCALAR_MULTIPLY)
        }
        "ir.twisted_elgamal.verify_publickey" => {
            Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_VERIFY_PUBLICKEY)
        }
        "ir.twisted_elgamal.verify_cipher" => {
            Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_VERIFY_CIPHER)
        }
        "ir.twisted_elgamal.verify_equal_amount" => {
            Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_VERIFY_EQUAL_AMOUNT)
        }
        "ir.twisted_elgamal.verify_zero_amount" => {
            Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_VERIFY_ZERO_AMOUNT)
        }
        "ir.twisted_elgamal.verify_amount_range" => {
            Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_VERIFY_AMOUNT_RANGE)
        }
        "ir.twisted_elgamal.verify_amount_proportion" => {
            Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_VERIFY_AMOUNT_PROPORTION)
        }
        "ir.twisted_elgamal.verify_amount_greater_equal" => {
            Some(IntrinsicFuncName::IR_TWISTED_ELGAMAL_VERIFY_AMOUNT_GREATER_EQUAL)
        }
        "ir.rand.gen_u64" => Some(IntrinsicFuncName::IR_RAND_GEN_U64),
        "ir.rand.gen_u64_from_seed" => Some(IntrinsicFuncName::IR_RAND_GEN_U64_FROM_SEED),
        "ir.rand.gen_u64_array" => Some(IntrinsicFuncName::IR_RAND_GEN_U64_ARRAY),
        "ir.rand.gen_u64_array_from_seed" => {
            Some(IntrinsicFuncName::IR_RAND_GEN_U64_ARRAY_FROM_SEED)
        }
        _ => None,
    }
}
