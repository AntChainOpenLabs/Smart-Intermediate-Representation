// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once
#ifndef __MATH_C_H_
#define __MATH_C_H_

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

#define uint128_t __uint128_t
#define int128_t __int128_t

// int256 extension needs at least LLVM12
typedef unsigned _ExtInt(256) uint256_t;
typedef _ExtInt(256) int256_t;


extern uint8_t
ir_builtin_pow_u8(uint8_t base, uint8_t exp);
extern uint16_t
ir_builtin_pow_u16(uint16_t base, uint16_t exp);
extern uint32_t
ir_builtin_pow_u32(uint32_t base, uint32_t exp);
extern uint64_t
ir_builtin_pow_u64(uint64_t base, uint64_t exp);
extern __uint128_t
ir_builtin_pow_u128(__uint128_t base, __uint128_t exp);
extern uint256_t
ir_builtin_pow_u256(uint256_t base, uint256_t exp);

extern int8_t
ir_builtin_pow_i8(int8_t base, int8_t exp);
extern int16_t
ir_builtin_pow_i16(int16_t base, int16_t exp);
extern int32_t
ir_builtin_pow_i32(int32_t base, int32_t exp);
extern int64_t
ir_builtin_pow_i64(int64_t base, int64_t exp);
extern __int128_t
ir_builtin_pow_i128(__int128_t base, __int128_t exp);
extern int256_t
ir_builtin_pow_i256(int256_t base, int256_t exp);


extern char *
builtin_i8_toa(int8_t num, int radix);
extern char *
builtin_i16_toa(int16_t num, int radix);
extern char *
builtin_i32_toa(int32_t num, int radix);
extern char *
builtin_i64_toa(int64_t num, int radix);
extern char *
builtin_i128_toa(__int128_t num, int radix);
extern char *
builtin_i256_toa(int256_t num, int radix);

extern char *
builtin_u8_toa(uint8_t num, int radix);
extern char *
builtin_u16_toa(uint16_t num, int radix);
extern char *
builtin_u32_toa(uint32_t num, int radix);
extern char *
builtin_u64_toa(uint64_t num, int radix);
extern char *
builtin_u128_toa(__uint128_t num, int radix);
extern char *
builtin_u256_toa(uint256_t num, int radix);

extern struct vector *
ir_builtin_i8_to_str(int8_t num, int radix);
extern struct vector *
ir_builtin_i16_to_str(int16_t num, int radix);
extern struct vector *
ir_builtin_i32_to_str(int32_t num, int radix);
extern struct vector *
ir_builtin_i64_to_str(int64_t num, int radix);
extern struct vector *
ir_builtin_i128_to_str(__int128_t num, int radix);
extern struct vector *
ir_builtin_i256_to_str(int256_t num, int radix);

extern struct vector *
ir_builtin_u8_to_str(uint8_t num, int radix);
extern struct vector *
ir_builtin_u16_to_str(uint16_t num, int radix);
extern struct vector *
ir_builtin_u32_to_str(uint32_t num, int radix);
extern struct vector *
ir_builtin_u64_to_str(uint64_t num, int radix);
extern struct vector *
ir_builtin_u128_to_str(uint128_t num, int radix);
extern struct vector *
ir_builtin_u256_to_str(uint256_t num, int radix);

// string to integer
extern __int128_t ir_builtin_str_to_i128(struct vector *str);
extern __uint128_t ir_builtin_str_to_u128(struct vector *str);
extern int256_t ir_builtin_str_to_i256(struct vector *str);
extern uint256_t ir_builtin_str_to_u256(struct vector *str);

#ifdef __cplusplus
}

#endif
#endif // __PATH_C_H_
