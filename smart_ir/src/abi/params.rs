// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use nano_leb128::ULEB128;
use num_bigint::BigInt;
#[allow(unused_imports)]
use num_traits::FromPrimitive;
use std::collections::HashMap;

use crate::encoding::datastream::ParamType;

type Bytes = Vec<u8>;
#[allow(unused)]
const VERSION: u8 = 0;

/// Smart Intermediate Representation ABI params.
///
/// TODO: use macros and se/der traits and macros to impl.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ABIParam {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
    Bool(bool),
    Str(String),
    Parampack(Vec<u8>),

    // Array params
    U8Array(Vec<u8>),
    I8Array(Vec<i8>),
    U16Array(Vec<u16>),
    I16Array(Vec<i16>),
    U32Array(Vec<u32>),
    I32Array(Vec<i32>),
    U64Array(Vec<u64>),
    I64Array(Vec<i64>),
    U128Array(Vec<u128>),
    I128Array(Vec<i128>),
    BoolArray(Vec<bool>),
    StrArray(Vec<String>),

    // Map params
    StrU8Map(HashMap<String, u8>),
    StrI8Map(HashMap<String, i8>),
    StrU16Map(HashMap<String, u16>),
    StrI16Map(HashMap<String, i16>),
    StrU32Map(HashMap<String, u32>),
    StrI32Map(HashMap<String, i32>),
    StrU64Map(HashMap<String, u64>),
    StrI64Map(HashMap<String, i64>),
    StrU128Map(HashMap<String, u128>),
    StrI128Map(HashMap<String, i128>),
    StrBoolMap(HashMap<String, bool>),
    StrStrMap(HashMap<String, String>),

    // 256-bit number
    U256(BigInt),
    I256(BigInt),
    U256Array(Vec<BigInt>),
    I256Array(Vec<BigInt>),
    StrU256Map(HashMap<String, BigInt>),
    StrI256Map(HashMap<String, BigInt>),
}

macro_rules! encode_vec {
    ($v:expr, $id:ident) => {{
        let mut buf = buffer_starts_with_uleb128_len($v.len());
        for elem_v in $v {
            buf.append(&mut ABIParam::$id(elem_v.clone()).as_bytes());
        }
        buf
    }};
}

macro_rules! encode_map {
    ($v:expr, $key_id:ident, $val_id:ident) => {{
        let mut buf = buffer_starts_with_uleb128_len($v.len());
        for (str_k, elem_v) in $v {
            buf.append(&mut ABIParam::$key_id(str_k.clone()).as_bytes());
            buf.append(&mut ABIParam::$val_id(elem_v.clone()).as_bytes());
        }
        buf
    }};
}

macro_rules! decode_int {
    ($data:expr, $offset:expr, $id:ident, $ty_id:ident, $size:expr) => {{
        const SIZE: usize = $size;
        let bytes = get_bytes::<SIZE>($data, $offset);
        let param = ABIParam::$id($ty_id::from_le_bytes(bytes));
        *$offset += SIZE;
        Ok(param)
    }};
}

macro_rules! decode_vec {
    ($data:expr, $offset:expr, $id:ident, $id_arr:ident) => {{
        let mut result = vec![];
        let (len, mut elem_offset) = read_uleb128_len_and_offset($data, $offset)?;
        for _ in 0..len {
            let param = decode_param(&ParamType::$id, $data, &mut elem_offset)?;
            if let ABIParam::$id(v) = param {
                result.push(v);
            } else {
                return Err(anyhow!("decode {} array element error", stringify!($id)));
            }
        }
        *$offset = elem_offset;
        Ok(ABIParam::$id_arr(result))
    }};
}

macro_rules! decode_map {
    ($data:expr, $offset:expr, $key_id:ident, $val_id:ident, $id_arr:ident) => {{
        let mut result = HashMap::new();
        let (len, mut elem_offset) = read_uleb128_len_and_offset($data, $offset)?;
        for _ in 0..len {
            let param_key = decode_param(&ParamType::$key_id, $data, &mut elem_offset)?;
            let key = if let ABIParam::$key_id(v) = param_key {
                v
            } else {
                return Err(anyhow!(
                    "data stream decode {} {} map key error",
                    stringify!($key_id),
                    stringify!($val_id)
                ));
            };
            let param_val = decode_param(&ParamType::$val_id, $data, &mut elem_offset)?;
            let val = if let ABIParam::$val_id(v) = param_val {
                v
            } else {
                return Err(anyhow!(
                    "data stream decode {} {} map value error",
                    stringify!($key_id),
                    stringify!($val_id)
                ));
            };
            result.insert(key, val);
        }
        *$offset = elem_offset;
        Ok(ABIParam::$id_arr(result))
    }};
}

