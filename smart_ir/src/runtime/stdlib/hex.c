// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./hex.h"
#include "./internal/hex/hex.c"

struct vector *
ir_builtin_encode_hex(const qvector_t *in)
{
    struct vector *out =
        vector_new(HEX_ENCODE_OUT_SIZE(in->num) + 2, 1, NULL);
    int n = hex_encode(in->data, in->num, (char *)out->data);
    out->len = n;
    return out;
}

qvector_t *
ir_builtin_decode_hex(const struct vector *in)
{
    qvector_t *out =
        qvector(HEX_DECODE_OUT_SIZE(in->len) + 2, 1, QVECTOR_RESIZE_DOUBLE);
    int n = hex_decode((const char *)in->data, in->len,
                          (unsigned char *)out->data);
    out->num = n;
    return out;
}
