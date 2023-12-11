// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "hex.h"

#define INVALID_CHAR (0xff)

/* hex digits table */
static const char hexDigits[] = "0123456789abcdef";

static inline unsigned char char_to_u4(char c) {
    if (c >= '0' && c <= '9') {
        return c - '0';
    }
    if (c >= 'a' && c <= 'f') {
        return c - 'a' + 10;
    }
    if (c >= 'A' && c <= 'F') {
        return c - 'A' + 10;
    }
    return INVALID_CHAR;
}

static inline char 
get_hex_char(uint32_t idx)
{
    if (idx >= (sizeof(hexDigits) - 1)) {
        return -1;
    }
    return hexDigits[idx];
}

uint32_t
hex_encode(const unsigned char *in, uint32_t inlen, char* out)
{
    int out_offset = 0;
    for (uint32_t i = 0; i < inlen; i++) {
        unsigned char c = in[i];

        out[out_offset++] = get_hex_char((c >> 4) & 0x0f);
        out[out_offset++] = get_hex_char(c & 0x0f);
    }

    return out_offset;
}

uint32_t 
hex_decode(const char *in, uint32_t inlen, unsigned char *out)
{
    if (inlen % 2 != 0) {
        return 0;
    }

    uint32_t offset = 0;
    // check 0x or 0X prefix
    if (inlen >= 2 && in[0] == '0' && ( in[1] == 'x' || in[1] == 'X')) {
        offset = 2;
    }

    int out_offset = 0;
    for (uint32_t i = offset; i < inlen; i += 2) {
        unsigned char h = char_to_u4(in[i]);
        if (h == INVALID_CHAR) {
            return 0;
        }
        
        unsigned char l = char_to_u4(in[i + 1]);
        if (l == INVALID_CHAR) {
            return 0;
        }

        out[out_offset++] = h << 4 | l;
    }

    return out_offset;
}