impl ABIParam {
    /// Cast contract ABI parameter to data stream bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            ABIParam::U8(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::I8(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::U16(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::I16(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::U32(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::I32(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::U64(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::I64(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::U128(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::I128(v) => (*v).to_le_bytes().to_vec(),
            ABIParam::U256(v) => (*v).to_bytes_le().1.to_vec(),
            ABIParam::I256(v) => (*v).to_bytes_le().1.to_vec(),
            ABIParam::Bool(v) => vec![(*v) as u8],
            ABIParam::Str(v) => {
                let mut buf = buffer_starts_with_uleb128_len(v.len());
                buf.append(&mut v.clone().as_bytes().to_vec());
                buf
            }
            ABIParam::U8Array(v) => encode_vec!(v, U8),
            ABIParam::I8Array(v) => encode_vec!(v, I8),
            ABIParam::U16Array(v) => encode_vec!(v, U16),
            ABIParam::I16Array(v) => encode_vec!(v, I16),
            ABIParam::U32Array(v) => encode_vec!(v, U32),
            ABIParam::I32Array(v) => encode_vec!(v, I32),
            ABIParam::U64Array(v) => encode_vec!(v, U64),
            ABIParam::I64Array(v) => encode_vec!(v, I64),
            ABIParam::U128Array(v) => encode_vec!(v, U128),
            ABIParam::I128Array(v) => encode_vec!(v, I128),
            ABIParam::U256Array(v) => encode_vec!(v, U256),
            ABIParam::I256Array(v) => encode_vec!(v, I256),
            ABIParam::BoolArray(v) => encode_vec!(v, Bool),
            ABIParam::StrArray(v) => encode_vec!(v, Str),
            ABIParam::StrU8Map(v) => encode_map!(v, Str, U8),
            ABIParam::StrI8Map(v) => encode_map!(v, Str, I8),
            ABIParam::StrU16Map(v) => encode_map!(v, Str, U16),
            ABIParam::StrI16Map(v) => encode_map!(v, Str, I16),
            ABIParam::StrU32Map(v) => encode_map!(v, Str, U32),
            ABIParam::StrI32Map(v) => encode_map!(v, Str, I32),
            ABIParam::StrU64Map(v) => encode_map!(v, Str, U64),
            ABIParam::StrI64Map(v) => encode_map!(v, Str, I64),
            ABIParam::StrU128Map(v) => encode_map!(v, Str, U128),
            ABIParam::StrI128Map(v) => encode_map!(v, Str, I128),
            ABIParam::StrU256Map(v) => encode_map!(v, Str, U256),
            ABIParam::StrI256Map(v) => encode_map!(v, Str, I256),
            ABIParam::StrBoolMap(v) => encode_map!(v, Str, Bool),
            ABIParam::StrStrMap(v) => encode_map!(v, Str, Str),
            ABIParam::Parampack(v) => {
                let mut buf = buffer_starts_with_uleb128_len(v.len());
                buf.append(&mut v.clone());
                buf
            }
        }
    }

    pub fn to_param_type(&self) -> ParamType {
        match self {
            ABIParam::U8(_) => ParamType::U8,
            ABIParam::I8(_) => ParamType::I8,
            ABIParam::U16(_) => ParamType::U16,
            ABIParam::I16(_) => ParamType::I16,
            ABIParam::U32(_) => ParamType::U32,
            ABIParam::I32(_) => ParamType::I32,
            ABIParam::U64(_) => ParamType::U64,
            ABIParam::I64(_) => ParamType::I64,
            ABIParam::U128(_) => ParamType::U128,
            ABIParam::I128(_) => ParamType::I128,
            ABIParam::U256(_) => ParamType::U256,
            ABIParam::I256(_) => ParamType::I256,
            ABIParam::Bool(_) => ParamType::Bool,
            ABIParam::Str(_) => ParamType::Str,
            ABIParam::Parampack(_) => ParamType::Parampack,
            ABIParam::U8Array(_) => ParamType::U8Array,
            ABIParam::I8Array(_) => ParamType::I8Array,
            ABIParam::U16Array(_) => ParamType::U16Array,
            ABIParam::I16Array(_) => ParamType::I16Array,
            ABIParam::U32Array(_) => ParamType::U32Array,
            ABIParam::I32Array(_) => ParamType::I32Array,
            ABIParam::U64Array(_) => ParamType::U64Array,
            ABIParam::I64Array(_) => ParamType::I64Array,
            ABIParam::U128Array(_) => ParamType::U128Array,
            ABIParam::I128Array(_) => ParamType::I128Array,
            ABIParam::U256Array(_) => ParamType::U256Array,
            ABIParam::I256Array(_) => ParamType::I256Array,
            ABIParam::BoolArray(_) => ParamType::BoolArray,
            ABIParam::StrArray(_) => ParamType::StrArray,
            ABIParam::StrU8Map(_) => ParamType::StrU8Map,
            ABIParam::StrI8Map(_) => ParamType::StrI8Map,
            ABIParam::StrU16Map(_) => ParamType::StrU16Map,
            ABIParam::StrI16Map(_) => ParamType::StrI16Map,
            ABIParam::StrU32Map(_) => ParamType::StrU32Map,
            ABIParam::StrI32Map(_) => ParamType::StrI32Map,
            ABIParam::StrU64Map(_) => ParamType::StrU64Map,
            ABIParam::StrI64Map(_) => ParamType::StrI64Map,
            ABIParam::StrU128Map(_) => ParamType::StrU128Map,
            ABIParam::StrI128Map(_) => ParamType::StrI128Map,
            ABIParam::StrU256Map(_) => ParamType::StrU256Map,
            ABIParam::StrI256Map(_) => ParamType::StrI256Map,
            ABIParam::StrBoolMap(_) => ParamType::StrBoolMap,
            ABIParam::StrStrMap(_) => ParamType::StrStrMap,
        }
    }
}

