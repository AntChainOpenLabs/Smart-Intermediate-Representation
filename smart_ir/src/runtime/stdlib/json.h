// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef __SSZ_C_H_
#define __SSZ_C_H_

#include "./stdlib.h"
#include "./qhashtbl.h"
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

extern struct vector *
ir_builtin_json_encode(uint32_t runtime_class_offset, void *val);

extern void*
ir_builtin_json_decode(uint32_t runtime_class_offset, struct vector *val);

#ifdef __cplusplus
}
#endif
#endif // __SSZ_C_H_
