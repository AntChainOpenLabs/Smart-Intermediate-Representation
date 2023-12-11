// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./mycrypto.h"

extern void
sha256(uint32_t msg, uint32_t len, uint32_t value);
extern void
sm3(uint32_t msg, uint32_t len, uint32_t value);
extern void
keccak256(uint32_t msg, uint32_t len, uint32_t value);
extern uint32_t
verify_mycrypto_signature(uint32_t pk, uint32_t pk_length, uint32_t sign,
                          uint32_t sign_length, uint32_t digest,
                          uint32_t digest_length);
extern uint32_t
eth_secp256k1_recovery(uint32_t hash, uint32_t v, uint32_t r, uint32_t s, uint32_t addr);

qvector_t *
ir_builtin_sha256(qvector_t *msg)
{
    int32_t hash_size = 32;
    qvector_t *out = qvector(hash_size, 1, QVECTOR_RESIZE_DOUBLE);
    out->num = hash_size;
    // call hostapi
    sha256((uint32_t)msg->data, msg->num, (uint32_t)out->data);
    return out;
}

qvector_t *
ir_builtin_sm3(qvector_t *msg)
{
    int32_t hash_size = 32;
    qvector_t *out = qvector(hash_size, 1, QVECTOR_RESIZE_DOUBLE);
    out->num = hash_size;
    // call hostapi
    sm3((uint32_t)msg->data, msg->num, (uint32_t)out->data);
    return out;
}

qvector_t*
ir_builtin_keccak256(qvector_t *msg)
{
    int32_t hash_size = 32;
    qvector_t *out = qvector(hash_size, 1, QVECTOR_RESIZE_DOUBLE);
    out->num = hash_size;
    // call hostapi
    keccak256((uint32_t)msg->data, msg->num, (uint32_t)out->data);
    return out;
}

bool
ir_builtin_verify_mycrypto_signature(qvector_t *pk, qvector_t *sign,
                                        qvector_t *digest)
{
    // check digest length
    int32_t hash_size = 32;
    if (digest->num != hash_size) {
        char msg[] = "DigestLengthError: digest of the msg must be 32B";
        IR_ABORT(msg, sizeof(msg) - 1);
    }
    // call hostapi
    uint32_t result = verify_mycrypto_signature(
        (uint32_t)pk->data, pk->num, (uint32_t)sign->data, sign->num,
        (uint32_t)digest->data, digest->num);
    return result != 0;
}

qvector_t *
ir_builtin_eth_secp256k1_recovery(qvector_t *hash, uint8_t v,
                                     qvector_t *r, qvector_t *s)
{
    int32_t result_size = 32;
    int32_t v_size = 32;
    qvector_t *out = qvector(result_size, 1, QVECTOR_RESIZE_DOUBLE);
    qvector_t *v_input = qvector(v_size, 1, QVECTOR_RESIZE_DOUBLE);
    __memset(v_input->data, 0, v_size - 1);
    __memset(v_input->data + v_size - 1, v, 1);
    out->num = result_size;

    // eth_secp256k1_recovery receive 32bytes r and 32bytes s
    size_t r_s_expect_size = 32;
    qvector_t *r32 = r;
    if (r->num < r_s_expect_size) {
        r32 = qvector(r_s_expect_size, 1, QVECTOR_RESIZE_DOUBLE);
        __memset(r32->data, 0, r_s_expect_size - r->num);
        memcpy(r32->data + (r_s_expect_size - r->num), r->data, r->num);
    }
    qvector_t *s32 = s;
    if (s->num < r_s_expect_size) {
        s32 = qvector(r_s_expect_size, 1, QVECTOR_RESIZE_DOUBLE);
        __memset(s32->data, 0, r_s_expect_size - s->num);
        memcpy(s32->data + (r_s_expect_size - s->num), s->data, s->num);
    }

    // call hostapi
    uint32_t succ = eth_secp256k1_recovery((uint32_t)hash->data, 
                                           (uint32_t)v_input->data,
                                           (uint32_t)r32->data,
                                           (uint32_t)s32->data,
                                           (uint32_t)out->data);
    if (succ == 0) {
        char msg[] = "eth secp256k1 recovery error";
        IR_ABORT(msg, sizeof(msg) - 1);
    }

    return out;
}