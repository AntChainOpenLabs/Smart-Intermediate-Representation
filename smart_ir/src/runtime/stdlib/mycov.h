// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#ifndef __MYCOV_H_
#define __MYCOV_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "./stdlib.h"
#include "./qvector.h"

extern void
ir_builtin_add_coverage_counter(int32_t bb_id);

extern qvector_t *
ir_builtin_get_coverage_counters(struct RuntimeContext *ctx);

extern void
ir_builtin_call_coverage_log(struct RuntimeContext *ctx, int32_t success);

#ifdef __cplusplus
}
#endif

#endif // __MYCOV_H_