// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./data_stream_builtin.h"
// include data_stream.h but not data_stream.c because data_stream.bc is
// included by ir
#include "./data_stream.h"
#include "./stdlib.h"

#define uint128_t __uint128_t
#define int128_t __int128_t

/*
 * Data stream encode function declaration macros.
 */
#define BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(id, ty, size)      \
    qvector_t *ir_builtin_data_stream_encode_##id(ty value)    \
    {                                                             \
        qvector_t *out = qvector(size, 1, QVECTOR_RESIZE_DOUBLE); \
        int n = data_stream_encode_##id(value, out->data, 0);     \
        out->num = n;                                             \
        return out;                                               \
    }

/*
 * Data stream encode array function declaration macros.
 * num * objsize + 5 to malloc enough [u8] memory for storing ds_encode(int_arr)
 */
#define BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(id)                   \
    qvector_t *ir_builtin_data_stream_encode_##id##array(qvector_t *in) \
    {                                                                      \
        qvector_t *out =                                                   \
            qvector(in->num * in->objsize + 5, 1, QVECTOR_RESIZE_DOUBLE);  \
        struct RuntimeContext ctx = { "", 0, 0 };                          \
        int n = data_stream_encode_##id##array(in, out->data, 0, &ctx);    \
        out->num = n;                                                      \
        return out;                                                        \
    }

size_t
calculate_str_int_map_data_stream_max_size(qhashtbl_t *in)
{
    size_t total_size = 5;     // max length of header
    size_t str_header_len = 5; // max length of str len header
    qhashtbl_obj_t item;
    while (qhashtbl_getnext(in, &item, false)) {
        total_size += (str_header_len + __strlen((char*)item.key));
        total_size += item.size; // TODO: test whether item.size == 16 when map
                                 // value is i128
    }
    return total_size;
}

size_t
calculate_str_str_map_data_stream_max_size(qhashtbl_t *in)
{
    size_t total_size = 5;     // max length of header
    size_t str_header_len = 5; // max length of str len header
    qhashtbl_obj_t item;
    while (qhashtbl_getnext(in, &item, false)) {
        total_size += (str_header_len + __strlen((char*)item.key));
        total_size += item.size; // TODO: test whether item.size == 16 when map
                                 // value is i128
        uint32_t *addr = (uint32_t *)item.data;
        uint32_t addr_value = *addr;
        struct vector *elem_v = (struct vector *)addr_value;
        total_size += elem_v->len;
    }
    return total_size;
}

/*
 * Data stream encode str map function declaration macros.
 */
#define BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(id)                   \
    qvector_t *ir_builtin_data_stream_encode_str##id##map(qhashtbl_t *in) \
    {                                                                        \
        size_t size = calculate_str_int_map_data_stream_max_size(in);        \
        qvector_t *out = qvector(size, 1, QVECTOR_RESIZE_DOUBLE);            \
        int n = data_stream_encode_str##id##map(in, out->data, 0);           \
        out->num = n;                                                        \
        return out;                                                          \
    }

BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(bool, bool, 1)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(u8, uint8_t, 1)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(u16, uint16_t, 2)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(u32, uint32_t, 4)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(u64, uint64_t, 8)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(u128, uint128_t, 16)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(u256, uint256_t, 32)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(i8, int8_t, 1)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(i16, int16_t, 2)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(i32, int32_t, 4)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(i64, int64_t, 8)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(i128, int128_t, 16)
BUILTIN_DATA_STREAM_ENCODE_INT_DECLARE(i256, int256_t, 32)

BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(bool)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u8)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u16)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u32)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u64)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u128)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u256)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i8)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i16)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i32)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i64)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i128)
BUILTIN_DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i256)

BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(bool)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u8)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u16)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u32)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u64)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u128)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u256)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i8)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i16)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i32)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i64)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i128)
BUILTIN_DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i256)

qvector_t *ir_builtin_data_stream_encode_strstrmap(qhashtbl_t *in)
{
    size_t size = calculate_str_str_map_data_stream_max_size(in);
    qvector_t *out = qvector(size, 1, QVECTOR_RESIZE_DOUBLE);
    int n = data_stream_encode_strstrmap(in, out->data, 0);
    out->num = n;
    return out;
}

qvector_t *
ir_builtin_data_stream_encode_str(const struct vector *in)
{
    qvector_t *out = qvector(6 + in->len, 1, QVECTOR_RESIZE_DOUBLE);
    int n = data_stream_encode_str(in, out->data, 0);
    out->num = n;
    return out;
}

size_t
calculate_str_arr_data_stream_max_size(qvector_t *in)
{
    size_t total_size = 5;     // max length of header
    size_t str_header_len = 5; // max length of str len header
    struct vector item;
    while (qvector_getnext(in, (qvector_obj_t *)&item, false)) {
        total_size += (item.len + str_header_len);
    }
    return total_size;
}

/*
 * Encode string vector, each element store the str pointer value and consists
 * of 4 bytes.
 */
qvector_t *
ir_builtin_data_stream_encode_strarray(qvector_t *in)
{
    size_t size = calculate_str_arr_data_stream_max_size(in);
    qvector_t *out = qvector(size, 1, QVECTOR_RESIZE_DOUBLE);
    struct RuntimeContext ctx = { "", 0, 0 };
    int n = data_stream_encode_strarray(in, out->data, 0, &ctx);
    out->num = n;
    return out;
}
