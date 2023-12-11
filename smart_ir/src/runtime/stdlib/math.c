// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./math.h"
#include "./stdlib.h"

extern struct vector *
vector_new(uint32_t length, uint32_t size, uint8_t *initial);

#ifdef CC_LIB_TEST_MOCK

#include <string.h>
#include <stdlib.h>

#define __malloc malloc
#else // wasm env
extern void *
__malloc(size_t size);
extern void
free(void *);
#endif CC_LIB_TEST_MOCK

extern size_t
__strlen(const char *);
/*
 * pow function declaration macros.
 */
#define POW_INT_DECLARE(id, ty)                \
    ty ir_builtin_pow_##id(ty base, ty exp) \
    {                                          \
        ty result = 1;                         \
        ty val = 1;                            \
        for (;;) {                             \
            if (exp & val)                     \
                result *= base;                \
            exp >>= 1;                         \
            if (!exp)                          \
                break;                         \
            base *= base;                      \
        }                                      \
        return result;                         \
    }

POW_INT_DECLARE(u8, uint8_t);

POW_INT_DECLARE(u16, uint16_t);

POW_INT_DECLARE(u32, uint32_t);

POW_INT_DECLARE(u64, uint64_t);

POW_INT_DECLARE(u128, __uint128_t);

POW_INT_DECLARE(i8, int8_t);

POW_INT_DECLARE(i16, int16_t);

POW_INT_DECLARE(i32, int32_t);

POW_INT_DECLARE(i64, int64_t);

POW_INT_DECLARE(i128, __int128_t);

// when num is negative, split to two parts to avoid int_min parsed to negative int
#define MAX_ITOA_STR_SIZE 128
#define INT_TO_A(id, ty, uty)                                           \
    char *builtin_##id##_toa(ty num, int radix)                         \
    {                                                                   \
        if (radix < 2 || radix > 36 ) {                                 \
            char msg[] = "ITOA Error: invalid radix";                   \
            IR_ABORT(msg, sizeof(msg) - 1);                          \
        }                                                               \
        char *str = (char *)__malloc(MAX_ITOA_STR_SIZE * sizeof(char)); \
        char index[] = "0123456789abcdefghijklmnopqrstuvwxyz";          \
        uint128_t unum;                                                 \
        int i = 0, j, k;                                                \
        if (radix == 10 && num < 0) {                                   \
            ty num_first = num / 2;                                     \
            ty num_second = num - num_first;                            \
            unum = ((uint128_t)-num_first) + ((uint128_t)-num_second);  \
            str[i++] = '-';                                             \
        }                                                               \
        else {                                                          \
            unum = (uint128_t)(uty)num;                                 \
        }                                                               \
        do {                                                            \
            if (i >= MAX_ITOA_STR_SIZE) {                               \
                free(str);                                              \
                return NULL;                                            \
            }                                                           \
            str[i++] = index[unum % (unsigned)radix];                   \
            unum /= radix;                                              \
        } while (unum);                                                 \
                                                                        \
        str[i] = '\0';                                                  \
                                                                        \
        if (str[0] == '-') {                                            \
            k = 1;                                                      \
        }                                                               \
        else {                                                          \
            k = 0;                                                      \
        }                                                               \
                                                                        \
        char temp;                                                      \
        for (j = k; j <= (i - 1) / 2; j++) {                            \
            temp = str[j];                                              \
            str[j] = str[i - 1 + k - j];                                \
            str[i - 1 + k - j] = temp;                                  \
        }                                                               \
                                                                        \
        return str;                                                     \
    }

INT_TO_A(i8, int8_t, uint8_t);

INT_TO_A(i16, int16_t, uint16_t);

INT_TO_A(i32, int32_t, uint32_t);

INT_TO_A(i64, int64_t, uint64_t);

INT_TO_A(i128, __int128_t, __uint128_t);

INT_TO_A(u8, uint8_t,uint8_t);

INT_TO_A(u16, uint16_t, uint16_t);

INT_TO_A(u32, uint32_t, uint32_t);

