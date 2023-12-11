// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "test.h"
#include "../qstring.h"
#include "../qvector.h"
#include <string.h>

TEST(qstring, len)
{
    const char *src[] = { "hello.world.a",
                          "Mary had a little lamb",
                          "",
                          "lionXXtigerXleopard",
                          "lion::tiger::leopard",
                          "||||a||b|c",
                          "(///)",
                          "010",
                          "ir",
                          "    a  b c",
                          "你好 世界",
                          "你好啊世界啊啊👌" };
    for (int i = 0; i < sizeof(src) / sizeof(src[0]); i++) {
        int len = strlen(src[i]);
        auto data = vector_new(__strlen(src[i]), 1, (uint8_t *)src[i]);
        ASSERT_EQ(vector_len(data), len);
    }
}

TEST(qstring, vector_at)
{
    auto str1 = vector_new(5, 1, nullptr);
    const char *tmpl1 = "hello";
    memcpy(vector_bytes(str1), tmpl1, __strlen(tmpl1));

    ASSERT_EQ(vector_len(str1), 5);
    for (size_t i = 0; i < __strlen(tmpl1); i++) {
        ASSERT_EQ(vector_at(str1, i), tmpl1[i]);
    }
}

TEST(qstring, simple_count)
{
    struct test_obj {
        const char *src;
        const char *sub;
        int count;
    };

    struct test_obj obj_value[12] = { { "hello.world.a", ".", 2 },
                                      { "Mary had a little lamb", " ", 4 },
                                      { "", "X", 0 },
                                      { "lionXXtigerXleopard", "X", 3 },
                                      { "lion::tiger::leopard", "::", 2 },
                                      { "||||a||b|c", "|", 7 },
                                      { "(///)", "/", 3 },
                                      { "010", "0", 2 },
                                      { "ir", "", 3 },
                                      { "    a  b c", " ", 7 },
                                      { "你好 世界", " ", 1 },
                                      { "你好啊世界啊啊👌", "啊", 3 } };

    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *sub_str = obj_value[i].sub;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto sub = vector_new(__strlen(sub_str), 1, (uint8_t *)sub_str);
        uint32_t ret = vector_count(src, sub, 0, __strlen(data));
        ASSERT_EQ(ret, obj_value[i].count);
    }
}

TEST(qstring, range_count)
{
    struct test_obj {
        const char *src;
        const char *sub;
        int beg;
        int end;
        int count;
    };

    struct test_obj obj_value[6] = {
        { "hello.world.a", ".", 6, 12, 1 },
        { "Mary had a little lamb", " ", 6, 17, 2 },
        { "", "X", 0, 0, 0 },
        { "lion::tiger::leopard", "::", 5, 1000, 1 },
        { "aaaaa", "", 1, 3, 3 },
        { "你好啊世界啊啊👌", "啊", 6, 13, 1 }
    };

    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *sub_str = obj_value[i].sub;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto sub = vector_new(__strlen(sub_str), 1, (uint8_t *)sub_str);
        uint32_t ret =
            vector_count(src, sub, obj_value[i].beg, obj_value[i].end);
        ASSERT_EQ(ret, obj_value[i].count);
    }
}

TEST(qstring, startswith)
{
    struct test_obj {
        const char *src;
        const char *sub;
        int beg;
        int end;
        bool flag;
    };
    struct test_obj obj_value[9] = { { "hello world", "hello", 0, 11, true },
                                     { "hello world", "ha", 0, 11, false },
                                     { "", "ha", 0, 0, false },
                                     { "", "", 0, 0, true },
                                     { "aahello world", "hello", 2, 20, true },
                                     { "hello world", "he", 2, 20, false },
                                     { "", "ha", 2, 20, false },
                                     { "hello world", "hello", 0, 3, false },
                                     { "hello world", "", 0, 3, true } };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *sub_str = obj_value[i].sub;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto sub = vector_new(__strlen(sub_str), 1, (uint8_t *)sub_str);
        bool ret =
            vector_startswith(src, sub, obj_value[i].beg, obj_value[i].end);
        ASSERT_EQ(ret, obj_value[i].flag);
    }
}

TEST(qstring, endwith)
{
    struct test_obj {
        const char *src;
        const char *sub;
        int beg;
        int end;
        bool flag;
    };
    struct test_obj obj_value[9] = { { "hello world", "world", 0, 11, true },
                                     { "hello world", "dl", 0, 11, false },
                                     { "", "ha", 0, 0, false },
                                     { "", "", 0, 0, true },
                                     { "hello world", "", 0, 11, true },
                                     { "hello world", "world", 6, 11, true },
                                     { "hello worldkkk", "world", 6, 11, true },
                                     { "hello world", "dl", 6, 11, false },
                                     { "hello world", "", 2, 11, true } };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *sub_str = obj_value[i].sub;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto sub = vector_new(__strlen(sub_str), 1, (uint8_t *)sub_str);
        bool ret =
            vector_endswith(src, sub, obj_value[i].beg, obj_value[i].end);
        ASSERT_EQ(ret, obj_value[i].flag);
    }
}

