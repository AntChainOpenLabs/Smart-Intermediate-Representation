// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#ifndef __CHAIN_H
#define __CHAIN_H
#include "./stdlib.h"
#include "./qvector.h"

extern void
builtin_revert(int32_t err_code, struct vector *msg_str);

void
builtin_abort(const char *msg, int32_t msg_len);

extern int32_t
builtin_co_call(struct vector *contract_name, struct vector *method,
                struct vector *encoded_params);

#endif // __CHAIN_H
