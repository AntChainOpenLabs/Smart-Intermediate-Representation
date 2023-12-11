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
ir_builtin_encode_base64(const qvector_t *in);

qvector_t *
ir_builtin_decode_base64(const struct vector *in);

#ifdef __cplusplus
} // end extern "C"
#endif
