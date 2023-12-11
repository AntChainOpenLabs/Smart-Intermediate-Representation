// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "test.h"
#include "../base64.h"
#include "../internal/base64/base64.h"
#include "../qvector.h"
#include <stdint.h>
#include <string>
#include <stdio.h>

static qvector_t *
get_bytes_from_str(const std::string &origin_str)
{
    auto origin_vec = qvector(1, 1, 2 /* QVECTOR_RESIZE_DOUBLE */);
    qvector_resize(origin_vec, origin_str.size());
    if (origin_str.size() > 0) {
        ::memcpy(origin_vec->data, origin_str.data(), origin_str.size());
    }
    origin_vec->num = origin_str.size();
    return origin_vec;
}

static std::string
string_from_ir_str(const struct vector *ir_str)
{
    std::string result_str;
    result_str.resize(ir_str->len);
    if (ir_str->len > 0) {
        ::memcpy((void *)result_str.data(), ir_str->data, ir_str->len);
    }
    return result_str;
}

static struct vector *
create_ir_str(const std::string &str)
{
    auto ir_str = vector_new(str.size(), 1, nullptr);
    if (!str.empty()) {
        ::memcpy(ir_str->data, str.data(), str.size());
    }
    return ir_str;
}

static std::string
string_from_ir_bytes(const qvector_t *ir_bytes)
{
    std::string result_str;
    result_str.resize(ir_bytes->num * ir_bytes->objsize);
    if (ir_bytes->num > 0) {
        ::memcpy((void *)result_str.data(), ir_bytes->data,
                 ir_bytes->num * ir_bytes->objsize);
    }
    return result_str;
}

TEST(base64, check_implementation)
{
    ASSERT_TRUE(check_implementation());
}

TEST(base64, base64_encode)
{
    {
        std::string source("hello");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_base64(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s base64 encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("aGVsbG8=", result_str.c_str());
    }
    {
        std::string source("1");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_base64(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s base64 encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("MQ==", result_str.c_str());
    }
    {
        std::string source("11");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_base64(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s base64 encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("MTE=", result_str.c_str());
    }
    {
        std::string source("111");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_base64(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s base64 encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("MTEx", result_str.c_str());
    }
    {
        std::string source("1111");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_base64(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s base64 encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("MTExMQ==", result_str.c_str());
    }
}

TEST(base64, base64_decode)
{
    // valid cases
    {
        std::string source("aGVsbG8=");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_base64(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s base64 decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("hello", result_str.c_str());
    }
    {
        std::string source("MQ==");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_base64(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s base64 decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("1", result_str.c_str());
    }
    {
        std::string source("MTE=");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_base64(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s base64 decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("11", result_str.c_str());
    }
    {
        std::string source("MTEx");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_base64(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s base64 decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("111", result_str.c_str());
    }
    {
        std::string source("MTExMQ==");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_base64(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s base64 decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("1111", result_str.c_str());
    }
    // invalid cases
    {
        // decode empty bytes
        std::string source("");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_base64(source_ir_str);
        ASSERT_EQ(0, result_ir_bytes->num);
    }
    {
        // decode more bytes then needed. valid
        std::string source("MTExMQ==aaaa");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_base64(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s base64 decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_EQ(0, result_ir_bytes->num);
    }
    {
        // decode invalid base64-char
        std::string source("&TExMQ==");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_base64(source_ir_str);
        ASSERT_EQ(0, result_ir_bytes->num);
    }
    {
        // decode not enough input, valid if only missing =
        std::string source("MTExMQ="); // missing one '='
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_base64(source_ir_str);
        ASSERT_EQ(4, result_ir_bytes->num);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s base64 decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("1111", result_str.c_str());
    }
}
