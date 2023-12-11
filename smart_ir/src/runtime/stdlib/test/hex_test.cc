// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "test.h"
#include "../hex.h"
#include "../internal/hex/hex.h"
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

TEST(hex, hex_encode)
{
    {
        std::string source("hello");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_hex(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s hex encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("68656c6c6f", result_str.c_str());
    }
    {
        std::string source("1");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_hex(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s hex encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("31", result_str.c_str());
    }
    {
        std::string source("11");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_hex(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s hex encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("3131", result_str.c_str());
    }
    {
        std::string source("111");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_hex(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s hex encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("313131", result_str.c_str());
    }
    {
        std::string source("1111");
        auto origin_vec = get_bytes_from_str(source);

        auto result_ir_str = ir_builtin_encode_hex(origin_vec);
        auto result_str = string_from_ir_str(result_ir_str);
        printf("origin %s hex encode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("31313131", result_str.c_str());
    }
}

TEST(hex, hex_decode)
{
    // valid cases
    {
        std::string source("68656c6c6f");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s hex decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("hello", result_str.c_str());
    }
    {
        std::string source("0x68656c6c6f");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s hex decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("hello", result_str.c_str());
    }
    {
        std::string source("31");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s hex decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("1", result_str.c_str());
    }
    {
        std::string source("0x31");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s hex decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("1", result_str.c_str());
    }
    {
        std::string source("3131");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s hex decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("11", result_str.c_str());
    }
    {
        std::string source("313131");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s hex decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("111", result_str.c_str());
    }
    {
        std::string source("31313131");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        auto result_str = string_from_ir_bytes(result_ir_bytes);
        printf("origin %s hex decode result %s\n", source.c_str(),
               result_str.c_str());
        ASSERT_STREQ("1111", result_str.c_str());
    }
    // invalid cases
    {
        // decode empty bytes
        std::string source("");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        ASSERT_EQ(0, result_ir_bytes->num);
    }
    {
        // decode wrong size 
        std::string source("68656c6c6");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        ASSERT_EQ(0, result_ir_bytes->num);
    }
    {
        // decode invalid hex-char
        std::string source("68656c6g6f");
        auto source_ir_str = create_ir_str(source);
        auto result_ir_bytes = ir_builtin_decode_hex(source_ir_str);
        ASSERT_EQ(0, result_ir_bytes->num);
    }
}
