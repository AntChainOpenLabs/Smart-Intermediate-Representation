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

struct storage_path {
    bool a;
};

void
builtin_init_storage_path(uint32_t const_storage_path_data_address,
                          uint32_t const_storage_path_offsets,
                          uint32_t const_sotrage_path_length);

extern uint32_t
build_storage_t_path_ptr(uint8_t **a, uint32_t b,
                         uint32_t *c,
                         uint32_t *d,
                         uint32_t e);

void
storage_store(uint32_t path_ptr, const void *value, uint32_t value_length);

int32_t
storage_read_object_length(uint32_t path_ptr);

void
assert_storage_array_index(struct storage_path *path);

void
storage_load(uint32_t path_ptr, const void *value);


#ifdef __cplusplus
}

#endif
#endif // __STORAGE_T_C_H_
