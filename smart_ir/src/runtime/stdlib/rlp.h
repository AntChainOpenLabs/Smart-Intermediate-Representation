// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#ifndef __RLP_H
#define __RLP_H

#include "./stream.h"
#include "./stdlib.h"
#include "./qvector.h"

#ifdef __cplusplus
extern "C" {
#endif

extern void rlp_encode_str(ByteStream *stream, struct vector *value);

extern void rlp_encode_bytes(ByteStream *stream, qvector_t *value);

extern void rlp_encode_str_list(ByteStream *stream, qvector_t *list);

extern void rlp_encode_bytes_list(ByteStream *stream, qvector_t *list);

// return str(bytes) or list
extern qvector_t *rlp_decode(ByteStream *stream);

void common_rlp_encode(ByteStream *stream, uint32_t runtime_class_offset, void *val);

void *common_rlp_decode(ByteStream *stream, uint32_t runtime_class_offset);

extern qvector_t *
ir_builtin_rlp_encode(uint32_t runtime_class_offset, void *val);

extern void *
ir_builtin_rlp_decode(uint32_t runtime_class_offset, qvector_t *val);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // __RLP_H