INT_TO_A(u64, uint64_t, uint64_t);

INT_TO_A(u128, __uint128_t, __uint128_t);

#define INT_TO_STR(id, ty)                                        \
    struct vector *ir_builtin_##id##_to_str(ty num, int radix) \
    {                                                             \
        char *ret = builtin_##id##_toa(num, radix);               \
        return vector_new(__strlen(ret), 1, ret);                 \
    }

INT_TO_STR(i8, int8_t);

INT_TO_STR(i16, int16_t);

INT_TO_STR(i32, int32_t);

INT_TO_STR(i64, int64_t);

INT_TO_STR(i128, __int128_t);

INT_TO_STR(u8, uint8_t);

INT_TO_STR(u16, uint16_t);

INT_TO_STR(u32, uint32_t);

INT_TO_STR(u64, uint64_t);

INT_TO_STR(u128, __uint128_t);

// this tmp result must use _target_unsigned_type, otherwise the negative min value decode will cause overflow
// the for loop index i must be signed int to avoid loop
// the final result parsed to result_first and result_second to avoid too large to parse to int type
#define COMMON_STR_TO_INTEGER(_str, _target_type, _target_unsigned_type, _is_signed, _radix)  \
    struct vector* str = (_str);                                       \
    int32_t radix = (_radix);                                          \
    bool neg = false;                                                  \
    _target_unsigned_type result = 0;                                  \
    size_t offset = 0;                                                 \
    uint8_t *data = (uint8_t*) str->data;                              \
    if (offset >= str->len) {                                          \
        char msg[] = "str to int failed: empty string";                \
        IR_ABORT(msg, sizeof(msg) - 1);                             \
        return result;                                                 \
    }                                                                  \
    if ((_is_signed)) {                                                \
        if (data[offset] == '-') {                                     \
            neg = true;                                                \
            offset++;                                                  \
        } else if (data[offset] == '+') {                              \
            offset++;                                                  \
        }                                                              \
    }                                                                  \
    _target_unsigned_type digit_multiply = 1;                          \
    size_t digits_count = 0;                                           \
    for (int32_t i=str->len-1;i>=(int32_t) offset;i--) {               \
        uint8_t digit = data[i];                                       \
        if (digit == ',') {                                            \
            continue;                                                  \
        }                                                              \
        if ((digit < '0') || (digit > '9')) {                \
            char msg[] = "str to int failed: invalid char: ";          \
            char *full_msg = __malloc(sizeof(msg) + 1);                \
            memcpy(full_msg, msg, sizeof(msg)-1);                      \
            full_msg[sizeof(msg)] = digit;                             \
            IR_ABORT(full_msg, sizeof(msg));                        \
            return result;                                             \
        }                                                              \
        uint32_t digit_int =  digit - '0';                             \
        _target_unsigned_type new_result = result + (digit_multiply * digit_int); \
        if (new_result < result) {                                     \
            char msg[] = "str to int failed: overflow";                \
            IR_ABORT(msg, sizeof(msg) - 1);                         \
            return result;                                             \
        }                                                              \
        result = new_result;                                           \
        digit_multiply *= radix;                                       \
        digits_count++;                                                \
    }                                                                  \
    if (digits_count < 1) {                                            \
        char msg[] = "str to int failed: no digits";                   \
        IR_ABORT(msg, sizeof(msg) - 1);                             \
        return result;                                                 \
    }                                                                  \
    if (neg) {                                                         \
        _target_unsigned_type result_first = result / 2;               \
        _target_unsigned_type result_second = result - result_first;   \
        return - ((_target_type) result_first) - ((_target_type) result_second); \
    }                                                                  \
    return (_target_type) result;


__int128_t
ir_builtin_str_to_i128(struct vector *arg_str) {
    COMMON_STR_TO_INTEGER(arg_str, __int128_t, __uint128_t, true, 10);
}

__uint128_t
ir_builtin_str_to_u128(struct vector *arg_str) {
    COMMON_STR_TO_INTEGER(arg_str, __uint128_t, __uint128_t, false, 10)
}
