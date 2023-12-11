// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./data_stream.h"

#ifndef CC_LIB_TEST_MOCK
#define assert(x) (0)
extern void *
__malloc(size_t size);
extern void *
memcpy(void *dest, const void *src, uint32_t length);
#else
extern void
__memset(void *dest, uint8_t val, size_t length);
#include <assert.h>
#include <string.h>
#endif // CC_LIB_TEST_MOCK

#define uint128_t __uint128_t
#define int128_t __int128_t

extern void
check_end_offset(int32_t offset, int32_t len)
{
    if (len > offset) {
        char msg[] = "DataStreamDecodeError: too long data stream";
        IR_ABORT(msg, sizeof(msg) - 1);
    }
}

/*
 * Data stream encode function declaration macros.
 */
#define DATA_STREAM_ENCODE_INT_DECLARE(id, ty)                          \
    int32_t data_stream_encode_##id(ty v, uint8_t *buf, int32_t offset) \
    {                                                                   \
        assert(buf != NULL);                                            \
        assert(offset >= 0);                                            \
        memcpy(buf + offset, &v, sizeof(v));                            \
        return offset + sizeof(v);                                      \
    }

/*
 * Data stream decode function declaration macros.
 */
#define DATA_STREAM_DECODE_INT_DECLARE(id, ty)                                \
    int32_t data_stream_decode_##id(ty *v, uint8_t *buf, int32_t offset,      \
                                    int32_t len)                              \
    {                                                                         \
        assert(v != NULL);                                                    \
        assert(buf != NULL);                                                  \
        assert(offset >= 0);                                                  \
        if (offset + sizeof(*v) > len) {                                      \
            char msg[] = "DataStreamDecodeError: decode offset out of range"; \
            IR_ABORT(msg, sizeof(msg) - 1);                                   \
        }                                                                     \
        memcpy(v, buf + offset, sizeof(*v));                                  \
        return offset + sizeof(*v);                                           \
    }

/*
 * Data stream encode array function declaration macros.
 */
#define DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(id, ty)                         \
    int32_t data_stream_encode_##id##array(qvector_t *v, uint8_t *buf,       \
                                           int32_t offset,                   \
                                           struct RuntimeContext *ctx)       \
    {                                                                        \
        int32_t n;                                                           \
        assert(v != NULL);                                                   \
        assert(buf != NULL);                                                 \
        assert(offset >= 0);                                                 \
        n = (int32_t)(v->num);                                               \
        int32_t n_offset = encode_uleb128(n, buf, offset);                   \
        int32_t elem_offset = offset + n_offset;                             \
        for (int32_t i = 0; i < n; ++i) {                                    \
            void *elem_v_p = qvector_getat(v, i, false, ctx);                \
            ty elem_v = 0;                                                   \
            memcpy(&elem_v, elem_v_p, sizeof(ty));                           \
            elem_offset = data_stream_encode_##id(elem_v, buf, elem_offset); \
        }                                                                    \
        return offset + n_offset + n * sizeof(ty);                           \
    }

/*
 * Data stream decode array function declaration macros.
 */
#define DATA_STREAM_DECODE_INT_ARRAY_DECLARE(id, ty)                     \
    int32_t data_stream_decode_##id##array(qvector_t *v, uint8_t *buf,   \
                                           int32_t offset, int32_t len)  \
    {                                                                    \
        assert(v != NULL);                                               \
        assert(v->num == 0);                                             \
        assert(v->max == 1);                                             \
        int32_t n = 0;                                                   \
        int32_t n_offset = decode_uleb128(&n, buf, offset, len);         \
        if (n < 0)                                                       \
            return -1;                                                   \
        qvector_clear(v);                                                \
        v->objsize = sizeof(ty);                                         \
        int32_t elem_offset = offset + n_offset;                         \
        for (int32_t i = 0; i < n; ++i) {                                \
            ty elem_v = 0;                                               \
            elem_offset =                                                \
                data_stream_decode_##id(&elem_v, buf, elem_offset, len); \
            qvector_addlast(v, &elem_v);                                 \
        }                                                                \
        return offset + n_offset + n * sizeof(ty);                       \
    }

/*
 * Data stream encode str map function declaration macros.
 */
