// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef __STORAGE_T_C_H_
#define __STORAGE_T_C_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "./stdlib.h"
#include "./qvector.h"

void
builtin_init_storage_path(uint32_t const_storage_path_data_address,
                          uint32_t const_storage_path_offsets,
                          uint32_t const_sotrage_path_length);

#ifdef __cplusplus
}

#endif
#endif // __STORAGE_T_C_H_
