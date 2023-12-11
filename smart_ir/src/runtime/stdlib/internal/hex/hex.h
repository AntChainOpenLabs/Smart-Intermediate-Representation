// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#ifndef HEX_H
#define HEX_H

#define HEX_ENCODE_OUT_SIZE(s) ((unsigned int)(s * 2))
#define HEX_DECODE_OUT_SIZE(s) ((unsigned int)(s / 2))

#ifdef __cplusplus
extern "C" {
#endif

/*
 * out is null-terminated encode string.
 * return values is out length, exclusive terminating `\0'
 */
uint32_t
hex_encode(const unsigned char *in, uint32_t inlen, char *out);

/*
 * return values is out length
 */
uint32_t
hex_decode(const char *in, uint32_t inlen, unsigned char *out);

#ifdef __cplusplus
} // extern "C"
#endif

#endif /* HEX_H */