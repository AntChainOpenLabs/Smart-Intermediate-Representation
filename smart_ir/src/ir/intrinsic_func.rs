// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::interface_type::{register_intrinsic_func_name, IntrinsicFuncName};

#[macro_export]
macro_rules! create_variant_intrinsic {
    ($name:ident, $func_name:tt) => {
        impl IntrinsicFuncName{
            // pub const concat_idents!(OBJ_, $name): IntrinsicFuncName = IntrinsicFuncName::new(stringify!($name), $func_name);
            pub const $name: &'static str = stringify!($name);
        }
        register_intrinsic_func_name(&IntrinsicFuncName::new(stringify!($name), $func_name));

    };
}

pub fn initialize_intrinisic_func_names() {
    // Vector
    create_variant_intrinsic!(IR_VECTOR_SET, "ir.vector.set");
    create_variant_intrinsic!(IR_VECTOR_GET, "ir.vector.get");
    create_variant_intrinsic!(IR_VECTOR_CREATE_ITER, "ir.vector.create_iter");
    create_variant_intrinsic!(IR_VECTOR_GET_NEXT, "ir.vector.get_next");
    create_variant_intrinsic!(IR_VECTOR_OBJ_KEY, "ir.vector.obj_key");
    create_variant_intrinsic!(IR_VECTOR_OBJ_VALUE, "ir.vector.obj_value");
    create_variant_intrinsic!(IR_VECTOR_PUSH, "ir.vector.push");
    create_variant_intrinsic!(IR_VECTOR_POP, "ir.vector.pop");
    create_variant_intrinsic!(IR_VECTOR_INSERT, "ir.vector.insert");
    create_variant_intrinsic!(IR_VECTOR_DELETE, "ir.vector.delete");
    create_variant_intrinsic!(IR_VECTOR_LEN, "ir.vector.len");
    create_variant_intrinsic!(IR_VECTOR_CLEAR, "ir.vector.clear");
    create_variant_intrinsic!(IR_VECTOR_REVERSE, "ir.vector.reverse");
    create_variant_intrinsic!(IR_VECTOR_AT, "ir.vector.at");
    create_variant_intrinsic!(IR_VECTOR_SLICE, "ir.vector.slice");
    create_variant_intrinsic!(IR_VECTOR_TO_STR, "ir.vector.to_str");

    // Tuple
    create_variant_intrinsic!(IR_TUPLE_SET, "ir.tuple.set");
    create_variant_intrinsic!(IR_TUPLE_GET, "ir.tuple.get");
    create_variant_intrinsic!(IR_TUPLE_LEN, "ir.tuple.len");
    create_variant_intrinsic!(IR_TUPLE_AT, "ir.tuple.at");

    // Map
    create_variant_intrinsic!(IR_MAP_SET, "ir.map.set");
    create_variant_intrinsic!(IR_MAP_GET, "ir.map.get");
    create_variant_intrinsic!(IR_MAP_CREATE_ITER, "ir.map.create_iter");
    create_variant_intrinsic!(IR_MAP_GET_NEXT, "ir.map.get_next");
    create_variant_intrinsic!(IR_MAP_OBJ_KEY, "ir.map.obj_key");
    create_variant_intrinsic!(IR_MAP_OBJ_VALUE, "ir.map.obj_value");
    create_variant_intrinsic!(IR_MAP_CONTAINS_KEY, "ir.map.contains_key");
    create_variant_intrinsic!(IR_MAP_INSERT, "ir.map.insert");
    create_variant_intrinsic!(IR_MAP_DELETE, "ir.map.delete");
    create_variant_intrinsic!(IR_MAP_LEN, "ir.map.len");
    create_variant_intrinsic!(IR_MAP_CLEAR, "ir.map.clear");

    // Storage
    create_variant_intrinsic!(IR_STORAGE_PUSH, "ir.storage.push");
    create_variant_intrinsic!(IR_STORAGE_PUSH_EMPTY, "ir.storage.push_empty");
    create_variant_intrinsic!(
        IR_STORAGE_MULTIARRAY_PUSH_EMPTY,
        "ir.storage.multiarray_push_empty"
    );
    create_variant_intrinsic!(IR_STORAGE_POP, "ir.storage.pop");
    create_variant_intrinsic!(IR_STORAGE_LEN, "ir.storage.len");
    create_variant_intrinsic!(IR_STORAGE_VERIFY_INDEX, "ir.storage.verify_index");
    create_variant_intrinsic!(IR_STORAGE_CONTAINS_KEY, "ir.storage.contains_key");
    create_variant_intrinsic!(IR_STORAGE_CONTAINS_ASSET, "ir.storage.contains_asset");
    create_variant_intrinsic!(IR_STORAGE_DELETE, "ir.storage.delete");
    create_variant_intrinsic!(IR_STORAGE_MINT, "ir.storage.mint");
    create_variant_intrinsic!(IR_STORAGE_BURN, "ir.storage.burn");
    create_variant_intrinsic!(IR_STORAGE_DESTROY, "ir.storage.destroy");
    create_variant_intrinsic!(IR_STORAGE_GET_BALANCE, "ir.storage.get_balance");
    create_variant_intrinsic!(IR_STORAGE_GET_TAG, "ir.storage.get_tag");
    create_variant_intrinsic!(IR_STORAGE_TRANSFER, "ir.storage.transfer");
    create_variant_intrinsic!(IR_STORAGE_SET_BSS, "ir.storage.set_bss");
    create_variant_intrinsic!(IR_STORAGE_GET_BSS, "ir.storage.get_bss");
    create_variant_intrinsic!(IR_STORAGE_PATH_JOIN, "ir.storage.path_join");

    // Builtin function
    create_variant_intrinsic!(IR_BUILTIN_ABORT, "ir.builtin.abort");
    create_variant_intrinsic!(IR_BUILTIN_ASSERT, "ir.builtin.assert");
    create_variant_intrinsic!(IR_BUILTIN_REQUIRE, "ir.builtin.require");
    create_variant_intrinsic!(IR_BUILTIN_PRINT, "ir.builtin.print");
    create_variant_intrinsic!(IR_BUILTIN_PRINT_TYPE, "ir.builtin.print_type");
    create_variant_intrinsic!(IR_BUILTIN_ADDRESS, "ir.builtin.address");
    create_variant_intrinsic!(IR_BUILTIN_ENCODE_BASE64, "ir.builtin.encode_base64");
    create_variant_intrinsic!(IR_BUILTIN_DECODE_BASE64, "ir.builtin.decode_base64");
    create_variant_intrinsic!(IR_BUILTIN_ENCODE_HEX, "ir.builtin.encode_hex");
    create_variant_intrinsic!(IR_BUILTIN_DECODE_HEX, "ir.builtin.decode_hex");
    create_variant_intrinsic!(IR_BUILTIN_ENCODE_PARAMS, "ir.builtin.encode_params");
    create_variant_intrinsic!(IR_BUILTIN_CALL_LOG, "ir.builtin.call_log");
    create_variant_intrinsic!(IR_BUILTIN_SHA256, "ir.builtin.sha256");
    create_variant_intrinsic!(IR_BUILTIN_SM3, "ir.builtin.sm3");
    create_variant_intrinsic!(IR_BUILTIN_KECCAK256, "ir.builtin.keccak256");
    create_variant_intrinsic!(
        IR_BUILTIN_VERIFY_MYCRYPTO_SIGNATURE,
        "ir.builtin.verify_mycrypto_signature"
    );
    create_variant_intrinsic!(
        IR_BUILTIN_ETH_SECP256K1_RECOVERY,
        "ir.builtin.eth_secp256k1_recovery"
    );
    create_variant_intrinsic!(IR_BUILTIN_GET_ACCOUT, "ir.builtin.get_account");
    create_variant_intrinsic!(IR_BUILTIN_GET_CONTRACT, "ir.builtin.get_contract");
    create_variant_intrinsic!(IR_BUILTIN_GET_ARTIFACT, "ir.builtin.get_artifact");
    create_variant_intrinsic!(
        IR_BUILTIN_ADD_COVERAGE_COUNTER,
        "ir.builtin.add_coverage_counter"
    );
    create_variant_intrinsic!(IR_BUILTIN_COCALL, "ir.builtin.co_call");
    create_variant_intrinsic!(IR_BUILTIN_CO_CALL, "ir.builtin.co_call");
    // co_call by let a: Contract = Contract("xxx"); a.fib(...)
    create_variant_intrinsic!(IR_BUILTIN_CO_CALL_DIRECTLY, "ir.builtin.co_call_directly");
    // co_call directly by let err: i32 = co_call("xxx","fib",...)
    create_variant_intrinsic!(IR_BUILTIN_REVERT, "ir.builtin.revert");
    create_variant_intrinsic!(IR_BUILTIN_GET_CALL_RESULT, "ir.builtin.get_call_result");

    // Builtin variable
    create_variant_intrinsic!(IR_BUILTIN_BLOCK_NUMBER, "ir.builtin.block_number");
    create_variant_intrinsic!(IR_BUILTIN_BLOCK_TIMESTAMP, "ir.builtin.block_timestamp");
    create_variant_intrinsic!(IR_BUILTIN_BLOCK_RANDOM_SEED, "ir.builtin.block_random_seed");
    create_variant_intrinsic!(IR_BUILTIN_BLOCK_VERSION, "ir.builtin.block_version");
    // create_variant_intrinsic!(IR_BUILTIN_BLOCK_FLAGS, "ir.builtin.block_flags");

    create_variant_intrinsic!(IR_BUILTIN_TX_SENDER, "ir.builtin.tx_sender");
    create_variant_intrinsic!(IR_BUILTIN_TX_HASH, "ir.builtin.tx_hash");
    create_variant_intrinsic!(IR_BUILTIN_TX_INDEX, "ir.builtin.tx_index");
    create_variant_intrinsic!(IR_BUILTIN_TX_GAS_LIMIT, "ir.builtin.tx_gas_limit");
    create_variant_intrinsic!(IR_BUILTIN_TX_TIMESTAMP, "ir.builtin.tx_timestamp");
    create_variant_intrinsic!(IR_BUILTIN_TX_NONCE, "ir.builtin.tx_nonce");

    create_variant_intrinsic!(IR_BUILTIN_CALL_SENDER, "ir.builtin.call_sender");
    create_variant_intrinsic!(
        IR_BUILTIN_CALL_THIS_CONTRACT,
        "ir.builtin.call_this_contract"
    );
    create_variant_intrinsic!(IR_BUILTIN_CALL_OP_CONTRACT, "ir.builtin.call_op_contract");
    create_variant_intrinsic!(IR_BUILTIN_CALL_GAS_LEFT, "ir.builtin.call_gas_left");
    create_variant_intrinsic!(IR_BUILTIN_CALL_GAS_LIMIT, "ir.builtin.call_gas_limit");

    // STR
    create_variant_intrinsic!(IR_STR_SPLIT, "ir.str.split");
    create_variant_intrinsic!(IR_STR_LEN, "ir.str.len");
    create_variant_intrinsic!(IR_STR_LOWER, "ir.str.lower");
    create_variant_intrinsic!(IR_STR_UPPER, "ir.str.upper");
    create_variant_intrinsic!(IR_STR_AT, "ir.str.at");
    create_variant_intrinsic!(IR_STR_COUNT, "ir.str.count");
    create_variant_intrinsic!(IR_STR_STARTSWITH, "ir.str.startswith");
    create_variant_intrinsic!(IR_STR_ENDSWITH, "ir.str.endswith");
    create_variant_intrinsic!(IR_STR_ISALNUM, "ir.str.isalnum");
    create_variant_intrinsic!(IR_STR_ISALPHA, "ir.str.isalpha");
    create_variant_intrinsic!(IR_STR_ISDIGIT, "ir.str.isdigit");
    create_variant_intrinsic!(IR_STR_ISLOWER, "ir.str.islower");
    create_variant_intrinsic!(IR_STR_ISUPPER, "ir.str.isupper");
    create_variant_intrinsic!(IR_STR_ISSPACE, "ir.str.isspace");
    create_variant_intrinsic!(IR_STR_STRIP, "ir.str.strip");
    create_variant_intrinsic!(IR_STR_LSTRIP, "ir.str.lstrip");
    create_variant_intrinsic!(IR_STR_RSTRIP, "ir.str.rstrip");
    create_variant_intrinsic!(IR_STR_JOIN, "ir.str.join");
    create_variant_intrinsic!(IR_STR_CONCAT, "ir.str.concat");
    create_variant_intrinsic!(IR_STR_REPLACE, "ir.str.replace");
    create_variant_intrinsic!(IR_STR_FIND, "ir.str.find");
    create_variant_intrinsic!(IR_STR_SUBSTR, "ir.str.substr");
    create_variant_intrinsic!(IR_STR_INSERT, "ir.str.insert");
    create_variant_intrinsic!(IR_STR_TO_BYTES, "ir.str.to_bytes");
    create_variant_intrinsic!(IR_STR_TO_I128, "ir.str.to_i128");
    create_variant_intrinsic!(IR_STR_TO_U128, "ir.str.to_u128");

    // builtin module func

    // Base64
    create_variant_intrinsic!(IR_BASE64_ENCODE, "ir.base64.encode");
    create_variant_intrinsic!(IR_BASE64_DECODE, "ir.base64.decode");

    // Hex
    create_variant_intrinsic!(IR_HEX_ENCODE, "ir.hex.encode");
    create_variant_intrinsic!(IR_HEX_DECODE, "ir.hex.decode");

    // SSZ
    create_variant_intrinsic!(IR_SSZ_ENCODE, "ir.ssz.encode");
    create_variant_intrinsic!(IR_SSZ_DECODE, "ir.ssz.decode");

    // JSON
    create_variant_intrinsic!(IR_JSON_ENCODE, "ir.json.encode");
    create_variant_intrinsic!(IR_JSON_DECODE, "ir.json.decode");

    // rlp
    create_variant_intrinsic!(IR_RLP_ENCODE, "ir.rlp.encode");
    create_variant_intrinsic!(IR_RLP_DECODE, "ir.rlp.decode");

    // Datastream
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_BOOL, "ir.data_stream.encode_bool");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_U8, "ir.data_stream.encode_u8");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_U16, "ir.data_stream.encode_u16");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_U32, "ir.data_stream.encode_u32");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_U64, "ir.data_stream.encode_u64");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_U128, "ir.data_stream.encode_u128");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_I8, "ir.data_stream.encode_i8");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_I16, "ir.data_stream.encode_i16");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_I32, "ir.data_stream.encode_i32");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_I64, "ir.data_stream.encode_i64");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_I128, "ir.data_stream.encode_i128");
    create_variant_intrinsic!(IR_DATASTREAM_ENCODE_STR, "ir.data_stream.encode_str");
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_BOOLARRAY,
        "ir.data_stream.encode_boolarray"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_U8ARRAY,
        "ir.data_stream.encode_u8array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_U16ARRAY,
        "ir.data_stream.encode_u16array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_U32ARRAY,
        "ir.data_stream.encode_u32array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_U64ARRAY,
        "ir.data_stream.encode_u64array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_U128ARRAY,
        "ir.data_stream.encode_u128array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_I8ARRAY,
        "ir.data_stream.encode_i8array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_I16ARRAY,
        "ir.data_stream.encode_i16array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_I32ARRAY,
        "ir.data_stream.encode_i32array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_I64ARRAY,
        "ir.data_stream.encode_i64array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_I128ARRAY,
        "ir.data_stream.encode_i128array"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRARRAY,
        "ir.data_stream.encode_strarray"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRBOOLMAP,
        "ir.data_stream.encode_strboolmap"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRU8MAP,
        "ir.data_stream.encode_stru8map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRU16MAP,
        "ir.data_stream.encode_stru16map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRU32MAP,
        "ir.data_stream.encode_stru32map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRU64MAP,
        "ir.data_stream.encode_stru64map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRU128MAP,
        "ir.data_stream.encode_stru128map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRI8MAP,
        "ir.data_stream.encode_stri8map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRI16MAP,
        "ir.data_stream.encode_stri16map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRI32MAP,
        "ir.data_stream.encode_stri32map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRI64MAP,
        "ir.data_stream.encode_stri64map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRI128MAP,
        "ir.data_stream.encode_stri128map"
    );
    create_variant_intrinsic!(
        IR_DATASTREAM_ENCODE_STRSTRMAP,
        "ir.data_stream.encode_strstrmap"
    );
    create_variant_intrinsic!(IR_DATASTREAM_DECODE, "ir.data_stream.decode");

    // Math
    create_variant_intrinsic!(IR_MATH_POW, "ir.math.pow");
    create_variant_intrinsic!(IR_MATH_ITOA, "ir.math.itoa");
}
