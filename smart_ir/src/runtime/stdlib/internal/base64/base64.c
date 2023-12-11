// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0


#include "base64.h"

/* BASE 64 encode table */
static const char base64en[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
                               "abcdefghijklmnopqrstuvwxyz"
                               "0123456789+/";

/* index in base64en */
static const unsigned char indexes_in_base64_en[] = {
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    /* '(', ')', '*', '+', ',', '-', '.', '/', */
    255,
    255,
    255,
    62,
    255,
    255,
    255,
    63,
    /* '0', '1', '2', '3', '4', '5', '6', '7', */
    52,
    53,
    54,
    55,
    56,
    57,
    58,
    59,
    /* '8', '9', ':', ';', '<', '=', '>', '?', */
    60,
    61,
    255,
    255,
    255,
    255,
    255,
    255,

    /* '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', */
    255,
    0,
    1,
    2,
    3,
    4,
    5,
    6,

    /* 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', */
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,

    /* 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', */
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,

    /* 'X', 'Y', 'Z', '[', '\', ']', '^', '_', */
    23,
    24,
    25,
    255,
    255,
    255,
    255,
    255,

    /* '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', */
    255,
    26,
    27,
    28,
    29,
    30,
    31,
    32,

    /* 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', */
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    40,

    /* 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', */
    41,
    42,
    43,
    44,
    45,
    46,
    47,
    48,

    /* 'x', 'y', 'z', '{', '|', '}', '~', del, */
    49,
    50,
    51,
    255,
    255,
    255,
    255,
    255,
};

bool
check_implementation()
{
    // check the base64en and indexes_in_base64_en valid
    for (uint32_t i = 0; i < sizeof(base64en) - 1; i++) {
        char c = base64en[i];
        unsigned char c_int = (unsigned char)c;
        unsigned int int_in_index_array = indexes_in_base64_en[c_int];
        if (int_in_index_array != i) {
            return false;
        }
    }
    return true;
}

static inline char
get_base64en_char(uint32_t idx)
{
    if (idx >= (sizeof(base64en) - 1)) {
        return -1;
    }
    return base64en[idx];
}

uint32_t
base64_encode(const unsigned char *in, uint32_t inlen, char *out)
{
    int counter = 0;
    uint32_t bit_stream = 0;
    uint32_t out_offset = 0;
    int offset = 0; // last_offset
    for (uint32_t i = 0; i < inlen; i++) {
        unsigned char c = in[i];
        unsigned int num_val = (unsigned int)c;
        offset = 16 - counter % 3 * 8;
        bit_stream += num_val << offset;
        if (offset == 16) {
            out[out_offset++] = get_base64en_char(bit_stream >> 18 & 0x3f);
        }
        if (offset == 8) {
            out[out_offset++] = get_base64en_char(bit_stream >> 12 & 0x3f);
        }
        if (offset == 0 && counter != 3) {
            out[out_offset++] = get_base64en_char(bit_stream >> 6 & 0x3f);
            out[out_offset++] = get_base64en_char(bit_stream & 0x3f);
            bit_stream = 0;
        }
        counter++;
    }
    if (offset == 16) {
        out[out_offset++] = get_base64en_char(bit_stream >> 12 & 0x3f);
        out[out_offset++] = '=';
        out[out_offset++] = '=';
    }
    if (offset == 8) {
        out[out_offset++] = get_base64en_char(bit_stream >> 6 & 0x3f);
        out[out_offset++] = '=';
    }
    return out_offset;
}

uint32_t
base64_decode(const char *in, uint32_t inlen, unsigned char *out)
{
    int counter = 0;
    uint32_t bit_stream = 0;
    uint32_t out_offset = 0;
    int offset = 0; // last offset
    int equal_counter = 0;
    for (uint32_t i = 0; i < inlen; i++) {
        unsigned char c = (unsigned char)in[i];
        if (equal_counter >= 2) {
            return 0;
        }
        if (c == '=') {
            counter++;
            equal_counter++;
            continue;
        }
        unsigned char base64en_idx = indexes_in_base64_en[c];
        if (base64en_idx != 255) {
            offset = 18 - counter % 4 * 6;
            bit_stream += base64en_idx << offset;
            if (offset == 12) {
                out[out_offset++] = (char)(bit_stream >> 16 & 0xff);
            }
            if (offset == 6) {
                out[out_offset++] = (char)(bit_stream >> 8 & 0xff);
            }
            if (offset == 0 && counter != 4) {
                out[out_offset++] = (char)(bit_stream & 0xff);
                bit_stream = 0;
            }
        }
        else {
            return 0;
        }
        counter++;
    }
    return out_offset;
}