/// Encodes vector of tokens into ABI compliant vector of bytes.
pub fn encode(params: &[ABIParam], version: u8) -> Bytes {
    let mut bytes = vec![version];
    for param in params {
        bytes.append(&mut param.as_bytes());
    }
    bytes
}

/// Decodes ABI compliant vector of bytes into vector of tokens described by types param.
pub fn decode(types: &[ParamType], data: &[u8]) -> anyhow::Result<Vec<ABIParam>> {
    if data.len() <= 1 && !types.is_empty() {
        return Err(anyhow!("please ensure the contract and method you're calling exist! failed to decode empty bytes."));
    }
    let mut params = vec![];
    // Skip the first byte version.
    let mut offset = 1;

    for param in types {
        let param = decode_param(param, data, &mut offset)?;
        params.push(param);
    }

    Ok(params)
}

pub fn decode_param(
    param_ty: &ParamType,
    data: &[u8],
    offset: &mut usize,
) -> anyhow::Result<ABIParam> {
    match param_ty {
        ParamType::U8 => decode_int!(data, offset, U8, u8, 1),
        ParamType::I8 => decode_int!(data, offset, I8, i8, 1),
        ParamType::U16 => decode_int!(data, offset, U16, u16, 2),
        ParamType::I16 => decode_int!(data, offset, I16, i16, 2),
        ParamType::U32 => decode_int!(data, offset, U32, u32, 4),
        ParamType::I32 => decode_int!(data, offset, I32, i32, 4),
        ParamType::U64 => decode_int!(data, offset, U64, u64, 8),
        ParamType::I64 => decode_int!(data, offset, I64, i64, 8),
        ParamType::U128 => decode_int!(data, offset, U128, u128, 16),
        ParamType::I128 => decode_int!(data, offset, I128, i128, 16),
        ParamType::U256 => {
            const SIZE: usize = 32;
            let bytes = get_bytes::<SIZE>(data, offset);
            let param = ABIParam::U256(BigInt::from_bytes_be(num_bigint::Sign::NoSign, &bytes));
            *offset += SIZE;
            Ok(param)
        }
        ParamType::I256 => {
            const SIZE: usize = 32;
            let bytes = get_bytes::<SIZE>(data, offset);
            let param = ABIParam::I256(BigInt::from_bytes_be(num_bigint::Sign::NoSign, &bytes));
            *offset += SIZE;
            Ok(param)
        }

        ParamType::Bool => {
            let param = ABIParam::Bool(data[*offset] != 0);
            *offset += 1;
            Ok(param)
        }
        ParamType::Str => {
            let (len, total_offset) = read_uleb128_len_and_offset(data, offset)?;
            let bytes = &data[total_offset..total_offset + len];
            *offset = total_offset + len;
            Ok(ABIParam::Str(String::from_utf8(bytes.to_vec())?))
        }
        ParamType::U8Array => decode_vec!(data, offset, U8, U8Array),
        ParamType::I8Array => decode_vec!(data, offset, I8, I8Array),
        ParamType::U16Array => decode_vec!(data, offset, U16, U16Array),
        ParamType::I16Array => decode_vec!(data, offset, I16, I16Array),
        ParamType::U32Array => decode_vec!(data, offset, U32, U32Array),
        ParamType::I32Array => decode_vec!(data, offset, I32, I32Array),
        ParamType::U64Array => decode_vec!(data, offset, U64, U64Array),
        ParamType::I64Array => decode_vec!(data, offset, I64, I64Array),
        ParamType::U128Array => decode_vec!(data, offset, U128, U128Array),
        ParamType::I128Array => decode_vec!(data, offset, I128, I128Array),
        ParamType::U256Array => decode_vec!(data, offset, U256, U256Array),
        ParamType::I256Array => decode_vec!(data, offset, I256, I256Array),
        ParamType::BoolArray => decode_vec!(data, offset, Bool, BoolArray),
        ParamType::StrArray => decode_vec!(data, offset, Str, StrArray),
        ParamType::StrU8Map => decode_map!(data, offset, Str, U8, StrU8Map),
        ParamType::StrI8Map => decode_map!(data, offset, Str, I8, StrI8Map),
        ParamType::StrU16Map => decode_map!(data, offset, Str, U16, StrU16Map),
        ParamType::StrI16Map => decode_map!(data, offset, Str, I16, StrI16Map),
        ParamType::StrU32Map => decode_map!(data, offset, Str, U32, StrU32Map),
        ParamType::StrI32Map => decode_map!(data, offset, Str, I32, StrI32Map),
        ParamType::StrU64Map => decode_map!(data, offset, Str, U64, StrU64Map),
        ParamType::StrI64Map => decode_map!(data, offset, Str, I64, StrI64Map),
        ParamType::StrU128Map => decode_map!(data, offset, Str, U128, StrU128Map),
        ParamType::StrI128Map => decode_map!(data, offset, Str, I128, StrI128Map),
        ParamType::StrU256Map => decode_map!(data, offset, Str, U256, StrU256Map),
        ParamType::StrI256Map => decode_map!(data, offset, Str, I256, StrI256Map),
        ParamType::StrBoolMap => decode_map!(data, offset, Str, Bool, StrBoolMap),
        ParamType::StrStrMap => decode_map!(data, offset, Str, Str, StrStrMap),
        ParamType::Parampack => unimplemented!(),
    }
}

fn get_bytes<const N: usize>(data: &[u8], offset: &usize) -> [u8; N] {
    let mut bytes: [u8; N] = [0; N];
    for i in 0..N {
        bytes[i] = data[*offset + i];
    }
    bytes
}

fn buffer_starts_with_uleb128_len(len: usize) -> Vec<u8> {
    let mut buf = [0; 5];
    let len = ULEB128::from(len as u64).write_into(&mut buf).unwrap();
    buf[..len].to_vec()
}

fn read_uleb128_len_and_offset(data: &[u8], offset: &usize) -> anyhow::Result<(usize, usize)> {
    let (len, len_offset) = match ULEB128::read_from(&data[*offset..]) {
        Ok(v) => v,
        Err(err) => {
            return Err(anyhow!("{:?}", err));
        }
    };
    let len = u64::from(len) as usize;
    let total_offset = *offset + len_offset;
    Ok((len, total_offset))
}
