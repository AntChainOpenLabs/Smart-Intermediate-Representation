// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#ifndef __STDLIB_H_
#define __STDLIB_H_

#ifdef CC_LIB_TEST_MOCK
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#endif // CC_LIB_TEST_MOCK

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "math.h"

#define uint128_t __uint128_t
#define int128_t __int128_t

// int256 extension needs at least LLVM12
typedef unsigned _ExtInt(256) uint256_t;
typedef _ExtInt(256) int256_t;

#define INT128_MAX (__int128) (((unsigned __int128) 1 << ((__SIZEOF_INT128__ * __CHAR_BIT__) - 1)) - 1)
#define INT128_MIN ((__int128_t)0 - ((__int128_t)1 << 126) - ((__int128_t)1 << 126))
#define UINT128_MAX (((__uint128_t)INT128_MAX << 1) + 1)

#define INT256_MAX (int256_t) (((uint256_t) 1 << (uint256_t)((2 * __SIZEOF_INT128__ * __CHAR_BIT__) - 1)) - (uint256_t)1)
#define INT256_MIN ((int256_t)0 - ((int256_t)1 << (int256_t)126) - ((int256_t)1 << (int256_t)126))
#define UINT256_MAX (((uint256_t)INT256_MAX << (uint256_t)1) + (uint256_t)1)

/*
 * Vector is used for dynamic array
 */
struct vector {
    uint32_t len;
    uint32_t cap;
    uint8_t *data;
};

struct RuntimeContext {
    const char *file_name;
    uint32_t line;
    uint32_t col;
};

#ifdef __cplusplus
extern "C" {
#endif

#ifdef CC_LIB_TEST_MOCK
#define __malloc malloc
#endif // CC_LIB_TEST_MOCK

#ifndef CC_LIB_TEST_MOCK
extern void*
memset(void *dest, uint8_t val, size_t length);
#endif

extern void
__memset(void *dest, uint8_t val, size_t length);
size_t
__strlen(const char *s);
extern bool
__memcmp(uint8_t *left, uint32_t left_len, uint8_t *right, uint32_t right_len);

#ifndef CC_LIB_TEST_MOCK
void *
__malloc(size_t size);
extern void *
memcpy(void *dest, const void *src, uint32_t length);
extern void *
realloc(void *ptr, size_t size);
void
free(void *ptr);
int32_t
__isdigit(int32_t c);
int32_t
__isupper(int32_t c);
int32_t
__tolower(int32_t c);
char *
__strcpy(char *__restrict dest, const char *__restrict src);
size_t
__numlen(int256_t num);
size_t
__unumlen(uint256_t num);
int32_t
__strncmp(const char *_l, const char *_r, size_t n);
int256_t
__strtoi256(const char *__restrict nptr, char **__restrict endptr, int base);
uint256_t
__strtou256(const char *__restrict nptr, char **__restrict endptr, int base);

//bigint div
uint256_t div256_u256_rem(uint256_t dividend, uint256_t divisor, uint256_t *remainder);

uint256_t div256_u256(uint256_t dividend, uint256_t divisor);

#define malloc __malloc

/*
 * External platform host API: abort
 */
extern void
abort(char *msg, uint32_t msg_length);

#ifdef __cplusplus
}
#endif

#endif // CC_LIB_TEST_MOCK

struct vector *
vector_concat(struct vector *left_vec, struct vector *right_vec);
struct vector *
vector_new(uint32_t length, uint32_t size, uint8_t *initial);
struct vector *
vector_copy(struct vector *value);
uint8_t *
vector_bytes(struct vector *v);
uint32_t
vector_len(struct vector *v);

int32_t
decode_uleb128(int32_t *v, uint8_t *buf, int32_t offset, int32_t len);
int32_t
encode_uleb128(int32_t value, uint8_t *buffer, uint32_t offset);
int32_t
decode_uleb128_value(uint8_t *buffer, uint32_t offset, int32_t len);
int32_t
uleb128_value_length(uint32_t value);
void
runtime_abort(char *msg, uint32_t msg_length,
              struct RuntimeContext *runtime_context);
uint8_t *
ptr_offset(uint8_t *buf, int32_t data_offset);
int32_t
memcpy_offset(uint8_t *des, int32_t des_length, int32_t offset, uint8_t *src,
              int32_t src_len);

// wrapper for co_call, when child reverted, auto revert current call
void builtin_co_call_or_revert(const char *contract,
                                uint32_t contract_length,
                                const char *method,
                                uint32_t method_length,
                                const char *argpack,
                                uint32_t argpack_length);

#ifdef __cplusplus
} // end "C"
#endif

#ifndef CC_LIB_TEST_MOCK
#define IR_ABORT(msg, len) abort((msg), (len))
#else // non-wasm env
#define abort() (0)
#define IR_ABORT(msg, len) abort()
#endif // CC_LIB_TEST_MOCK

#endif // __STDLIB_H_ 
