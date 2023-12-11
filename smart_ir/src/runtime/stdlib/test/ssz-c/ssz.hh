// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef __SSZ_CC_HH_
#define __SSZ_CC_HH_

#include "../../ssz.h"

#include <stdio.h>
#include <string>
#include <ctype.h>

#define STD_STRING_LIT(s) \
    std::string(s, sizeof(s) - 1) // STD_STRING_LIT("\0\0")

namespace ssz {
inline uint8_t
hex_to_int(uint8_t c)
{
    if (c >= '0' && c <= '9')
        return c - '0';
    if (c >= 'a' && c <= 'f')
        return c - 'a' + 10;
    if (c >= 'A' && c <= 'F')
        return c - 'A' + 10;
    return 0;
}
inline void
print_hex(const std::string &s)
{
    printf("\n");
    for (int i = 0; i < s.size(); i++) {
        printf("print_hex: s[%d]: %02x\n", i, uint8_t(s[i]));
    }
}
inline std::string
from_hex(const std::string &s)
{
    if (s.size() % 2 != 0) {
        return "";
    }
    std::string dst;
    dst.resize(s.size() / 2);
    for (int i = 0; i < dst.size(); i++) {
        uint8_t c0 = s[i * 2 + 0];
        uint8_t c1 = s[i * 2 + 1];
        if (!isxdigit(c0) || !isxdigit(c1)) {
            return "";
        }
        uint8_t v = (hex_to_int(c0) << 4) + hex_to_int(c1);
        dst[i] = v;
    }
    return dst;
}
inline std::string
to_hex(const std::string &s)
{
    std::string dst;
    for (int i = 0; i < s.size(); i++) {
        char buf[4];
        sprintf(buf, "%02x", uint8_t(s[i]));
        dst += std::string(buf, 2);
    }
    return dst;
}

inline std::string
encode_u8_hex(uint8_t v)
{
    uint8_t buf[32];
    int32_t n = ssz_encode_u8(v, &buf[0], 0);
    std::string s = std::string((char *)buf, n);
    return to_hex(s);
}
inline std::string
encode_u16_hex(uint16_t v)
{
    uint8_t buf[32];
    int32_t n = ssz_encode_u16(v, &buf[0], 0);
    std::string s = std::string((char *)buf, n);
    return to_hex(s);
}
inline std::string
encode_u32_hex(uint32_t v)
{
    uint8_t buf[32];
    int32_t n = ssz_encode_u32(v, &buf[0], 0);
    std::string s = std::string((char *)buf, n);
    return to_hex(s);
}
inline std::string
encode_u64_hex(uint64_t v)
{
    uint8_t buf[32];
    int32_t n = ssz_encode_u64(v, &buf[0], 0);
    std::string s = std::string((char *)buf, n);
    return to_hex(s);
}
inline std::string
encode_offset_hex(uint32_t offset)
{
    return encode_u32_hex(offset);
}
inline std::string
encode_str_hdr_hex(const std::string &s)
{
    return encode_offset_hex(s.size());
}
inline std::string
encode_str_data_hex(const std::string &s)
{
    return to_hex(s);
}
inline uint8_t
decode_u8(uint8_t *buf)
{
    return 0;
}

}

#endif // __SSZ_CC_HH_
