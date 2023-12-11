// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef __DATA_STREAM_BUILTIN_C_H_
#define __DATA_STREAM_BUILTIN_C_H_

#ifdef __cplusplus
extern "C" {
#endif
#include "./stdlib.h"
#include "./qvector.h"
#include "./qhashtbl.h"

extern qvector_t *
ir_builtin_data_stream_encode_bool(bool value);
extern qvector_t *
ir_builtin_data_stream_encode_u8(uint8_t value);
extern qvector_t *
ir_builtin_data_stream_encode_u16(uint16_t value);
extern qvector_t *
ir_builtin_data_stream_encode_u32(uint32_t value);
extern qvector_t *
ir_builtin_data_stream_encode_u64(uint64_t value);
extern qvector_t *
ir_builtin_data_stream_encode_u128(__uint128_t value);
extern qvector_t *
ir_builtin_data_stream_encode_i8(int8_t value);
extern qvector_t *
ir_builtin_data_stream_encode_i16(int16_t value);
extern qvector_t *
ir_builtin_data_stream_encode_i32(int32_t value);
extern qvector_t *
ir_builtin_data_stream_encode_i64(int64_t value);
extern qvector_t *
ir_builtin_data_stream_encode_i128(__int128_t value);
extern qvector_t *
ir_builtin_data_stream_encode_str(const struct vector *value);

/* Array */
extern qvector_t *
ir_builtin_data_stream_encode_strarray(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_boolarray(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_u8array(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_u16array(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_u32array(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_u64array(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_u128array(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_i8array(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_i16array(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_i32array(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_i64array(qvector_t *v);
extern qvector_t *
ir_builtin_data_stream_encode_i128array(qvector_t *v);

/* Map */
extern qvector_t *
ir_builtin_data_stream_encode_strboolmap(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stru8map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stru16map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stru32map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stru64map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stru128map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stri8map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stri16map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stri32map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stri64map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_stri128map(qhashtbl_t *value);
extern qvector_t *
ir_builtin_data_stream_encode_strstrmap(qhashtbl_t *value);

#ifdef __cplusplus
}

#endif
#endif // __DATA_STREAM_BUILTIN_C_H_
