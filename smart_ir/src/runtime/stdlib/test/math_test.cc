// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "test.h"
#include "../math.h"
#include "../stdlib.h"
#include <math.h>
#include <time.h>

TEST(math, pow) {
    for (int i = 0; i < 255; i++) {
        uint8_t actual = ir_builtin_pow_u8(2, i);
        uint8_t expect = pow(2, i);
        ASSERT_EQ(actual % 255, expect % 255);
    }

    for (int i = 0; i < 65535; i++) {
        uint16_t actual = ir_builtin_pow_u16(2, i);
        uint16_t expect = pow(2, i);
        ASSERT_EQ(actual % 65535, expect % 65535);
    }

    srand((unsigned int) time(NULL));
    uint32_t min = 0;
    uint32_t max = 100000;
    for (uint32_t i = 0; i < max; i++) {
        uint32_t a = rand();
        uint32_t exp = a % (max - min + 1) + min;
        uint32_t actual = ir_builtin_pow_u32(2, exp);
        uint32_t expect = pow(2, exp);
        ASSERT_EQ(actual % 65535, expect % 65535);
    }

    srand((unsigned int) time(NULL));
    uint64_t min_u64 = 0;
    uint64_t max_u64 = 1000000;
    for (int i = 0; i < max_u64; i++) {
        uint64_t a = rand();
        uint64_t exp = a % (max_u64 - min_u64 + 1) + min_u64;
        long double expect_double = pow(2, (long double) exp);
        uint64_t expect = (uint64_t) expect_double;
        if (isinf(expect_double)) {
            continue;
        }
        uint64_t actual = ir_builtin_pow_u64(2, exp);
        ASSERT_EQ(actual % 65535, expect % 65535);
    }
}

static char *ir_str_to_c_str(struct vector *ir_str) {
    if (ir_str->len == 0) {
        return "";
    }
    char *str = (char *) malloc(ir_str->len + 1);
    memcpy(str, ir_str->data, ir_str->len);
    str[ir_str->len] = '\0';
    return str;
}

TEST(math, str_to_integer) {
    struct int128_str_pair {
        __int128_t value;
        const char *str;
    };
    {
        // i128 <-> str tests
        int128_str_pair nums[] = {
                {
                        0,                                                        "0"
                },
                {
                        123,                                                      "123"
                },
                {
                        -123,                                                     "-123"
                },
                {
                        123456789012345678L,                                      "123456789012345678"
                },
                {
                        -123456789012345678L,                                     "-123456789012345678"
                },
                {
                        ((__int128_t) 123456789012345678L) * 123456789012345678L, "15241578753238836527968299765279684"
                },
                {
                        ((__int128_t) -123456789012345678L) * 123456789012345678L,
                                                                                  "-15241578753238836527968299765279684"
                },
                // INT128_MAX
                {
                        1 + ((__int128_t) 4) * 9223372036854775807L +
                        ((__int128_t) 2) * 9223372036854775807L * 9223372036854775807L,
                                                                                  "170141183460469231731687303715884105727"
                },
                // INT128_MIN + 1
                {
                        -1 - ((__int128_t) 4) * 9223372036854775807L -
                        ((__int128_t) 2) * 9223372036854775807L * 9223372036854775807L,
                                                                                  "-170141183460469231731687303715884105727"
                },
                // INT128_MIN
                {
                        -2 - ((__int128_t) 4) * 9223372036854775807L -
                        ((__int128_t) 2) * 9223372036854775807L * 9223372036854775807L,
                                                                                  "-170141183460469231731687303715884105728"
                },
        };
        size_t count = sizeof(nums) / sizeof(int128_str_pair);
        for (size_t i = 0; i < count; i++) {
            int128_str_pair p = nums[i];
            struct vector *generated_str = ir_builtin_i128_to_str(p.value, 10);
            printf("num %s generated str %s by ir itoa\n", p.str, ir_str_to_c_str(generated_str));
            ASSERT_TRUE(0 == memcmp(generated_str->data, p.str, strlen(p.str)));
            __int128_t generated_num = ir_builtin_str_to_i128(generated_str);
            if (generated_num != p.value) {
                printf("invalid ir stoi\n");
            }
            ASSERT_TRUE(generated_num == p.value);
        }
    }

    struct uint128_str_pair {
        __uint128_t value;
        const char *str;
    };
    {
        // u128 <-> str tests
        uint128_str_pair nums[] = {
                {
                        0,                                                         "0"
                },
                {
                        123,                                                       "123"
                },
                {
                        123456789012345678L,                                       "123456789012345678"
                },
                {
                        ((__uint128_t) 123456789012345678L) * 123456789012345678L, "15241578753238836527968299765279684"
                },
                // UINT128_MAX
                {
                        3 + ((__uint128_t) 8) * 9223372036854775807L +
                        ((__uint128_t) 4) * 9223372036854775807L * 9223372036854775807L,
                                                                                   "340282366920938463463374607431768211455"
                },
        };
        size_t count = sizeof(nums) / sizeof(uint128_str_pair);
        for (size_t i = 0; i < count; i++) {
            uint128_str_pair p = nums[i];
            struct vector *generated_str = ir_builtin_u128_to_str(p.value, 10);
            printf("num %s generated str %s by ir itoa\n", p.str, ir_str_to_c_str(generated_str));
            ASSERT_TRUE(0 == memcmp(generated_str->data, p.str, strlen(p.str)));
            __uint128_t generated_num = ir_builtin_str_to_u128(generated_str);
            if (generated_num != p.value) {
                printf("invalid ir stoi\n");
            }
            ASSERT_TRUE(generated_num == p.value);
        }
    }
}
