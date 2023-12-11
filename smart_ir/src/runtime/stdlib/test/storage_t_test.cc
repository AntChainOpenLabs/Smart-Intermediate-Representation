// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "test.h"
#include "../storage_t.h"
#include "../qvector.h"

static uint32_t zero_hints[1] = {0};
static uint32_t zero_hints_len = 1;

static uint32_t one_hints[1] = {1};
static uint32_t one_hints_len = 1;

static uint32_t one_zero_hints[2] = {1, 0};
static uint32_t one_zero_hints_len = 2;

// path1 has no mut_comps
TEST(storage_t, storage_path_join)
{
    builtin_init_storage_path(0,0,0);
    // path1 comps
    uint8_t *comps[] = { (uint8_t *)"path1_immut0",
                               (uint8_t *)"path1_immut1",
                               (uint8_t *)"path1_immut2",
                               (uint8_t *)"path1_immut3"
                               // no mut paths
                                };
    uint32_t path1_hints[] = { (uint32_t) 1, (uint32_t)0 };
    uint32_t comps_d1_length = sizeof(comps) / sizeof(uint8_t *);
    uint32_t comps_d2_length[comps_d1_length];
    for (int i = 0; i < comps_d1_length; i++) {
        comps_d2_length[i] = strlen((char *)comps[i]);
    }

    struct storage_path *path1 = storage_t_path_ptr(
        (uint8_t **)&comps, comps_d1_length,
        (uint32_t *)&comps_d2_length,
        path1_hints, sizeof(path1_hints) / sizeof(uint32_t));

    // path2 comps
    uint8_t *comps2[] = { (uint8_t *)"path2_immut0",
                                (uint8_t *)"path2_immut1",
                                // mut positions
                                (uint8_t *)"path2_mut0"
                                 };
    uint32_t path2_hints[] = { (uint32_t) 1, (uint32_t)0 };                            
    uint32_t comps_d1_length2 = sizeof(comps2) / sizeof(uint8_t *);
    uint32_t comps_d2_length2[comps_d1_length2];
    for (int i = 0; i < comps_d1_length2; i++) {
        comps_d2_length2[i] = strlen((char *)comps2[i]);
    }

    struct storage_path *path2 = storage_t_path_ptr(
        (uint8_t **)&comps2, comps_d1_length2,
        (uint32_t *)&comps_d2_length2,
        path2_hints, sizeof(path2_hints) / sizeof(uint32_t));

    // join_paths
    struct storage_path *join_path = storage_path_join(path1, path2, false);

    uint8_t *join_comps[] = {
        (uint8_t *)"path1_immut0", (uint8_t *)"path1_immut1",
        (uint8_t *)"path1_immut2", (uint8_t *)"path1_immut3",
        (uint8_t *)"path2_immut0", (uint8_t *)"path2_immut1",
        // mut positions
        (uint8_t *)"path2_mut0"
    };
    uint32_t join_comps_d1_length =
        path1->comps_d1_length + path2->comps_d1_length;
    uint32_t join_comps_d2_length[join_comps_d1_length];
    for (int i = 0; i < path1->comps_d1_length; i++) {
        join_comps_d2_length[i] = path1->comps_d2_length.u32_p[i];
    }
    for (int i = path1->comps_d1_length; i < join_comps_d1_length;
         i++) {
        join_comps_d2_length[i] =
            path2->comps_d2_length.u32_p[i - path1->comps_d1_length];
    }

    // verify
    ASSERT_EQ(join_path->comps_d1_length, join_comps_d1_length);
    for (int i = 0; i < join_comps_d1_length; i++) {
        ASSERT_STREQ((char *)join_path->comps.u8_pp[i],
                     (char *)join_comps[i]);
        ASSERT_EQ(join_path->comps_d2_length.u32_p[i],
                  join_comps_d2_length[i]);
    }
    // TODO: verify result hints
}

// path1 has mut_comps
TEST(storage_t, storage_path_join_2)
{
    builtin_init_storage_path(0,0,0);
    // path1 comps
    uint8_t *comps[] = { (uint8_t *)"path1_immut0",
                               (uint8_t *)"path1_immut1",
                               (uint8_t *)"path1_immut2",
                               (uint8_t *)"path1_immut3",
                               // mut positions
                               (uint8_t *)"path1_mut0", (uint8_t *)"path1_mut1"
                                };
    uint32_t comps_d1_length = sizeof(comps) / sizeof(uint8_t *);
    uint32_t comps_d2_length[comps_d1_length];
    for (int i = 0; i < comps_d1_length; i++) {
        comps_d2_length[i] = strlen((char *)comps[i]);
    }
    
    struct storage_path *path1 = storage_t_path_ptr(
        (uint8_t **)&comps, comps_d1_length,
        (uint32_t *)&comps_d2_length,
        one_zero_hints, one_zero_hints_len);

    // path2 comps
    uint8_t *comps2[] = { (uint8_t *)"path2_immut0",
                                (uint8_t *)"path2_immut1",
                                // mut positions
                                (uint8_t *)"path2_mut0"
                                 };
    uint32_t comps_d1_length2 = sizeof(comps2) / sizeof(uint8_t *);
    uint32_t comps_d2_length2[comps_d1_length2];
    for (int i = 0; i < comps_d1_length2; i++) {
        comps_d2_length2[i] = strlen((char *)comps2[i]);
    }

    struct storage_path *path2 = storage_t_path_ptr(
        (uint8_t **)&comps2, comps_d1_length2,
        (uint32_t *)&comps_d2_length2,
        one_zero_hints, one_zero_hints_len);

    // join_paths
    struct storage_path *join_path = storage_path_join(path1, path2, false);

    uint8_t *join_comps[] = { (uint8_t *)"path1_immut0",
                               (uint8_t *)"path1_immut1",
                               (uint8_t *)"path1_immut2",
                               (uint8_t *)"path1_immut3",
                               // mut positions
                               (uint8_t *)"path1_mut0",
                                  (uint8_t *)"path1_mut1",
                                  (uint8_t *)"path2_immut0",
                                  (uint8_t *)"path2_immut1",
                                  (uint8_t *)"path2_mut0"
                                };
    uint32_t join_comps_d1_length = path1->comps_d1_length + path2->comps_d1_length;
    uint32_t *join_comps_d2_length = (uint32_t *) malloc(sizeof(uint32_t) * join_comps_d1_length);

    for (int i = 0; i < path1->comps_d1_length; i++) {
        join_comps_d2_length[i] = path1->comps_d2_length.u32_p[i];
    }
    for (int i = 0; i < path2->comps_d1_length; i++) {
        join_comps_d2_length[i + path1->comps_d1_length] = path2->comps_d2_length.u32_p[i];
    }
    
    // verify
    ASSERT_EQ(join_path->comps_d1_length, join_comps_d1_length);
    for (int i = 0; i < join_comps_d1_length; i++) {
        ASSERT_STREQ((char *)join_path->comps.u8_pp[i],
                     (char *)join_comps[i]);
        ASSERT_EQ(join_path->comps_d2_length.u32_p[i],
                  join_comps_d2_length[i]);
    }
    // TODO: verify result hints
}
