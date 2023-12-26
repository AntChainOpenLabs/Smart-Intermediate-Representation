// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef __CALL_LOG_C_H_
#define __CALL_LOG_C_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "./stdlib.h"
#include "./qvector.h"

extern void
ir_builtin_call_log(qvector_t *topics, qvector_t *desc);

extern void
ir_builtin_print(struct vector *str);

extern void
log(int32_t topics, uint32_t topics_d1_length, uint32_t topics_length,
    uint32_t desc, uint32_t desc_length);

extern void
println(uint32_t src, uint32_t len);

#ifdef __cplusplus
}

#endif
#endif // __CALL_LOG_C_H_