TEST(qstring, lower)
{
    struct test_obj {
        const char *src;
        const char *dst;
    };
    struct test_obj obj_value[5] = { { "    A  B C", "    a  b c" },
                                     { "    a  b c", "    a  b c" },
                                     { "HELLO WORLD", "hello world" },
                                     { "HELlO wOrLD", "hello world" },
                                     { "    ", "    " } };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *dst = obj_value[i].dst;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto ret = vector_lower(src);
        ASSERT_STREQ((const char *)vector_bytes(ret), dst);
    }
    const char *data = "";
    auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
    auto ret = vector_lower(src);
    ASSERT_TRUE(vector_bytes(ret) == NULL);
}

TEST(qstring, upper)
{
    struct test_obj {
        const char *src;
        const char *dst;
    };
    struct test_obj obj_value[5] = { { "    a  b c", "    A  B C" },
                                     { "    a  b c", "    A  B C" },
                                     { "hello world", "HELLO WORLD" },
                                     { "HELlO wOrLD", "HELLO WORLD" },
                                     { "    ", "    " } };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *dst = obj_value[i].dst;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto ret = vector_upper(src);
        ASSERT_STREQ((const char *)vector_bytes(ret), dst);
    }

    const char *data = "";
    auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
    auto ret = vector_upper(src);
    ASSERT_TRUE(vector_bytes(ret) == NULL);
}

TEST(qstring, isalnum)
{
    struct test_obj {
        const char *src;
        bool flag;
    };
    struct test_obj obj_value[10] = {
        { "    a  b c", false }, { "abc", true },         { "", false },
        { "你好", false },       { "    1  2 c", false }, { "123", true },
        { "0.123", false },      { "12abc", true },       { "Abc123", true },
        { "1A2Bc3deio", true }
    };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto ret = vector_isalnum(src);
        ASSERT_EQ(ret, obj_value[i].flag);
    }
}

TEST(qstring, isalpha)
{
    struct test_obj {
        const char *src;
        bool flag;
    };
    struct test_obj obj_value[11] = {
        { "    a  b c", false }, { "abc", true },
        { "", false },           { "Abc", true },
        { "你好", false },       { "    1  2 c", false },
        { "123", false },        { "0.123", false },
        { "12abc", false },      { "Abc123", false },
        { "1A2Bc3deio", false }
    };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto ret = vector_isalpha(src);
        ASSERT_EQ(ret, obj_value[i].flag);
    }
}

TEST(qstring, isdigit)
{
    struct test_obj {
        const char *src;
        bool flag;
    };
    struct test_obj obj_value[11] = {
        { "    a  b c", false }, { "abc", false },
        { "", false },           { "Abc", false },
        { "你好", false },       { "    1  2 c", false },
        { "123", true },         { "0.123", false },
        { "12abc", false },      { "Abc123", false },
        { "1A2Bc3deio", false }
    };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto ret = vector_isdigit(src);
        ASSERT_EQ(ret, obj_value[i].flag);
    }
}

TEST(qstring, islower)
{
    struct test_obj {
        const char *src;
        bool flag;
    };
    struct test_obj obj_value[9] = {
        { "    a  b c", true },   { "abcd", true },
        { "123456789", false },   { "你好", false },
        { "    1  2 3", false },  { "h@110 w0rl3", true },
        { "HELLO WORLD", false }, { "", false },
        { "   ", false }
    };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto ret = vector_islower(src);
        ASSERT_EQ(ret, obj_value[i].flag);
    }
}

TEST(qstring, isupper)
{
    struct test_obj {
        const char *src;
        bool flag;
    };
    struct test_obj obj_value[9] = {
        { "    a  b c", false }, { "ABCD", true },
        { "123456789", false },  { "你好", false },
        { "    1  2 3", false }, { "H@110 W0RL3", true },
        { "HELLO WORLD", true }, { "", false },
        { "   ", false }
    };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto ret = vector_isupper(src);
        ASSERT_EQ(ret, obj_value[i].flag);
    }
}

TEST(qstring, isspace)
{
    struct test_obj {
        const char *src;
        bool flag;
    };
    struct test_obj obj_value[6] = { { "    a  b c", false },
                                     { "    1  2 3", false },
                                     { "   \n\t\r\r\r\x09\x0b", true },
                                     { "HELLO WORLD", false },
                                     { "", false },
                                     { "   ", true } };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto ret = vector_isspace(src);
        ASSERT_EQ(ret, obj_value[i].flag);
    }
}

