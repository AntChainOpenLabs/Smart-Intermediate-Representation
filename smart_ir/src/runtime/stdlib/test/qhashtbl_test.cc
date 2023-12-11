// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "test.h"
#include "../qstring.h"
#include "../qhashtbl.h"
#include <string.h>

TEST(qhashtbl, basic_features)
{
    const char *KEYS[] = {
        "key0",
        "key1_long_key-"
        "fef6bd00f77aef990a6d62969fee0cb904d052665a1dcf10492156124fafc59769e91d"
        "1a06ec1215e435e29ef43de177f6f2a5e035860e702c82e08084950313",
    };
    const char *VALUES[] = {
        "value0",
        "value1_long_value-"
        "1a087a6982371bbfc9d4e14ae76e05ddd784a5d9c6b0fc9e6cd715baab66b90987b2ee"
        "054764e58fc04e449dfa060a68398601b64cf470cb6f0a260ec6539866",
    };

    qhashtbl_t *tbl = qhashtbl(0, IR_RUNTIME_TYPE_STR, 0);
    ASSERT_EQ(0, qhashtbl_size(tbl));

    qhashtbl_putstr(tbl, (int64_t) KEYS[0], VALUES[0]);
    ASSERT_EQ(1, qhashtbl_size(tbl));
    ASSERT_STREQ(VALUES[0], qhashtbl_getstr(tbl,  (int64_t) KEYS[0], false));

    qhashtbl_putstr(tbl, (int64_t) KEYS[1], VALUES[1]);
    ASSERT_EQ(2, qhashtbl_size(tbl));
    ASSERT_STREQ(VALUES[1], qhashtbl_getstr(tbl, (int64_t) KEYS[1], false));

    qhashtbl_remove(tbl, (int64_t)  KEYS[0]);
    ASSERT_EQ(1, qhashtbl_size(tbl));
    ASSERT_TRUE(qhashtbl_getstr(tbl, (int64_t) KEYS[0], false) == NULL);
    ASSERT_STREQ(VALUES[1], qhashtbl_getstr(tbl, (int64_t) KEYS[1], false));

    qhashtbl_clear(tbl);
    ASSERT_EQ(0, qhashtbl_size(tbl));

    qhashtbl_free(tbl);
}

TEST(qhashtbl, i8_u64_map_insert_many_test)
{
    const size_t count = 101;
    int8_t keys[count];
    for (size_t i=0; i < count; i++) {
        keys[i] = i;
    }
    uint64_t values[count];
    for (size_t i=0; i < count; i++) {
        values[i] = 10000000000L + i;
    }

    qhashtbl_t *tbl = qhashtbl(0, IR_RUNTIME_TYPE_I8, 0);
    ASSERT_EQ(0, qhashtbl_size(tbl));

    for (size_t i=0; i < count; i++) {
        qhashtbl_put(tbl, (int64_t) keys[i], &values[i], sizeof(uint64_t));
    }
    for (size_t i=0; i < count; i++) {
        void *data = qhashtbl_get(tbl, (int64_t) keys[i], NULL, false);
        int64_t value = *((int64_t*)data);
        ASSERT_EQ(value, values[i]);
    }
    qhashtbl_clear(tbl);
    ASSERT_EQ(0, qhashtbl_size(tbl));

    qhashtbl_free(tbl);
}