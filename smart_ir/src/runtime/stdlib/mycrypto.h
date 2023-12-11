// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef __MYCRYPTO_C_H_
#define __MYCRYPTO_C_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "./stdlib.h"
#include "./qvector.h"

qvector_t *
ir_builtin_sha256(qvector_t *str);

qvector_t *
ir_builtin_sm3(qvector_t *str);

qvector_t *
ir_builtin_keccak256(qvector_t *str);

bool
ir_builtin_verify_mycrypto_signature(qvector_t *pk, qvector_t *sign,
                                        qvector_t *digest);

qvector_t *
ir_builtin_eth_secp256k1_recovery(qvector_t *hash, uint8_t v,
                                     qvector_t *r, qvector_t *s);

#ifdef __cplusplus
}

#endif
#endif // __MYCRYPTO_C_H_
