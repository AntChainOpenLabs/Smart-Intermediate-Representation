// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once
#ifndef __DATASTREAM_C_H_
#define __DATASTREAM_C_H_

#include <stddef.h>
#include <stdint.h>
#include "./stdlib.h"
#include "./qvector.h"
#include "./qhashtbl.h"

#ifdef __cplusplus
extern "C" {
#endif

extern void
check_end_offset(int32_t offset, int32_t len);

extern int32_t
data_stream_encode_bool(bool v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_bool(uint8_t *v, uint8_t *buf, int32_t offset, int32_t len);
extern int32_t
data_stream_encode_u8(uint8_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_u8(uint8_t *v, uint8_t *buf, int32_t offset, int32_t len);
extern int32_t
data_stream_encode_u16(uint16_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_u16(uint16_t *v, uint8_t *buf, int32_t offset, int32_t len);
extern int32_t
data_stream_encode_u32(uint32_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_u32(uint32_t *v, uint8_t *buf, int32_t offset, int32_t len);
extern int32_t
data_stream_encode_u64(uint64_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_u64(uint64_t *v, uint8_t *buf, int32_t offset, int32_t len);
extern int32_t
data_stream_encode_u128(__uint128_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_u128(__uint128_t *v, uint8_t *buf, int32_t offset,
                        int32_t len);
extern int32_t
data_stream_encode_u256(uint256_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_u256(uint256_t *v, uint8_t *buf, int32_t offset,
                        int32_t len);
extern int32_t
data_stream_encode_i8(int8_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_i8(int8_t *v, uint8_t *buf, int32_t offset, int32_t len);
extern int32_t
data_stream_encode_i16(int16_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_i16(int16_t *v, uint8_t *buf, int32_t offset, int32_t len);
extern int32_t
data_stream_encode_i32(int32_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_i32(int32_t *v, uint8_t *buf, int32_t offset, int32_t len);
extern int32_t
data_stream_encode_i64(int64_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_i64(int64_t *v, uint8_t *buf, int32_t offset, int32_t len);
extern int32_t
data_stream_encode_i128(__int128_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_i128(__int128_t *v, uint8_t *buf, int32_t offset,
                        int32_t len);
extern int32_t
data_stream_encode_i256(int256_t v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_i256(int256_t *v, uint8_t *buf, int32_t offset,
                        int32_t len);

extern int32_t
data_stream_encode_str(const struct vector *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_str(struct vector *v, uint8_t *buf, int32_t offset,
                       int32_t len);
extern int32_t
data_stream_encode_vec(const struct vector *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_vec(struct vector *v, uint8_t *buf, int32_t offset,
                       int32_t len);

/* Array */

extern int32_t
data_stream_decode_strarray(qvector_t *v, uint8_t *buf, int32_t offset,
                            int32_t len);
extern int32_t
data_stream_encode_strarray(qvector_t *v, uint8_t *buf, int32_t offset,
                            struct RuntimeContext *ctx);
extern int32_t
data_stream_encode_boolarray(qvector_t *v, uint8_t *buf, int32_t offset,
                             struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_boolarray(qvector_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_u8array(qvector_t *v, uint8_t *buf, int32_t offset,
                           struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_u8array(qvector_t *v, uint8_t *buf, int32_t offset,
                           int32_t len);
extern int32_t
data_stream_encode_u16array(qvector_t *v, uint8_t *buf, int32_t offset,
                            struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_u16array(qvector_t *v, uint8_t *buf, int32_t offset,
                            int32_t len);
extern int32_t
data_stream_encode_u32array(qvector_t *v, uint8_t *buf, int32_t offset,
                            struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_u32array(qvector_t *v, uint8_t *buf, int32_t offset,
                            int32_t len);
extern int32_t
data_stream_encode_u64array(qvector_t *v, uint8_t *buf, int32_t offset,
                            struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_u64array(qvector_t *v, uint8_t *buf, int32_t offset,
                            int32_t len);
extern int32_t
data_stream_encode_u128array(qvector_t *v, uint8_t *buf, int32_t offset,
                             struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_u128array(qvector_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_u256array(qvector_t *v, uint8_t *buf, int32_t offset,
                             struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_u256array(qvector_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_i8array(qvector_t *v, uint8_t *buf, int32_t offset,
                           struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_i8array(qvector_t *v, uint8_t *buf, int32_t offset,
                           int32_t len);
extern int32_t
data_stream_encode_i16array(qvector_t *v, uint8_t *buf, int32_t offset,
                            struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_i16array(qvector_t *v, uint8_t *buf, int32_t offset,
                            int32_t len);
extern int32_t
data_stream_encode_i32array(qvector_t *v, uint8_t *buf, int32_t offset,
                            struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_i32array(qvector_t *v, uint8_t *buf, int32_t offset,
                            int32_t len);
extern int32_t
data_stream_encode_i64array(qvector_t *v, uint8_t *buf, int32_t offset,
                            struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_i64array(qvector_t *v, uint8_t *buf, int32_t offset,
                            int32_t len);
extern int32_t
data_stream_encode_i128array(qvector_t *v, uint8_t *buf, int32_t offset,
                             struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_i128array(qvector_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_i256array(qvector_t *v, uint8_t *buf, int32_t offset,
                             struct RuntimeContext *ctx);
extern int32_t
data_stream_decode_i256array(qvector_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);

/* Map */
extern int32_t
data_stream_encode_strboolmap(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_strboolmap(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                              int32_t len);
extern int32_t
data_stream_encode_stru8map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stru8map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                            int32_t len);
extern int32_t
data_stream_encode_stru16map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stru16map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_stru32map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stru32map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_stru64map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stru64map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_stru128map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stru128map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                              int32_t len);
extern int32_t
data_stream_encode_stru256map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stru256map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                              int32_t len);
extern int32_t
data_stream_encode_stri8map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stri8map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                            int32_t len);
extern int32_t
data_stream_encode_stri16map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stri16map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_stri32map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stri32map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_stri64map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stri64map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                             int32_t len);
extern int32_t
data_stream_encode_stri128map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stri128map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                              int32_t len);
extern int32_t
data_stream_encode_stri256map(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_stri256map(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                              int32_t len);
extern int32_t
data_stream_encode_strstrmap(qhashtbl_t *v, uint8_t *buf, int32_t offset);
extern int32_t
data_stream_decode_strstrmap(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                              int32_t len);
/* Util */

extern size_t
qhashtbl_total_space(qhashtbl_t *v);

#ifdef __cplusplus
}

#endif
#endif // __DATASTREAM_C_H_
