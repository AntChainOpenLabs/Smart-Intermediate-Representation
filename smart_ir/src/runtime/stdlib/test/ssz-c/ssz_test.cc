// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./ssz.hh"
#include "./test.h"

#include <stdlib.h>

// test data
// https://github.com/NAKsir-melody/cpp_ssz/blob/master/test_uint.cpp
// https://github.com/NAKsir-melody/cpp_ssz/blob/master/test_bytes.cpp

// https://github.com/protolambda/zssz/blob/master/zssz_test.go

TEST(szz, hex) {
    ASSERT_TRUE(ssz::from_hex("ab").size() == 1);
    ASSERT_TRUE(std::string("\xab").size() == 1);

    std::string s1 = ssz::from_hex("ab"); // ssz::print_hex(s1);
    std::string s2 = std::string("\xab"); // ssz::print_hex(s2);
    ASSERT_TRUE(s1 == s2);

    s1 = ssz::from_hex("ab1234");
    s2 = std::string("\xab\x12\x34");
    ASSERT_TRUE(s1 == s2);
}

TEST(szz, encode_u8)
{
    ASSERT_STREQ(ssz::encode_u8_hex(0).c_str(), "00");
    ASSERT_STREQ(ssz::encode_u8_hex(1).c_str(), "01");
    ASSERT_STREQ(ssz::encode_u8_hex(127).c_str(), "7f");
    ASSERT_STREQ(ssz::encode_u8_hex(255).c_str(), "ff");

    ASSERT_STREQ(ssz::encode_u8_hex(0xab).c_str(), "ab");
    ASSERT_STREQ(ssz::encode_u8_hex(0xff).c_str(), "ff");
}

TEST(szz, encode_u16)
{
    ASSERT_STREQ(ssz::encode_u16_hex(0).c_str(), "0000");
    ASSERT_STREQ(ssz::encode_u16_hex(5).c_str(), "0500");
    ASSERT_STREQ(ssz::encode_u16_hex(127).c_str(), "7f00");
    ASSERT_STREQ(ssz::encode_u16_hex(1024).c_str(), "0004");
    ASSERT_STREQ(ssz::encode_u16_hex(65535).c_str(), "ffff");

    ASSERT_STREQ(ssz::encode_u16_hex(0x0000).c_str(), "0000");
    ASSERT_STREQ(ssz::encode_u16_hex(0xabcd).c_str(), "cdab");
}

TEST(szz, encode_u32)
{
    ASSERT_STREQ(ssz::encode_u32_hex(0).c_str(), "00000000");
    ASSERT_STREQ(ssz::encode_u32_hex(5).c_str(), "05000000");
    ASSERT_STREQ(ssz::encode_u32_hex(65536).c_str(), "00000100");
    ASSERT_STREQ(ssz::encode_u32_hex(4294967295).c_str(), "ffffffff");

    ASSERT_STREQ(ssz::encode_u32_hex(0x00000000).c_str(), "00000000");
    ASSERT_STREQ(ssz::encode_u32_hex(0x01234567).c_str(), "67452301");
}

TEST(szz, encode_u64)
{
    ASSERT_STREQ(ssz::encode_u64_hex(0).c_str(), "0000000000000000");
    ASSERT_STREQ(ssz::encode_u64_hex(1).c_str(), "0100000000000000");
    ASSERT_STREQ(ssz::encode_u64_hex(0x0123456789abcdef).c_str(),
                 "efcdab8967452301");
    ASSERT_STREQ(ssz::encode_u64_hex(18446744073709551615ULL).c_str(),
                 "ffffffffffffffff");
}
