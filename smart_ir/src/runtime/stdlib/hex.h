// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./stdlib.h"

#ifdef __cplusplus
extern "C" {
#endif

#include "./stdlib.h"
#include "./qvector.h"

struct vector *
ir_builtin_encode_hex(const qvector_t *in);

qvector_t *
ir_builtin_decode_hex(const struct vector *in);

#ifdef __cplusplus
} // end extern "C"
#endif