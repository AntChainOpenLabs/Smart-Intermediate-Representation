// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "test.h"
#include "../qvector.h"
#include "../stdlib.h"

TEST(qvector, resize)
{
    qvector_t *vector = qvector(10, sizeof(int), 0);
    ASSERT_EQ(0, qvector_size(vector));

    int i;
    for (i = 0; i < 5; i++) {
        qvector_addlast(vector, &i);
    }
    ASSERT_EQ(5, qvector_size(vector));

    struct RuntimeContext ctx = { "", 0, 0 };

    for (i = 0; i < qvector_size(vector); i++) {
        int elem = *(int *)qvector_getat(vector, i, false, &ctx);
        ASSERT_EQ(i, elem);
    }

    qvector_resize(vector, 3);
    ASSERT_EQ(3, qvector_size(vector));

    for (i = 0; i < qvector_size(vector); i++) {
        int elem = *(int *)qvector_getat(vector, i, false, &ctx);
        ASSERT_EQ(i, elem);
    }

    qvector_free(vector);
}

TEST(qvector, basic_features)
{
    const int values[] = { 0, 1, 2 };

    qvector_t *vector = qvector(3, sizeof(int), 0);
    ASSERT_EQ(0, qvector_size(vector));

    void *data;
    struct RuntimeContext ctx = { "", 0, 0 };
    bool result = qvector_addfirst(vector, &values[0]);
    ASSERT_EQ(result, true);
    ASSERT_EQ(1, qvector_size(vector));
    data = qvector_getfirst(vector, false, &ctx);
    ASSERT_EQ(values[0], *((int *)data));

    qvector_addlast(vector, values + 2);
    ASSERT_EQ(2, qvector_size(vector));
    data = qvector_getfirst(vector, false, &ctx);
    ASSERT_EQ(values[0], *((int *)data));
    data = qvector_getlast(vector, false, &ctx);
    ASSERT_EQ(values[2], *((int *)data));

    qvector_addat(vector, 1, values + 1);
    ASSERT_EQ(3, qvector_size(vector));
    data = qvector_getfirst(vector, false, &ctx);
    ASSERT_EQ(values[0], *((int *)data));
    data = qvector_getat(vector, 1, false, &ctx);
    ASSERT_EQ(values[1], *((int *)data));
    data = qvector_getlast(vector, false, &ctx);
    ASSERT_EQ(values[2], *((int *)data));

    data = qvector_popat(vector, 1, &ctx);
    ASSERT_EQ(2, qvector_size(vector));
    ASSERT_EQ(values[1], *((int *)data));
    free(data);
    data = qvector_popfirst(vector, &ctx);
    ASSERT_EQ(1, qvector_size(vector));
    ASSERT_EQ(values[0], *((int *)data));
    free(data);
    data = qvector_poplast(vector, &ctx);
    ASSERT_EQ(0, qvector_size(vector));
    ASSERT_EQ(values[2], *((int *)data));
    free(data);
    qvector_free(vector);
}

