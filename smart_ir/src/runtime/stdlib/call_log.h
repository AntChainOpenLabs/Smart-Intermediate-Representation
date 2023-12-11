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

#ifdef __cplusplus
}

#endif
#endif // __CALL_LOG_C_H_
