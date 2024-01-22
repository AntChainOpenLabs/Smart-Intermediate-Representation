// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./storage_t.h"
#include "./qvector.h"

#define STORAGE_PATH_POOL_SIZE 20
#define MAX_COMPS_LENGTH 8
#define MAX_HINTS_LENGTH 9
#define MAX_COMP_SIZE 64


void
builtin_init_storage_path(uint32_t const_storage_path_data_address,
                          uint32_t const_storage_path_offsets,
                          uint32_t const_sotrage_path_length)
{

}

extern uint32_t
build_storage_t_path_ptr(uint8_t **a, uint32_t b,
                         uint32_t *c,
                         uint32_t *d,
                         uint32_t e)
{
    return 0;
}

void storage_path_used_marker(struct storage_path dummy) {}

void
storage_store(uint32_t path_ptr, const void *value, uint32_t value_length)
{
    return;
}

int32_t
storage_read_object_length(uint32_t path_ptr)
{
    return 0;
}

void
assert_storage_array_index(struct storage_path *path)
{
    return;
}

void
storage_load(uint32_t path_ptr, const void *value)
{
    return;
}