TEST(qvector, boundary_conditions)
{
    int values[] = { 1000, 1001, 1002 };

    /*test when vector is empty*/
    qvector_t *vector = qvector(1, sizeof(int), 0);
    bool result;
    void *data;
    qvector_obj_t obj;
    struct RuntimeContext ctx = { "", 0, 0 };
    result = qvector_addat(vector, 2, &values[0]);
    ASSERT_EQ(result, false);
    data = qvector_getfirst(vector, false, &ctx);
    ASSERT_TRUE(data == NULL);
    data = qvector_getlast(vector, false, &ctx);
    ASSERT_TRUE(data == NULL);
    data = qvector_getat(vector, 2, false, &ctx);
    ASSERT_TRUE(data == NULL);
    data = qvector_popfirst(vector, &ctx);
    ASSERT_TRUE(data == NULL);
    data = qvector_poplast(vector, &ctx);
    ASSERT_TRUE(data == NULL);
    data = qvector_popat(vector, 2, &ctx);
    ASSERT_TRUE(data == NULL);
    result = qvector_removefirst(vector);
    ASSERT_EQ(result, false);
    result = qvector_removelast(vector);
    ASSERT_EQ(result, false);
    result = qvector_removeat(vector, 2);
    ASSERT_EQ(result, false);
    result = qvector_getnext(vector, NULL, false);
    ASSERT_EQ(result, false);
    __memset((void *)&obj, 0, sizeof(obj));
    result = qvector_getnext(vector, &obj, false);
    ASSERT_EQ(result, false);
    qvector_free(vector);

    /*test when vector contains 1 elements*/
    vector = qvector(1, sizeof(int), 0);
    qvector_addfirst(vector, &values[0]);

    data = qvector_getfirst(vector, false, &ctx);
    ASSERT_EQ(values[0], *((int *)data));
    data = qvector_getlast(vector, false, &ctx);
    ASSERT_EQ(values[0], *((int *)data));
    data = qvector_getat(vector, 2, false, &ctx);
    ASSERT_TRUE(data == NULL);
    qvector_setat(vector, 0, &values[2], &ctx);
    data = qvector_getfirst(vector, false, &ctx);
    ASSERT_EQ(values[2], *((int *)data));
    qvector_setfirst(vector, &values[1], &ctx);
    data = qvector_getfirst(vector, false, &ctx);
    ASSERT_EQ(values[1], *((int *)data));
    qvector_setlast(vector, &values[2], &ctx);
    data = qvector_getfirst(vector, false, &ctx);
    ASSERT_EQ(values[2], *((int *)data));
    data = qvector_popfirst(vector, &ctx);
    ASSERT_EQ(values[2], *((int *)data));
    free(data);
    ASSERT_EQ(0, qvector_size(vector));

    qvector_addfirst(vector, &values[0]);
    data = qvector_popat(vector, 2, &ctx);
    ASSERT_TRUE(data == NULL);
    data = qvector_popat(vector, 0, &ctx);
    ASSERT_EQ(values[0], *((int *)data));
    free(data);

    qvector_addfirst(vector, &values[0]);
    result = qvector_removefirst(vector);
    ASSERT_EQ(result, true);
    ASSERT_EQ(0, qvector_size(vector));

    qvector_addfirst(vector, &values[0]);
    result = qvector_removeat(vector, 2);
    ASSERT_EQ(result, false);
    result = qvector_removeat(vector, 0);
    ASSERT_EQ(result, true);
    ASSERT_EQ(0, qvector_size(vector));

    qvector_addfirst(vector, &values[0]);
    qvector_reverse(vector);
    data = qvector_getfirst(vector, false, &ctx);
    ASSERT_EQ(values[0], *((int *)data));
    data = qvector_popfirst(vector, &ctx);
    ASSERT_EQ(values[0], *((int *)data));
    free(data);
    ASSERT_EQ(0, qvector_size(vector));

    qvector_addfirst(vector, &values[0]);
    data = qvector_popat(vector, 2, &ctx);
    ASSERT_TRUE(data == NULL);
    data = qvector_popat(vector, 0, &ctx);
    ASSERT_EQ(values[0], *((int *)data));
    free(data);

    qvector_addfirst(vector, &values[0]);
    result = qvector_removefirst(vector);
    ASSERT_EQ(result, true);
    ASSERT_EQ(0, qvector_size(vector));

    qvector_addfirst(vector, &values[0]);
    result = qvector_removeat(vector, 2);
    ASSERT_EQ(result, false);
    result = qvector_removeat(vector, 0);
    ASSERT_EQ(result, true);
    ASSERT_EQ(0, qvector_size(vector));

    /*test when add NULL element into vector*/
    vector = qvector(1, sizeof(int), 0);
    result = qvector_addfirst(vector, 0);
    ASSERT_EQ(result, false);
    qvector_free(vector);
}