TEST(qstring, lstrip)
{
    struct test_obj {
        const char *src;
        const char *regxp;
        const char *dst;
    };
    struct test_obj obj_value[2] = { { "   hello   ", " ", "hello   " },
                                     { "xyzzyhelloxyzzy", "xyz",
                                       "helloxyzzy" } };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *regxp = obj_value[i].regxp;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto sub = vector_new(__strlen(regxp), 1, (uint8_t *)regxp);
        auto ret = vector_lstrip(src, sub);
        ASSERT_STREQ((const char *)vector_bytes(ret), obj_value[i].dst);
    }
}

TEST(qstring, rstrip)
{
    struct test_obj {
        const char *src;
        const char *regxp;
        const char *dst;
    };
    struct test_obj obj_value[2] = { { "   hello   ", " ", "   hello" },
                                     { "xyzzyhelloxyzzy", "xyz",
                                       "xyzzyhello" } };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *regxp = obj_value[i].regxp;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto sub = vector_new(__strlen(regxp), 1, (uint8_t *)regxp);
        auto ret = vector_rstrip(src, sub);
        ASSERT_STREQ((const char *)vector_bytes(ret), obj_value[i].dst);
    }
}

TEST(qstring, strip)
{
    struct test_obj {
        const char *src;
        const char *regxp;
        const char *dst;
    };
    struct test_obj obj_value[3] = { { "   hello   ", " ", "hello" },
                                     { "mississippi", "mississippi", "" },
                                     { "mississippi", "i", "mississipp" } };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *regxp = obj_value[i].regxp;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto sub = vector_new(__strlen(regxp), 1, (uint8_t *)regxp);
        auto ret = vector_strip(src, sub);
        if (__strlen(obj_value[i].dst) == 0) {
            ASSERT_TRUE(vector_bytes(ret) == NULL);
        }
        else {
            ASSERT_STREQ((const char *)vector_bytes(ret), obj_value[i].dst);
        }
    }
}

TEST(qstring, replace)
{
    struct test_obj {
        const char *src;
        const char *old_chs;
        const char *new_chs;
        int count;
        const char *result;
    };
    struct test_obj obj_value[9] = { { "", "", "", 100, "" },
                                     { "", "", "A", 100, "A" },
                                     { "AA", "", "*-", 3, "*-A*-A*-" },
                                     { "AAAAAAAAAA", "A", "", 100, "" },
                                     { "spam, spam, eggs and spam", "spam",
                                       "ham", 100, "ham, ham, eggs and ham" },
                                     { "spam, spam, eggs and spam", "spam",
                                       "ham", -1, "ham, ham, eggs and ham" },
                                     { "spam, spam, eggs and spam", "spam",
                                       "ham", 2, "ham, ham, eggs and spam" },
                                     { "spam, spam, eggs and spam", "spam",
                                       "ham", 1, "ham, spam, eggs and spam" },
                                     { "spam, spam, eggs and spam", "spam",
                                       "ham", 0,
                                       "spam, spam, eggs and spam" } };

    struct RuntimeContext ctx = { "", 0, 0 };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        auto src = vector_new(__strlen(obj_value[i].src), 1,
                              (uint8_t *)obj_value[i].src);
        auto old_chs = vector_new(__strlen(obj_value[i].old_chs), 1,
                                  (uint8_t *)obj_value[i].old_chs);
        auto new_chs = vector_new(__strlen(obj_value[i].new_chs), 1,
                                  (uint8_t *)obj_value[i].new_chs);
        auto ret =
            vector_replace(src, old_chs, new_chs, obj_value[i].count, &ctx);
        if (__strlen(obj_value[i].result) == 0) {
            ASSERT_TRUE(vector_bytes(ret) == NULL);
        }
        else {
            ASSERT_STREQ((const char *)vector_bytes(ret), obj_value[i].result);
        }
    }
}

TEST(qstring, split)
{
    const char *data = "hello.world.a";
    const char *regex_str = ".";
    struct RuntimeContext ctx = { "", 0, 0 };
    auto src = vector_new(__strlen(data), 1, nullptr);
    memcpy(vector_bytes(src), data, __strlen(data));
    auto regex = vector_new(__strlen(regex_str), 1, nullptr);
    memcpy(vector_bytes(regex), regex_str, __strlen(regex_str));

    auto actual = vector_split(src, regex, &ctx);
    const char *expect[] = { "hello", "world", "a" };
    auto len = sizeof(expect) / sizeof(expect[0]);
    ASSERT_EQ((unsigned long)qvector_size(actual), len);
}

TEST(qstring, at)
{
    const char *data = "hello world";
    auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
    int len = strlen(data);
    for (int i = 0; i < len; i++) {
        uint8_t expect = data[i];
        uint8_t actual = vector_at(src, i);
        ASSERT_EQ(actual, expect);
    }

    for (int i = 1; i <= len; i++) {
        int index = 0 - i;
        uint8_t expect = data[len + index];
        uint8_t actual = vector_at(src, index);
        ASSERT_EQ(actual, expect);
    }
}