#define DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(id, ty)                        \
    int32_t data_stream_encode_str##id##map(qhashtbl_t *v, uint8_t *buf,      \
                                            int32_t offset)                   \
    {                                                                         \
        qhashtbl_obj_t obj;                                                   \
        __memset((void *)&obj, 0, sizeof(obj));                               \
        int32_t n = (int32_t)qhashtbl_size(v);                                \
        int32_t n_offset = encode_uleb128(n, buf, offset);                    \
        int32_t elem_offset = offset + n_offset;                              \
        while (qhashtbl_getnext(v, &obj, true) == true) {                     \
            int32_t str_n = (int32_t)__strlen((char*)obj.key);                      \
            struct vector *str_v = vector_new(str_n, 1, (uint8_t *)obj.key); \
            ty elem_v = 0;                                                    \
            memcpy(&elem_v, obj.data, sizeof(ty));                            \
            elem_offset = data_stream_encode_str(str_v, buf, elem_offset);    \
            elem_offset = data_stream_encode_##id(elem_v, buf, elem_offset);  \
        }                                                                     \
        return elem_offset;                                                   \
    }

/*
 * Data stream decode str map function declaration macros.
 */
#define DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(id, ty)                         \
    int32_t data_stream_decode_str##id##map(qhashtbl_t *v, uint8_t *buf,       \
                                            int32_t offset, int32_t len)       \
    {                                                                          \
        int32_t n = 0;                                                         \
        int32_t n_offset = decode_uleb128(&n, buf, offset, len);               \
        if (n < 0)                                                             \
            return -1;                                                         \
        int32_t elem_offset = offset + n_offset;                               \
        for (int32_t i = 0; i < n; ++i) {                                      \
            int32_t buf_length = decode_uleb128_value(buf, elem_offset, len);  \
            ty elem_v = 0;                                                     \
            struct vector *str_v = vector_new(buf_length, 1, NULL);            \
            elem_offset =                                                      \
                data_stream_decode_str(str_v, buf, elem_offset, len);          \
            elem_offset =                                                      \
                data_stream_decode_##id(&elem_v, buf, elem_offset, len);       \
            qhashtbl_put(v, (int64_t)vector_bytes(str_v), &elem_v, sizeof(ty)); \
        }                                                                      \
        return elem_offset;                                                    \
    }

DATA_STREAM_ENCODE_INT_DECLARE(bool, bool)
DATA_STREAM_ENCODE_INT_DECLARE(u8, uint8_t)
DATA_STREAM_ENCODE_INT_DECLARE(u16, uint16_t)
DATA_STREAM_ENCODE_INT_DECLARE(u32, uint32_t)
DATA_STREAM_ENCODE_INT_DECLARE(u64, uint64_t)
DATA_STREAM_ENCODE_INT_DECLARE(u128, uint128_t)
DATA_STREAM_ENCODE_INT_DECLARE(i8, int8_t)
DATA_STREAM_ENCODE_INT_DECLARE(i16, int16_t)
DATA_STREAM_ENCODE_INT_DECLARE(i32, int32_t)
DATA_STREAM_ENCODE_INT_DECLARE(i64, int64_t)
DATA_STREAM_ENCODE_INT_DECLARE(i128, int128_t)

DATA_STREAM_DECODE_INT_DECLARE(bool, uint8_t)
DATA_STREAM_DECODE_INT_DECLARE(u8, uint8_t)
DATA_STREAM_DECODE_INT_DECLARE(u16, uint16_t)
DATA_STREAM_DECODE_INT_DECLARE(u32, uint32_t)
DATA_STREAM_DECODE_INT_DECLARE(u64, uint64_t)
DATA_STREAM_DECODE_INT_DECLARE(u128, uint128_t)
DATA_STREAM_DECODE_INT_DECLARE(i8, int8_t)
DATA_STREAM_DECODE_INT_DECLARE(i16, int16_t)
DATA_STREAM_DECODE_INT_DECLARE(i32, int32_t)
DATA_STREAM_DECODE_INT_DECLARE(i64, int64_t)
DATA_STREAM_DECODE_INT_DECLARE(i128, int128_t)

DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(bool, bool)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u8, uint8_t)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u16, uint16_t)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u32, uint32_t)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u64, uint64_t)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(u128, uint128_t)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i8, int8_t)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i16, int16_t)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i32, int32_t)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i64, int64_t)
DATA_STREAM_ENCODE_INT_ARRAY_DECLARE(i128, int128_t)

