// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#ifndef BASE64_H
#define BASE64_H

#define BASE64_ENCODE_OUT_SIZE(s) ((unsigned int)((((s) + 2) / 3) * 4 + 1))
#define BASE64_DECODE_OUT_SIZE(s) ((unsigned int)(((s) / 4) * 3))

#ifdef __cplusplus
extern "C" {
#endif

/*
 * out is null-terminated encode string.
 * return values is out length, exclusive terminating `\0'
 */
uint32_t
base64_encode(const unsigned char *in, uint32_t inlen, char *out);

/*
 * return values is out length
 */
uint32_t
base64_decode(const char *in, uint32_t inlen, unsigned char *out);

bool
check_implementation();

#ifdef __cplusplus
} // extern "C"
#endif

#endif /* BASE64_H */