TEST(qstring, append)
{
    struct test_obj {
        const char *left;
        const char *right;
        const char *result;
    };

    struct test_obj obj_value[6] = {
        { "hello", "world", "helloworld" }, { "hello", "", "hello" },
        { "", "world", "world" },           { "hello", "    ", "hello    " },
        { "    ", "hello", "    hello" },   { "", "", "" }
    };

    struct RuntimeContext ctx = { "", 0, 0 };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        auto left = obj_value[i].left;
        auto right = obj_value[i].right;
        auto left_src = vector_new(__strlen(left), 1, (uint8_t *)left);
        auto right_src = vector_new(__strlen(right), 1, (uint8_t *)right);
        vector_append(left_src, right_src, &ctx);
        if (strlen(obj_value[i].result) == 0) {
            ASSERT_TRUE(vector_bytes(left_src) == NULL);
        }
        else {
            ASSERT_STREQ((const char *)vector_bytes(left_src),
                         obj_value[i].result);
        }
    }
}

TEST(qstring, find)
{
    struct test_obj {
        const char *src;
        const char *sub;
        int beg;
        int end;
        int index;
    };
    struct test_obj obj_value[9] = {
        { "World", "W", 0, 4, 0 },     { "World", "W", 2, 4, 5 },
        { "World", "W", 0, 10, 0 },    { "World", "d", 3, 10, 4 },
        { "World", "w", 0, 4, 5 },     { "World", "or", 0, 4, 1 },
        { "World", "or", -10, 10, 1 }, { "", "or", 0, 4, 0 },
        { "", "or", 2, 4, 0 }
    };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *sub_str = obj_value[i].sub;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto sub = vector_new(__strlen(sub_str), 1, (uint8_t *)sub_str);
        int32_t ret = vector_find(src, sub, obj_value[i].beg, obj_value[i].end);
        ASSERT_EQ(ret, obj_value[i].index);
    }
}

TEST(qstring, substr)
{
    struct test_obj {
        const char *src;
        int beg;
        int end;
        const char *result;
    };
    struct test_obj obj_value[6] = {
        { "HelloWorld", 0, 10, "HelloWorld" },
        { "HelloWorld", 5, 10, "World" },
        { "HelloWorld", 10, 20, "" },
        { "HelloWorld", -10, 20, "HelloWorld" },
        { "", -10, 20, "" },
        { "", 2, 5, "" },
    };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto ret = vector_substr(src, obj_value[i].beg, obj_value[i].end);
        if (strlen(obj_value[i].result) == 0) {
            ASSERT_TRUE(vector_bytes(ret) == NULL);
        }
        else {
            ASSERT_STREQ((const char *)vector_bytes(ret), obj_value[i].result);
        }
    }
}

TEST(qstring, insert)
{
    struct test_obj {
        const char *src;
        const char *sub;
        int index;
        const char *result;
    };
    struct test_obj obj_value[7] = {
        { "HelloWorld", "Hi,", 0, "Hi,HelloWorld" },
        { "HelloWorld", ":aaaa", 10, "HelloWorld:aaaa" },
        { "HelloWorld", ":aaaa:", 5, "Hello:aaaa:World" },
        { "HelloWorld", "", 0, "HelloWorld" },
        { "HelloWorld", ":aaaa", -10, ":aaaaHelloWorld" },
        { "", ":aaaa", -10, ":aaaa" },
        { "", ":aaaa", 10, ":aaaa" }
    };
    struct RuntimeContext ctx = { "", 0, 0 };
    for (int i = 0; i < sizeof(obj_value) / sizeof(obj_value[0]); i++) {
        const char *data = obj_value[i].src;
        const char *sub_str = obj_value[i].sub;
        auto src = vector_new(__strlen(data), 1, (uint8_t *)data);
        auto sub = vector_new(__strlen(sub_str), 1, (uint8_t *)sub_str);
        auto ret = vector_insert(src, sub, obj_value[i].index, &ctx);
        ASSERT_STREQ((const char *)vector_bytes(ret), obj_value[i].result);
    }
}

TEST(qstring, to_bytes) {

    const char *tmpl1 = "hello";
    auto str1 = vector_new(strlen(tmpl1), 1, (uint8_t*)tmpl1);

    ASSERT_EQ(vector_len(str1), 5);
    auto bytes = vector_to_bytes(str1);
    struct RuntimeContext ctx = { "", 0, 0 };
    for (size_t i = 0; i < __strlen(tmpl1); i++) {
        auto t = qvector_getat(bytes, i, false, &ctx);
        ASSERT_EQ(tmpl1[i], *(uint8_t*)t);
    }

}