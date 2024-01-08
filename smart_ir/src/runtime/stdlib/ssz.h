// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef __SSZ_C_H_
#define __SSZ_C_H_

#include <stddef.h>
#include <stdint.h>
#include "./qvector.h"
#include "./stdlib.h"

#ifdef __cplusplus
extern "C" {
#endif

// https://github.com/ethereum/consensus-specs/blob/dev/ssz/simple-serialize.md

#define uint128_t __uint128_t
#define int128_t __int128_t

extern int32_t
ssz_encode_bool(bool v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_bool(uint8_t *v, uint8_t *buf);

extern int32_t
ssz_encode_u8(uint8_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_u8(uint8_t *v, uint8_t *buf);

extern int32_t
ssz_encode_u16(uint16_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_u16(uint16_t *v, uint8_t *buf);

extern int32_t
ssz_encode_u32(uint32_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_u32(uint32_t *v, uint8_t *buf);

extern int32_t
ssz_encode_u64(uint64_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_u64(uint64_t *v, uint8_t *buf);

extern int32_t
ssz_encode_u128(uint128_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_u128(uint128_t *v, uint8_t *buf);

extern int32_t
ssz_encode_u256(uint256_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_u256(uint256_t *v, uint8_t *buf);

extern int32_t
ssz_encode_i8(int8_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_i8(int8_t *v, uint8_t *buf);

extern int32_t
ssz_encode_i16(int16_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_i16(int16_t *v, uint8_t *buf);

extern int32_t
ssz_encode_i32(int32_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_i32(int32_t *v, uint8_t *buf);

extern int32_t
ssz_encode_i64(int64_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_i64(int64_t *v, uint8_t *buf);

extern int32_t
ssz_encode_i128(int128_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_i128(int128_t *v, uint8_t *buf);

extern int32_t
ssz_encode_i256(int256_t v, uint8_t *buf, int32_t offset);
extern int32_t
ssz_decode_i256(int256_t *v, uint8_t *buf);

extern int32_t
ssz_encode_str(const struct vector *v, uint8_t *buf, int32_t hdr_offset,
               int32_t data_offset);
extern int32_t
ssz_decode_str(struct vector *v, uint8_t *buf, int32_t length);

extern int32_t
ssz_encode_vec(const struct vector *v, uint8_t *buf, int32_t offset,
               int32_t data_offset);
extern int32_t
ssz_decode_vec(struct vector *v, uint8_t *buf, int32_t length);

extern qvector_t *
ir_builtin_ssz_encode(uint32_t runtime_class_offset, void *val);

extern void*
ir_builtin_ssz_decode(uint32_t runtime_class_offset, qvector_t *val);

extern void*
ir_builtin_ssz_decode_impl(uint32_t runtime_class_offset, bool allow_empty_object, qvector_t *val);

extern void *
ir_builtin_ssz_decode_void_ptr(uint32_t runtime_class_offset, bool allow_empty_object,
 void *val, uint32_t data_len);

extern void*
ir_builtin_versioned_ssz_get_data_ptr(
    uint32_t data_ptr, uint32_t data_len, bool is_versioned, uint32_t ssz_version_size);

extern uint32_t
ir_builtin_versioned_ssz_get_data_len(
    uint32_t data_len, bool is_versioned, uint32_t ssz_version_size);

#ifdef __cplusplus
}
#endif
#endif // __SSZ_C_H_