DATA_STREAM_DECODE_INT_ARRAY_DECLARE(bool, uint8_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(u8, uint8_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(u16, uint16_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(u32, uint32_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(u64, uint64_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(u128, uint128_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(i8, int8_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(i16, int16_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(i32, int32_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(i64, int64_t)
DATA_STREAM_DECODE_INT_ARRAY_DECLARE(i128, int128_t)

DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(bool, bool)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u8, uint8_t)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u16, uint16_t)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u32, uint32_t)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u64, uint64_t)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(u128, uint128_t)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i8, int8_t)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i16, int16_t)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i32, int32_t)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i64, int64_t)
DATA_STREAM_ENCODE_STR_INT_MAP_DECLARE(i128, int128_t)

DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(bool, uint8_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(u8, uint8_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(u16, uint16_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(u32, uint32_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(u64, uint64_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(u128, uint128_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(i8, int8_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(i16, int16_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(i32, int32_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(i64, int64_t)
DATA_STREAM_DECODE_STR_INT_MAP_DECLARE(i128, int128_t)

int32_t
data_stream_encode_str(const struct vector *v, uint8_t *buf, int32_t offset)
{
    assert(v != NULL);
    assert(v->len >= 0);
    assert(v->cap == 1);
    return data_stream_encode_vec(v, buf, offset);
}

int32_t
data_stream_decode_str(struct vector *v, uint8_t *buf, int32_t offset,
                       int32_t len)
{
    assert(v != NULL);
    assert(v->len == 0);
    assert(v->cap == 1);
    return data_stream_decode_vec(v, buf, offset, len);
}

int32_t
data_stream_encode_vec(const struct vector *v, uint8_t *buf, int32_t offset)
{
    int32_t n;
    assert(v != NULL);
    assert(v->len >= 0);
    assert(v->cap > 0);
    assert(buf != NULL);
    assert(offset >= 0);
    n = (int32_t)(v->len);
    int32_t n_offset = encode_uleb128(n, buf, offset);
    memcpy((void *)(buf + offset + n_offset), v->data, n);
    return offset + n_offset + n;
}

int32_t
data_stream_encode_strstrmap(qhashtbl_t *v, uint8_t *buf, int32_t offset)
{
    qhashtbl_obj_t obj;
    __memset((void *)&obj, 0, sizeof(obj));
    int32_t n = (int32_t)qhashtbl_size(v);
    int32_t n_offset = encode_uleb128(n, buf, offset);
    int32_t elem_offset = offset + n_offset;
    while (qhashtbl_getnext(v, &obj, true) == true) {
        int32_t str_k_n = (int32_t)__strlen((char*)obj.key);
        struct vector *str_k = vector_new(str_k_n, 1, (uint8_t *)obj.key);
        elem_offset = data_stream_encode_str(str_k, buf, elem_offset);

        uint32_t *addr = (uint32_t *)obj.data;
        uint32_t addr_value = *addr;
        struct vector *elem_v = (struct vector *)addr_value;
        elem_offset = data_stream_encode_str(elem_v, buf, elem_offset);
    }
    return elem_offset;
}

int32_t
data_stream_decode_vec(struct vector *v, uint8_t *buf, int32_t offset,
                       int32_t len)
{
    int32_t n = 0;
    int32_t n_offset = decode_uleb128(&n, buf, offset, len);
    if (n < 0)
        return -1;
    v->len = n;
    if (offset + n_offset + n > len) {
        char msg[] = "DataStreamDecodeError: decode offset out of range";
        IR_ABORT(msg, sizeof(msg) - 1);
    }
    memcpy(&v->data[0], buf + offset + n_offset, n);
    return offset + n_offset + n;
}

/*
 * Encode string vector, each element store the str pointer value and consists
 * of 4 bytes.
 */
int32_t
data_stream_encode_strarray(qvector_t *v, uint8_t *buf, int32_t offset,
                            struct RuntimeContext *ctx)
{
    int32_t n;
    assert(v != NULL);
    assert(buf != NULL);
    assert(offset >= 0);
    n = (int32_t)(v->num);
    int32_t n_offset = encode_uleb128(n, buf, offset);
    int32_t elem_offset = offset + n_offset;
    for (int32_t i = 0; i < n; ++i) {
        // Read the vector pointer value into the array.
        uint32_t *addr = qvector_getat(v, i, false, ctx);
        uint32_t addr_value = *addr;
        struct vector *elem_v = (struct vector *)addr_value;
        elem_offset = data_stream_encode_str(elem_v, buf, elem_offset);
    }
    return elem_offset;
}

int32_t
data_stream_decode_strstrmap(qhashtbl_t *v, uint8_t *buf, int32_t offset,
                             int32_t len)
{
    int32_t n = 0;
    int32_t n_offset = decode_uleb128(&n, buf, offset, len);
    if (n < 0)
        return -1;
    int32_t elem_offset = offset + n_offset;
    for (int32_t i = 0; i < n; ++i) {
        int32_t k_length = decode_uleb128_value(buf, elem_offset, len);

        struct vector *str_v = vector_new(k_length, 1, NULL);
        elem_offset = data_stream_decode_str(str_v, buf, elem_offset, len);

        int32_t v_length = decode_uleb128_value(buf, elem_offset, len);
        struct vector *elem_vector = vector_new(v_length, 1, 0);
        elem_offset =
            data_stream_decode_str(elem_vector, buf, elem_offset, len);
        // Put the vector pointer value into the array.
        uint32_t vector_addr = (uint32_t)elem_vector;

        qhashtbl_put(v, (int64_t) vector_bytes(str_v), &vector_addr,
                     sizeof(uint32_t));
    }
    return elem_offset;
}

/*
 * Decode string vector, each element store the str pointer value and consists
 * of 4 bytes.
 */
int32_t
data_stream_decode_strarray(qvector_t *v, uint8_t *buf, int32_t offset,
                            int32_t len)
{
    assert(v != NULL);
    assert(v->max == 0);
    assert(v->num == 1);
    int32_t n = 0;
    int32_t n_offset = decode_uleb128(&n, buf, offset, len);
    if (n < 0)
        return -1;
    qvector_clear(v);
    v->objsize = sizeof(struct vector *);
    int32_t elem_offset = offset + n_offset;
    for (int32_t i = 0; i < n; ++i) {
        int32_t length = 0;
        decode_uleb128(&length, buf, elem_offset, len);
        struct vector *elem_vector = vector_new(length, 1, 0);
        elem_offset =
            data_stream_decode_str(elem_vector, buf, elem_offset, len);
        // Put the vector pointer value into the array.
        uint32_t vector_addr = (uint32_t)elem_vector;
        qvector_addlast(v, &vector_addr);
    }
    return elem_offset;
}

/*
 * Get all k-v spaces in data stream.
 */
size_t
qhashtbl_total_space(qhashtbl_t *v)
{
    qhashtbl_obj_t obj;
    // must be cleared before call
    __memset((void *)&obj, 0, sizeof(obj));
    size_t space = 0;
    while (qhashtbl_getnext(v, &obj, true) == true) {
        int32_t str_n = 0;
        if (TABLE_KEY_IS_INT(v)) {
            switch (v->key_runtime_ty) {
                case IR_RUNTIME_TYPE_U8: str_n = 1; break;
                case IR_RUNTIME_TYPE_U16: str_n = 2; break;
                case IR_RUNTIME_TYPE_U32: str_n = 4; break;
                case IR_RUNTIME_TYPE_U64: str_n = 8; break;
                case IR_RUNTIME_TYPE_U128: str_n = 16; break;
                case IR_RUNTIME_TYPE_I8: str_n = 1; break;
                case IR_RUNTIME_TYPE_I16: str_n = 2; break;
                case IR_RUNTIME_TYPE_I32: str_n = 4; break;
                case IR_RUNTIME_TYPE_I64: str_n = 8; break;
                case IR_RUNTIME_TYPE_I128: str_n = 16; break;
                default: {
                    char msg[] = "invalid map int key type";
                    IR_ABORT(msg, sizeof(msg) - 1);
                    return NULL;
                }
            }
        } else {
            str_n = (int32_t)__strlen((char*)obj.key);
        }
        // Key size
        space += uleb128_value_length(str_n);
        space += str_n;
        // Value size
        space += obj.size;
    }
    return space;
}