void
test_thousands_of_values(int num_values, int options, char *prefix,
                         char *postfix)
{
    struct test_obj {
        char *prefix;
        int value;
        char *postfix;
    };

    qvector_t *vector = qvector(0, sizeof(struct test_obj), options);
    ASSERT_EQ(0, qvector_size(vector));

    int i;
    struct test_obj obj_value;
    for (i = 0; i < num_values; i++) {
        obj_value.prefix = strdup(prefix);
        obj_value.postfix = strdup(postfix);
        obj_value.value = i;

        bool result = qvector_addlast(vector, &obj_value);
        ASSERT_EQ(result, true);
        ASSERT_EQ(i + 1, qvector_size(vector));
    }

    /*test iteration*/
    qvector_obj_t obj;
    __memset((void *)&obj, 0, sizeof(obj));
    i = 0;
    while (qvector_getnext(vector, &obj, true)) {
        struct test_obj value;
        value.prefix = strdup(prefix);
        value.postfix = strdup(postfix);
        value.value = i;
        ASSERT_STREQ(((struct test_obj *)obj.data)->prefix, value.prefix);
        ASSERT_EQ(((struct test_obj *)obj.data)->value, value.value);
        ASSERT_STREQ(((struct test_obj *)obj.data)->postfix, value.postfix);

        free(value.prefix);
        free(value.postfix);
        free(obj.data);
        i++;
    }

    /*test reverse()*/
    qvector_reverse(vector);
    i = num_values - 1;
    __memset((void *)&obj, 0, sizeof(obj));
    while (qvector_getnext(vector, &obj, false)) {
        struct test_obj value;
        value.prefix = strdup(prefix);
        value.postfix = strdup(postfix);
        value.value = i;

        ASSERT_STREQ(((struct test_obj *)obj.data)->prefix, value.prefix);
        ASSERT_EQ(((struct test_obj *)obj.data)->value, value.value);
        ASSERT_STREQ(((struct test_obj *)obj.data)->postfix, value.postfix);

        free(value.prefix);
        free(value.postfix);

        i--;
    }

    /*test toarray()*/
    qvector_reverse(vector);
    void *to_array = qvector_toarray(vector, NULL);
    i = 0;
    __memset((void *)&obj, 0, sizeof(obj));
    while (qvector_getnext(vector, &obj, false)) {
        struct test_obj value;
        value.prefix = strdup(prefix);
        value.postfix = strdup(postfix);
        value.value = i;

        ASSERT_STREQ(((struct test_obj *)obj.data)->prefix, value.prefix);
        ASSERT_EQ(((struct test_obj *)obj.data)->value, value.value);
        ASSERT_STREQ(((struct test_obj *)obj.data)->postfix, value.postfix);

        free(value.prefix);
        free(value.postfix);
        free(((struct test_obj *)obj.data)->prefix);
        free(((struct test_obj *)obj.data)->postfix);

        i++;
    }

    free(to_array);

    qvector_free(vector);
}

TEST(qvector, without_prefix_and_postfix)
{
    test_thousands_of_values(10000, 0, "", "");
}

TEST(qvector, with_prefix_and_without_postfix)
{
    test_thousands_of_values(
        10000, QVECTOR_RESIZE_DOUBLE,
        "1a087a6982371bbfc9d4e14ae76e05ddd784a5d9c6b0fc9e6cd715baab66b90987b2ee"
        "054764e58fc04e449dfa060a68398601b64cf470cb6f0a260ec6539866",
        "");
}

TEST(qvector, without_prefix_and_with_postfix)
{
    test_thousands_of_values(
        10000, QVECTOR_RESIZE_LINEAR, "",
        "1a087a6982371bbfc9d4e14ae76e05ddd784a5d9c6b0fc9e6cd715baab66b90987b2ee"
        "054764e58fc04e449dfa060a68398601b64cf470cb6f0a260ec6539866");
}

TEST(qvector, with_prefix_and_postfix)
{
    test_thousands_of_values(
        10000, QVECTOR_RESIZE_EXACT,
        "1a087a6982371bbfc9d4e14ae76e05ddd784a5d9c6b0fc9e6cd715baab66b90987b2ee"
        "054764e58fc04e449dfa060a68398601b64cf470cb6f0a260ec6539866",
        "1a087a6982371bbfc9d4e14ae    "
        "76e05ddd784a5d9c6b0fc9e6cd715baab66b90987b2ee054764e58fc04e449dfa060a6"
        "8398601b64cf470cb6f0a260ec6539866");
}

TEST(qvector, to_str) {
    uint8_t values[] = {72, 101, 108, 108, 111, 87, 111, 114, 108, 100};
    int len = sizeof(values) / sizeof(values[0]);
    qvector_t *res = qvector(len, sizeof(int), 0);
    ASSERT_EQ(0, qvector_size(res));

    int i;
    for (i = 0; i < len; i++) {
        auto elem = values[i];
        qvector_addlast(res, &elem);
    }

    struct RuntimeContext ctx = { "", 0, 0 };
    auto ret = qvector_to_str(res, &ctx);
    ASSERT_EQ(len, vector_len(ret));
    auto bytes = vector_bytes(ret);
    for (int i =0; i< len; i++) {
        ASSERT_EQ(values[i], bytes[i]);
    }
}

TEST(qvector, set_data) {
    int values[] = {72, 101, 108, 108, 111, 87, 111, 114, 108, 100};
    int len = 10;

    qvector_t *res = qvector(0, sizeof(int), 0);
    qvector_setdata(res, values, len);
    ASSERT_EQ(len, qvector_size(res));

    struct RuntimeContext ctx = { "", 0, 0 };
    for (int i = 0; i < len; i++) {
        void* data = qvector_getat(res, i, false, &ctx);
        ASSERT_EQ(values[i], *((int *)data));
    }

    ASSERT_EQ(qvector_setdata(res, NULL, len), false);
    ASSERT_EQ(qvector_size(res), len);

    ASSERT_EQ(qvector_setdata(res, NULL, 0), true);
    ASSERT_EQ(qvector_size(res), 0);

    
}