// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./call_log.h"

extern void
log(int32_t topics, uint32_t topics_d1_length, uint32_t topics_length,
    uint32_t desc, uint32_t desc_length);

extern void
println(uint32_t src, uint32_t len);

void
ir_builtin_call_log(qvector_t *topics, qvector_t *desc)
{
    uint32_t *topics_array = (uint32_t *)malloc(sizeof(uint32_t) * topics->num);
    uint32_t *topics_length_array =
        (uint32_t *)malloc(sizeof(uint32_t) * topics->num);
    for (int i = 0; i < topics->num; i++) {
        qvector_t *item =
            *((qvector_t **)qvector_getat(topics, i, false, NULL));
        topics_array[i] = (uint32_t)((intptr_t)(char *)item->data);
        topics_length_array[i] = item->num;
    }
    uint32_t topics_d1_length = topics->num;
    uint32_t desc_length = desc->num;

    // call hostapi
    log((int32_t)topics_array, (uint32_t)topics_d1_length,
        (uint32_t)topics_length_array, (uint32_t)desc->data,
        (uint32_t)desc_length);
}

void
ir_builtin_print(struct vector *str)
{
    println((uint32_t)str->data, str->len);
}
