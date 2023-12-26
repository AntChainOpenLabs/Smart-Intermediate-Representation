// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./ssz.h"
#include "./ir_type.h"
#include "./stdlib.h"
#include "./data_stream.h"

extern uint32_t
get_ir_type_size_as_element(struct IRRuntimeClass *runtime_class);

extern intptr_t
get_all_runtimes_classes_address();

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

/*
 * SSZ encode int function declaration macros.
 */
#define SSZ_ENCODE_INT_DECLARE(id, ty)                          \
    int32_t ssz_encode_##id(ty v, uint8_t *buf, int32_t offset) \
    {                                                           \
        assert(buf != NULL);                                    \
        assert(offset >= 0);                                    \
        memcpy(buf + offset, &v, sizeof(v));                    \
        return offset + sizeof(v);                              \
    }
/*
 * SSZ decode int function declaration macros.
 */
#define SSZ_DECODE_INT_DECLARE(id, ty)           \
    int32_t ssz_decode_##id(ty *v, uint8_t *buf) \
    {                                            \
        assert(v != NULL);                       \
        assert(buf != NULL);                     \
        memcpy(v, buf, sizeof(*v));              \
        return sizeof(*v);                       \
    }

SSZ_ENCODE_INT_DECLARE(bool, bool)
SSZ_ENCODE_INT_DECLARE(u8, uint8_t)
SSZ_ENCODE_INT_DECLARE(u16, uint16_t)
SSZ_ENCODE_INT_DECLARE(u32, uint32_t)
SSZ_ENCODE_INT_DECLARE(u64, uint64_t)
SSZ_ENCODE_INT_DECLARE(u128, uint128_t)
SSZ_ENCODE_INT_DECLARE(u256, uint256_t)
SSZ_ENCODE_INT_DECLARE(i8, int8_t)
SSZ_ENCODE_INT_DECLARE(i16, int16_t)
SSZ_ENCODE_INT_DECLARE(i32, int32_t)
SSZ_ENCODE_INT_DECLARE(i64, int64_t)
SSZ_ENCODE_INT_DECLARE(i128, int128_t)
SSZ_ENCODE_INT_DECLARE(i256, int256_t)

SSZ_DECODE_INT_DECLARE(bool, uint8_t)
SSZ_DECODE_INT_DECLARE(u8, uint8_t)
SSZ_DECODE_INT_DECLARE(u16, uint16_t)
SSZ_DECODE_INT_DECLARE(u32, uint32_t)
SSZ_DECODE_INT_DECLARE(u64, uint64_t)
SSZ_DECODE_INT_DECLARE(u128, uint128_t)
SSZ_DECODE_INT_DECLARE(u256, uint256_t)
SSZ_DECODE_INT_DECLARE(i8, int8_t)
SSZ_DECODE_INT_DECLARE(i16, int16_t)
SSZ_DECODE_INT_DECLARE(i32, int32_t)
SSZ_DECODE_INT_DECLARE(i64, int64_t)
SSZ_DECODE_INT_DECLARE(i128, int128_t)
SSZ_DECODE_INT_DECLARE(i256, int256_t)

int32_t
ssz_encode_str(const struct vector *v, uint8_t *buf, int32_t hdr_offset,
               int32_t data_offset)
{
    assert(v != NULL);
    assert(v->len >= 0);
    assert(v->cap >= v->len);

    return ssz_encode_vec(v, buf, hdr_offset, data_offset);
}

int32_t
ssz_decode_str(struct vector *v, uint8_t *buf, int32_t length)
{
    assert(v != NULL);
    assert(v->len == 0);
    assert(v->cap >= v->len);

    return ssz_decode_vec(v, buf, length);
}

int32_t
ssz_encode_vec(const struct vector *v, uint8_t *buf, int32_t hdr_offset,
               int32_t data_offset)
{
    uint32_t n;

    assert(v != NULL);
    assert(v->len >= 0);
    assert(v->cap >= v->len);

    assert(buf != NULL);
    assert(hdr_offset >= 0);
    assert(data_offset >= 0);

    n = v->len;
    memcpy((void *)(buf + hdr_offset), &data_offset, sizeof(int32_t));
    memcpy((void *)(buf + data_offset), v->data, n);
    return data_offset + n;
}

int32_t
ssz_decode_vec(struct vector *v, uint8_t *buf, int32_t length)
{
    assert(v != NULL);
    assert(v->len == 0);
    assert(v->cap >= v->len);

    assert(buf != NULL);

    // 4 denotes the SSZ length bytes size

    memcpy(&v->data[0], buf, length);

    return length;
}

#define IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(id, ty, size)        \
    qvector_t *ir_builtin_ssz_encode_##id(void *val)           \
    {                                                             \
        ty *_val = (ty *)val;                                     \
        qvector_t *out = qvector(size, 1, QVECTOR_RESIZE_DOUBLE); \
        int n = ssz_encode_##id(*_val, out->data, 0);             \
        out->num = n;                                             \
        return out;                                               \
    }

IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(bool, bool, 1)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(u8, uint8_t, 1)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(u16, uint16_t, 2)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(u32, uint32_t, 4)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(u64, uint64_t, 8)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(u128, uint128_t, 16)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(u256, uint256_t, 32)

IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(i8, int8_t, 1)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(i16, int16_t, 2)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(i32, int32_t, 4)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(i64, int64_t, 8)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(i128, int128_t, 16)
IR_BUILTIN_SSZ_ENCODE_INT_DECLARE(i256, int256_t, 32)

#define IR_BUILTIN_SSZ_DECODE_INT_DECLARE(id, ty, size) \
    void *ir_builtin_ssz_decode_##id(qvector_t *val)    \
    {                                                      \
        ty *ret = malloc(size);                            \
        ssz_decode_##id(ret, val->data);                   \
        return (void *)ret;                                \
    }

IR_BUILTIN_SSZ_DECODE_INT_DECLARE(bool, bool, 1)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(u8, uint8_t, 1)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(u16, uint16_t, 2)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(u32, uint32_t, 4)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(u64, uint64_t, 8)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(u128, uint128_t, 16)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(u256, uint256_t, 32)

IR_BUILTIN_SSZ_DECODE_INT_DECLARE(i8, int8_t, 1)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(i16, int16_t, 2)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(i32, int32_t, 4)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(i64, int64_t, 8)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(i128, int128_t, 16)
IR_BUILTIN_SSZ_DECODE_INT_DECLARE(i256, int256_t, 32)

// When this type is encoded to [u8], whether the length of the array is fixed
bool
is_ssz_fixed_len(uint32_t runtime_class_offset)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_U16:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_U32:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_U64:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_U128:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_U256:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_I8:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_I16:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_I32:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_I64:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_I128:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_I256:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_BOOL:
        {
            return true;
        } break;
        case IR_RUNTIME_TYPE_STR:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_ASSET:
        {
            uint32_t *fields_offsets_array =
                (uint32_t *)(all_runtimes_classes_address
                             + runtime_class->struct_fields);
            for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
                uint32_t field_offset = fields_offsets_array[i];
                if (!is_ssz_fixed_len(field_offset)) {
                    return false;
                }
            }
            return true;
        } break;
        case IR_RUNTIME_TYPE_STRUCT:
        {
            uint32_t *fields_offsets_array =
                (uint32_t *)(all_runtimes_classes_address
                             + runtime_class->struct_fields);
            for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
                uint32_t field_offset = fields_offsets_array[i];
                if (!is_ssz_fixed_len(field_offset)) {
                    return false;
                }
            }
            return true;
        } break;
        case IR_RUNTIME_TYPE_ARRAY:
        {
            // array_size == 0 means it is an array with variable length: arr[T]
            if ((runtime_class->array_size == 0)
                || !(is_ssz_fixed_len(runtime_class->array_item_ty))) {
                return false;
            }
            else {
                return true;
            }
        } break;
        default:
        {
            char msg[] = "unknown ir runtime type in ssz type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

// Return the length of result([u8]) of ssz encode
uint32_t
ssz_encode_len(uint32_t runtime_class_offset, void *val)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8:
        {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_U16:
        {
            return 2;
        } break;
        case IR_RUNTIME_TYPE_U32:
        {
            return 4;
        } break;
        case IR_RUNTIME_TYPE_U64:
        {
            return 8;
        } break;
        case IR_RUNTIME_TYPE_U128:
        {
            return 16;
        } break;
        case IR_RUNTIME_TYPE_U256:
        {
            return 32;
        } break;
        case IR_RUNTIME_TYPE_I8:
        {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_I16:
        {
            return 2;
        } break;
        case IR_RUNTIME_TYPE_I32:
        {
            return 4;
        } break;
        case IR_RUNTIME_TYPE_I64:
        {
            return 8;
        } break;
        case IR_RUNTIME_TYPE_I128:
        {
            return 16;
        } break;
        case IR_RUNTIME_TYPE_I256:
        {
            return 32;
        } break;
        case IR_RUNTIME_TYPE_BOOL:
        {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_STR:
        {
            struct vector *_val = (struct vector *)val;
            return vector_len(_val);
        } break;
        case IR_RUNTIME_TYPE_ASSET:
        {
            size_t total = 0;
            uint32_t offset = 0;
            uint32_t *fields_offsets_array =
                (uint32_t *)(all_runtimes_classes_address
                             + runtime_class->struct_fields);
            for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
                uint32_t field_offset = fields_offsets_array[i];
                struct IRRuntimeClass *field_type =
                    (struct IRRuntimeClass *)(all_runtimes_classes_address
                                                 + field_offset);

                void *field_ptr =
                    get_data_ptr_of_ptr_value(field_offset, val + offset);

                total += ssz_encode_len(field_offset, field_ptr);
                if (!is_ssz_fixed_len(field_offset)) {
                    total += 4;
                }
                offset += get_ir_type_size_as_element(field_type);
            }

            return total;
        } break;
        case IR_RUNTIME_TYPE_STRUCT:
        {
            size_t total = 0;
            uint32_t offset = 0;
            uint32_t *fields_offsets_array =
                (uint32_t *)(all_runtimes_classes_address
                             + runtime_class->struct_fields);
            for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
                uint32_t field_offset = fields_offsets_array[i];
                struct IRRuntimeClass *field_type =
                    (struct IRRuntimeClass *)(all_runtimes_classes_address
                                                 + field_offset);
                void *field_ptr =
                    get_data_ptr_of_ptr_value(field_offset, val + offset);

                total += ssz_encode_len(field_offset, field_ptr);
                if (!is_ssz_fixed_len(field_offset)) {
                    total += 4;
                }
                offset += get_ir_type_size_as_element(field_type);
            }

            return total;
        } break;
        case IR_RUNTIME_TYPE_ARRAY:
        {
            qvector_t *_val = (qvector_t *)val;
            uint32_t len = 0;
            uint32_t elem_ty_offset = runtime_class->array_item_ty;
            IRRuntimeClass *elem_ty =
                (IRRuntimeClass *)(all_runtimes_classes_address
                                      + elem_ty_offset);

            uint32_t elem_len = get_ir_type_size_as_element(elem_ty);

            // if array is u8 array, then maybe very large, and each value
            // encoded size is same
            if (elem_ty->ty == IR_RUNTIME_TYPE_U8
                || elem_ty->ty == IR_RUNTIME_TYPE_I8) {
                // u8/i8 ssz encoded as 1 byte
                len += _val->num * 1;
            }
            else {
                for (uint32_t i = 0; i < _val->num; i++) {
                    void *elem_ptr =
                        get_array_elem_ptr_at_idx(runtime_class_offset, val, i);
                    len += ssz_encode_len(elem_ty_offset, elem_ptr);
                }
            }
            if (!is_ssz_fixed_len(elem_ty_offset)) {
                len += 4 * _val->num;
            }
            return len;

        } break;
        default:
        {
            char msg[] = "unknown ir runtime type in ssz type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

// if a ir type is ssz encode fixed length type, calculate the length
uint32_t
ssz_fix_ty_length(uint32_t runtime_class_offset)
{
    assert(is_ssz_fixed_len(runtime_class_offset));
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8:
        {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_U16:
        {
            return 2;
        } break;
        case IR_RUNTIME_TYPE_U32:
        {
            return 4;
        } break;
        case IR_RUNTIME_TYPE_U64:
        {
            return 8;
        } break;
        case IR_RUNTIME_TYPE_U128:
        {
            return 16;
        } break;
        case IR_RUNTIME_TYPE_U256:
        {
            return 32;
        } break;
        case IR_RUNTIME_TYPE_I8:
        {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_I16:
        {
            return 2;
        } break;
        case IR_RUNTIME_TYPE_I32:
        {
            return 4;
        } break;
        case IR_RUNTIME_TYPE_I64:
        {
            return 8;
        } break;
        case IR_RUNTIME_TYPE_I128:
        {
            return 16;
        } break;
        case IR_RUNTIME_TYPE_I256:
        {
            return 32;
        } break;
        case IR_RUNTIME_TYPE_BOOL:
        {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_STR:
        {
            char msg[] = "Bug: string is not a ssz encode fixed length";
            IR_ABORT(msg, sizeof(msg) - 1);
        } break;
        case IR_RUNTIME_TYPE_ASSET:
        {
            size_t total = 0;
            uint32_t *fields_offsets_array =
                (uint32_t *)(all_runtimes_classes_address
                             + runtime_class->struct_fields);
            for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
                uint32_t field_offset = fields_offsets_array[i];
                assert(is_ssz_fixed_len(field_offset));
                total += ssz_fix_ty_length(field_offset);
            }

            return total;
        } break;
        case IR_RUNTIME_TYPE_STRUCT:
        {
            size_t total = 0;
            uint32_t *fields_offsets_array =
                (uint32_t *)(all_runtimes_classes_address
                             + runtime_class->struct_fields);
            for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
                uint32_t field_offset = fields_offsets_array[i];
                assert(is_ssz_fixed_len(field_offset));
                total += ssz_fix_ty_length(field_offset);
            }

            return total;

            return total;
        } break;
        case IR_RUNTIME_TYPE_ARRAY:
        {
            uint32_t elem_ty_offset = runtime_class->array_item_ty;
            assert(is_ssz_fixed_len(elem_ty_offset));
            assert(runtime_class->array_size > 0);
            return runtime_class->array_size
                   * ssz_fix_ty_length(elem_ty_offset);

        } break;
        default:
        {
            char msg[] = "unknown ir runtime type in ssz type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

// ssz encode str to [u8]
qvector_t *
ir_builtin_ssz_encode_str(void *val)
{
    struct vector *_val = (struct vector *)val;
    uint32_t vec_len = vector_len(_val);
    qvector_t *out = qvector(vec_len, 1, QVECTOR_RESIZE_DOUBLE);
    ssz_encode_str(_val, out->data, 0, 0);
    out->num = vec_len;
    return out;
}

// sszdencode str to [u8]
void *
ir_builtin_ssz_decode_str(qvector_t *val)
{
    struct vector *ret = vector_new(val->num, 1, 0);
    ssz_decode_str(ret, val->data, val->num);
    return (void *)ret;
}

// ssz encode array type to [u8]
// fix len elem [u32;2]: [1, 2]: [1, 0, 0, 0, 2, 0, 0, 0]
// unfix len elem: ["hello", "world"] -> [8, 0, 0, 0, 13, 0 ,0, 0, h, e, l, l,o,
// w, o, r, l, d]
qvector_t *
ir_builtin_ssz_encode_array(uint32_t runtime_class_offset, void *val)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    assert(runtime_class->ty = IR_RUNTIME_TYPE_ARRAY);
    qvector_t *_val = (qvector_t *)val;

    uint32_t elem_ty_offset = runtime_class->array_item_ty;
    IRRuntimeClass *elem_ty =
        (IRRuntimeClass *)(all_runtimes_classes_address + elem_ty_offset);

    uint32_t encode_len = ssz_encode_len(runtime_class_offset, val);

    qvector_t *out = qvector(encode_len, 1, QVECTOR_RESIZE_DOUBLE);

    uint32_t offset = 0;
    if (!is_ssz_fixed_len(elem_ty_offset)) {
        offset = 4 * _val->num;
    }

    if (elem_ty->ty == IR_RUNTIME_TYPE_I8
        || elem_ty->ty == IR_RUNTIME_TYPE_U8) {
        memcpy(out->data + offset, _val->data, _val->num);
        offset += _val->num;
    }
    else {
        for (uint32_t i = 0; i < _val->num; i++) {
            void *elem_ptr =
                get_array_elem_ptr_at_idx(runtime_class_offset, val, i);
            qvector_t *elem_encode =
                ir_builtin_ssz_encode(elem_ty_offset, elem_ptr);
            memcpy(out->data + offset, elem_encode->data, elem_encode->num);
            if (!is_ssz_fixed_len(elem_ty_offset)) {
                memcpy(out->data + i * 4, (void *)&offset, 4);
            }
            offset += elem_encode->num;
        }
    }
    out->num = encode_len;
    return out;
}

// decode [u8] to arr type(arr[T:N] or arr[T])
void *
ir_builtin_ssz_decode_array(uint32_t runtime_class_offset,
                               bool allow_empty_object, qvector_t *val)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    assert(runtime_class->ty = IR_RUNTIME_TYPE_ARRAY);

    struct IRRuntimeClass *elem_ty =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class->array_item_ty);

    uint32_t arr_size = 0;
    if (val) {
        if (is_ssz_fixed_len(runtime_class->array_item_ty)) {
            // val: [1, 0, 0, 0, 2, 0, 0, 0] , type: [u32]
            // arr_size = val->num / 4
            uint32_t elem_len = ssz_fix_ty_length(runtime_class->array_item_ty);
            assert(val->num % elem_len == 0);
            arr_size = val->num / elem_len;
        }
        else if (val->num > 0) {
            // val: [8, 0, 0, 0, 13, 0, 0, 0, h, e, l, l, o, w, o, r, l, d] ,
            // type: [str] arr_size = 8 / 4
            uint32_t *first_offset = (uint32_t *)val->data;
            arr_size = *first_offset / 4;
        }
    }

    if (arr_size == 0) {
        size_t element_size = get_ir_type_size_as_element(elem_ty);
        return qvector(1, element_size, 0x02 /* QVECTOR_RESIZE_DOUBLE */);
    }
    else {
        // elem size in memery
        uint32_t elem_size = get_ir_type_size_as_element(elem_ty);
        qvector_t *ret = qvector(arr_size, elem_size, QVECTOR_RESIZE_DOUBLE);
        ret->num = arr_size;

        if (is_ssz_fixed_len(runtime_class->array_item_ty)) {
            // elem length in ssz encode [u8]
            uint32_t elem_len = ssz_fix_ty_length(runtime_class->array_item_ty);
            qvector_t *elem_u8_arr =
                qvector(elem_len, 1, QVECTOR_RESIZE_DOUBLE);
            elem_u8_arr->num = elem_len;
            for (uint32_t i = 0; i < arr_size; i++) {
                memcpy(elem_u8_arr->data, val->data + i * elem_len, elem_len);
                void *elem = ir_builtin_ssz_decode_impl(
                    runtime_class->array_item_ty, false, elem_u8_arr);
                void *elem_ptr =
                    get_ptr_of_ptr_value(runtime_class->array_item_ty, elem);
                memcpy(ret->data + i * elem_size, elem_ptr, elem_size);
            }
        }
        else {
            uint32_t offset = 4 * arr_size;
            for (uint32_t i = 0; i < arr_size - 1; i++) {
                // elem_len = 13 - 8
                uint32_t elem_len = *(uint32_t *)(val->data + (i + 1) * 4)
                                    - *(uint32_t *)(val->data + i * 4);
                // elem_u8_arr = [h, e, l, l, o]
                qvector_t *elem_u8_arr =
                    qvector(elem_len, 1, QVECTOR_RESIZE_DOUBLE);

                memcpy(elem_u8_arr->data, val->data + offset, elem_len);
                elem_u8_arr->num = elem_len;

                // ir_builtin_ssz_decode([h, e, l, l, o], str)
                void *elem = ir_builtin_ssz_decode_impl(
                    runtime_class->array_item_ty, false, elem_u8_arr);

                void *elem_ptr =
                    get_ptr_of_ptr_value(runtime_class->array_item_ty, elem);
                memcpy(ret->data + i * elem_size, elem_ptr, elem_size);

                offset += elem_len;
            }
            // last_elem_len = 18 - 13
            uint32_t last_elem_len =
                val->num * val->objsize
                - *(uint32_t *)(val->data + (arr_size - 1) * 4);
            // elem_u8_arr = [w, o, r, l, d]
            qvector_t *elem_u8_arr =
                qvector(last_elem_len, 1, QVECTOR_RESIZE_DOUBLE);

            memcpy(elem_u8_arr->data, val->data + offset, last_elem_len);
            elem_u8_arr->num = last_elem_len;
            // ir_builtin_ssz_decode([w, o, r, l, d], str)
            void *elem = ir_builtin_ssz_decode_impl(
                runtime_class->array_item_ty, false, elem_u8_arr);
            void *elem_ptr =
                get_ptr_of_ptr_value(runtime_class->array_item_ty, elem);
            memcpy(ret->data + (arr_size - 1) * elem_size, elem_ptr, elem_size);
        }
        return (void *)ret;
    }
}

// ssz encode struct/asset to [u8]
// struct Foo {
//     a: u8 = 1  // fix len elem
//     b: str = "aa" // str = [u8] // fix elem len, variable array size
//     c: [u8;2] = [3, 4] // fix elem len, fix array size
//     d: [str] = ["hello", "world"] // variable elem len, variable array size
// }
// encode to =>
// [1, 4B, 3, 4, 4B, a, a, 4B, 4B, h, e, l, l, o, w, o, r, l ,d]
// Each 4B represents the starting position of the variable-length element,
// occupying 4 bytes [1, 11, 0, 0, 0, 3, 4, 13, 0, 0, 0, a, a, 8, 0, 0, 0, 13,
// 0, 0, 0, h, e, l, l, o, w, o, r, l ,d]
qvector_t *
ir_builtin_ssz_encode_struct_like_ty(uint32_t runtime_class_offset,
                                        void *val)
{

    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    uint32_t encode_len = ssz_encode_len(runtime_class_offset, val);
    qvector_t *out = qvector(encode_len, 1, QVECTOR_RESIZE_DOUBLE);
    uint32_t offset = 0;
    uint32_t hdr = 0;
    uint32_t ptr_offset = 0;
    uint32_t *fields_offsets_array =
        (uint32_t *)(all_runtimes_classes_address
                     + runtime_class->struct_fields);

    for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
        uint32_t field_offset = fields_offsets_array[i];
        if (is_ssz_fixed_len(field_offset)) {
            offset += ssz_fix_ty_length(field_offset);
        }
        else {
            offset += 4;
        }
    }

    ptr_offset = 0;
    // [ 1, 11, 0, 0, 0, 3, 4, 13, 0, 0, 0, a, a, 8, 0, 0, 0, 13, 0, 0, 0, h, e,
    // l, l, o, w, o, r, l ,d]
    //  |                                  |
    //  hdr                                offset

    for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
        uint32_t field_offset = fields_offsets_array[i];
        struct IRRuntimeClass *field_type =
            (struct IRRuntimeClass *)(all_runtimes_classes_address
                                         + field_offset);
        void *field_ptr =
            get_data_ptr_of_ptr_value(field_offset, val + ptr_offset);
        qvector_t *elem_encode =
            ir_builtin_ssz_encode(field_offset, field_ptr);

        if (is_ssz_fixed_len(field_offset)) {
            // Write fix length elem to value_ptr[hdr]
            memcpy(out->data + hdr, elem_encode->data, elem_encode->num);
            hdr += elem_encode->num;
        }
        else {
            // Write fix length elem to value_ptr[offset], i.e., Foo.b = "aa"
            memcpy(out->data + offset, elem_encode->data, elem_encode->num);
            // Write 'offset' to value_ptr[hdr], i.e., write `11` at
            // out->data[1]
            memcpy(out->data + hdr, (void *)&offset, 4);
            offset += elem_encode->num;
            hdr += 4;
        }
        ptr_offset += get_ir_type_size_as_element(field_type);
    }

    out->num = encode_len;
    return out;
}

// ssz decode [u8] to struct/asset
// val: [1, 11, 0, 0, 0, 3, 4, 13, 0, 0, 0, a, a, 8, 0, 0, 0, 13, 0, 0, 0, h, e,
// l, l, o, w, o, r, l ,d] ty: struct Foo {
//     a: u8
//     b: str
//     c: [u8;2]
//     d: [str]
// }
// 1. Split val to fix_fields and var_items by field_ty, use [] to represent
// variable len field and store their (offset, idx_in_field) to offsets fields:
// [[1],[], [3, 4], []]
// In fact, use (offset_in_val, length) to represent field
// data: [(0, 1), (1, 0), (5, 2), (7, 0)]
// offsets [(11, 1), (13, 3)]
// 2. Calculate length of variable len field by offsets
// length = (offsets[i + 1].0 - offsets[i].0) or (val_length - offsets[-1].0)
// fileds[offsets[i].1] = (offsets[i].0, length)
// fields: [(0, 1), (11, 2), [5, 2], [13, 18]], which value is
// [[1], [a, a], [3, 4], [8, 0, 0, 0, 13, 0, 0, 0, h, e, l, l, o, w, o, r, l
// ,d]]
// 3. Iter struct.fields and decode(field_ty, val + fields[i].0, fields[i].1) to
// build struct foo {
//     a: u8 = 1 // decode(u8, [1], 1)
//     b: str = "aa" // decode(str, [a, a], 2)
//     c: [u8;2] = [3, 4] // decode([u8;2], [3, 4], 2)
//     d: [str] = ["hello", "world"] // decode([str], [8, 0, 0, 0, 13, 0, 0, 0,
//     h, e, l, l, o, w, o, r, l ,d], 18)
// }
void *
ir_builtin_ssz_decode_struct_like_ty(uint32_t runtime_class_offset,
                                        bool allow_empty_object, qvector_t *val)
{
    if (val == NULL && !allow_empty_object) {
        char msg[] = "ssz decode empty bytes failed";
        IR_ABORT(msg, sizeof(msg) / sizeof(char));
        return NULL;
    }
    uint32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    uint32_t fields_count = runtime_class->struct_fields_count;
    uint32_t *fields_offsets_array =
        (uint32_t *)(all_runtimes_classes_address
                     + runtime_class->struct_fields);

    // 1. Split data to fix_fields and var_items by field_ty, use [] to
    // represent variable len field and store their (offset, idx_in_field) to
    // offsets fields:
    // [[1],[], [3, 4], []]
    // In fact, use (offset_in_data, length) to represent field data:
    // [(0, 1), (1, 0), (5, 2), (7, 0)]
    // offsets: [(11, 1), (13, 3)]
    uint32_t fields[fields_count][2];
    uint32_t offsets[fields_count][2];
    uint32_t variable_len_field_count = 0;
    uint32_t offset = 0;

    for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
        uint32_t field_offset = fields_offsets_array[i];
        if (is_ssz_fixed_len(field_offset)) {
            uint32_t field_len = ssz_fix_ty_length(field_offset);
            fields[i][0] = offset;
            fields[i][1] = field_len;
            offset += field_len;
        }
        else {
            if (val) {
                uint32_t *field_offset = (uint32_t *)(val->data + offset);
                offsets[variable_len_field_count][0] = *field_offset;
            }
            offsets[variable_len_field_count][1] = i;
            offset += 4;
            variable_len_field_count += 1;
        }
    }

    // 2. Calculate length of variable len field by offsets
    // length = (offsets[i + 1].0 - offsets[i].0) or (val_length -
    // offsets[-1].0) fileds[offsets[i].1] = (offsets[i].0, length) fields:
    // [(0, 1), (11, 2), [5, 2], [13, 18]], which value is
    // [[1], [a, a], [3, 4], [8, 0, 0, 0, 13, 0, 0, 0, h, e, l, l, o, w, o, r, l
    // ,d]]

    // If all fields in struct/asset is fixed len, skip step 2.
    if (variable_len_field_count > 0) {
        for (uint32_t i = 0; i < variable_len_field_count - 1; i++) {
            fields[offsets[i][1]][0] = offsets[i][0];
            fields[offsets[i][1]][1] = offsets[i + 1][0] - offsets[i][0];
        }
        // last variable len field
        fields[offsets[variable_len_field_count - 1][1]][0] =
            offsets[variable_len_field_count - 1][0];
        fields[offsets[variable_len_field_count - 1][1]][1] =
            val->num - offsets[variable_len_field_count - 1][0];
    }

    // 3. Iter struct.fields and decode(field_ty, data + fields[i].0,
    // fields[i].1) to build struct
    size_t value_size = calculate_ir_type_size(runtime_class);
    void *ret = __malloc(value_size);
    __memset(ret, 0x0, value_size);
    offset = 0;

    for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
        uint32_t field_offset = fields_offsets_array[i];
        struct IRRuntimeClass *field_type =
            (struct IRRuntimeClass *)(all_runtimes_classes_address
                                         + field_offset);
        if (val == NULL) {
            void *field = ir_builtin_create_ir_value(field_offset);
            uint32_t field_size_as_element =
                get_ir_type_size_as_element(field_type);

            // if field is object pointer, then writer the pinter address into
            // the slot
            if (is_pointer_ir_type(field_type->ty)) {
                uint32_t *to_store = (uint32_t *)(ret + offset);
                *to_store = (uint32_t)field;
            }
            else {
                memcpy(ret + offset, field, field_size_as_element);
            }

            offset += field_size_as_element;
            continue;
        }

        qvector_t *field_u8_arr =
            qvector(fields[i][1], 1, QVECTOR_RESIZE_DOUBLE);

        memcpy(field_u8_arr->data, val->data + fields[i][0], fields[i][1]);
        field_u8_arr->num = fields[i][1];
        void *field =
            ir_builtin_ssz_decode_impl(field_offset, false, field_u8_arr);

        void *field_ptr = get_ptr_of_ptr_value(field_offset, field);

        uint32_t field_size_as_element =
            get_ir_type_size_as_element(field_type);
        memcpy(ret + offset, field_ptr, field_size_as_element);
        offset += field_size_as_element;
    }
    return (void *)ret;
}

// ssz encode a ir type to void ptr
void *
ir_builtin_ssz_encode_void_ptr(uint32_t runtime_class_offset, void *val)
{
    // TODO: use bytes_stream with bytes slice to optimize nested ssz encode
    qvector_t *out = ir_builtin_ssz_encode(runtime_class_offset, val);
    return out->data;
}

// ssz encode a ir type to [u8]
qvector_t *
ir_builtin_ssz_encode(uint32_t runtime_class_offset, void *val)
{
    uint32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);

    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8:
        {
            return ir_builtin_ssz_encode_u8(val);
        } break;
        case IR_RUNTIME_TYPE_U16:
        {
            return ir_builtin_ssz_encode_u16(val);
        } break;
        case IR_RUNTIME_TYPE_U32:
        {
            return ir_builtin_ssz_encode_u32(val);
        } break;
        case IR_RUNTIME_TYPE_U64:
        {
            return ir_builtin_ssz_encode_u64(val);
        } break;
        case IR_RUNTIME_TYPE_U128:
        {
            return ir_builtin_ssz_encode_u128(val);
        } break;
        case IR_RUNTIME_TYPE_U256:
        {
            return ir_builtin_ssz_encode_u256(val);
        } break;
        case IR_RUNTIME_TYPE_I8:
        {
            return ir_builtin_ssz_encode_i8(val);
        } break;
        case IR_RUNTIME_TYPE_I16:
        {
            return ir_builtin_ssz_encode_i16(val);
        } break;
        case IR_RUNTIME_TYPE_I32:
        {
            return ir_builtin_ssz_encode_i32(val);
        } break;
        case IR_RUNTIME_TYPE_I64:
        {
            return ir_builtin_ssz_encode_i64(val);
        } break;
        case IR_RUNTIME_TYPE_I128:
        {
            return ir_builtin_ssz_encode_i128(val);
        } break;
        case IR_RUNTIME_TYPE_I256:
        {
            return ir_builtin_ssz_encode_i256(val);
        } break;
        case IR_RUNTIME_TYPE_BOOL:
        {
            return ir_builtin_ssz_encode_bool(val);
        } break;
        case IR_RUNTIME_TYPE_STR:
        {
            return ir_builtin_ssz_encode_str(val);
        } break;
        case IR_RUNTIME_TYPE_ASSET:
        {
            return ir_builtin_ssz_encode_struct_like_ty(runtime_class_offset,
                                                           val);
        } break;
        case IR_RUNTIME_TYPE_STRUCT:
        {
            return ir_builtin_ssz_encode_struct_like_ty(runtime_class_offset,
                                                           val);
        } break;
        case IR_RUNTIME_TYPE_ARRAY:
        {
            return ir_builtin_ssz_encode_array(runtime_class_offset, val);
        } break;
        default:
        {
            char msg[] = "unknown ir runtime type in ssz type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

// ssz decode a void ptr and length to ir type value
// when ssz decode asset, maybe the asset exists but has no asset data(not set
// yet. eg. after transfer_asset) so asset should allow ssz decode allow empty
// object
void *
ir_builtin_ssz_decode_void_ptr(uint32_t runtime_class_offset,
                                  bool allow_empty_object, void *val,
                                  uint32_t data_len)
{
    if (data_len == 0 && !allow_empty_object) {
        char msg[] = "ssz decode can't decode empty bytes";
        IR_ABORT(msg, (sizeof(msg) / sizeof(char)) - 1);
    }
    qvector_t *u8_vec = NULL;
    if (data_len > 0) {
        u8_vec = qvector(data_len, 1, QVECTOR_RESIZE_DOUBLE);
        memcpy(u8_vec->data, val, data_len);
        u8_vec->num = data_len;
    }
    return ir_builtin_ssz_decode_impl(runtime_class_offset,
                                         allow_empty_object, u8_vec);
}

// ssz decode a [u8] to ir type value
void *
ir_builtin_ssz_decode(uint32_t runtime_class_offset, qvector_t *val)
{
    return ir_builtin_ssz_decode_impl(runtime_class_offset, false, val);
}

// ssz decode a [u8] to ir type value
void *
ir_builtin_ssz_decode_impl(uint32_t runtime_class_offset,
                              bool allow_empty_object, qvector_t *val)
{
    uint32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);

    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8:
        {
            return ir_builtin_ssz_decode_u8(val);
        } break;
        case IR_RUNTIME_TYPE_U16:
        {
            return ir_builtin_ssz_decode_u16(val);
        } break;
        case IR_RUNTIME_TYPE_U32:
        {
            return ir_builtin_ssz_decode_u32(val);
        } break;
        case IR_RUNTIME_TYPE_U64:
        {
            return ir_builtin_ssz_decode_u64(val);
        } break;
        case IR_RUNTIME_TYPE_U128:
        {
            return ir_builtin_ssz_decode_u128(val);
        } break;
        case IR_RUNTIME_TYPE_U256:
        {
            return ir_builtin_ssz_decode_u256(val);
        } break;
        case IR_RUNTIME_TYPE_I8:
        {
            return ir_builtin_ssz_decode_i8(val);
        } break;
        case IR_RUNTIME_TYPE_I16:
        {
            return ir_builtin_ssz_decode_i16(val);
        } break;
        case IR_RUNTIME_TYPE_I32:
        {
            return ir_builtin_ssz_decode_i32(val);
        } break;
        case IR_RUNTIME_TYPE_I64:
        {
            return ir_builtin_ssz_decode_i64(val);
        } break;
        case IR_RUNTIME_TYPE_I128:
        {
            return ir_builtin_ssz_decode_i128(val);
        } break;
        case IR_RUNTIME_TYPE_I256:
        {
            return ir_builtin_ssz_decode_i256(val);
        } break;
        case IR_RUNTIME_TYPE_BOOL:
        {
            return ir_builtin_ssz_decode_bool(val);
        } break;
        case IR_RUNTIME_TYPE_STR:
        {
            return ir_builtin_ssz_decode_str(val);
        } break;
        case IR_RUNTIME_TYPE_ASSET:
        {
            return ir_builtin_ssz_decode_struct_like_ty(
                runtime_class_offset, allow_empty_object, val);
        } break;
        case IR_RUNTIME_TYPE_STRUCT:
        {
            return ir_builtin_ssz_decode_struct_like_ty(
                runtime_class_offset, allow_empty_object, val);
        } break;
        case IR_RUNTIME_TYPE_ARRAY:
        {
            return ir_builtin_ssz_decode_array(runtime_class_offset,
                                                  allow_empty_object, val);
        } break;
        default:
        {
            char msg[] = "unknown ir runtime type in ssz type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

void *__attribute__((artificial)) __attribute__((always_inline))
ir_builtin_versioned_ssz_get_data_ptr(uint32_t data_ptr, uint32_t data_len,
                                         bool is_versioned,
                                         uint32_t ssz_version_size)
{
    void *data = (void *)data_ptr;
    if (data == NULL || data_len == 0) {
        return NULL;
    }
    if (is_versioned) {
        data_len -= ssz_version_size;
        data = ptr_offset(data, ssz_version_size);
    }
    return data;
}

uint32_t __attribute__((artificial)) __attribute__((always_inline))
ir_builtin_versioned_ssz_get_data_len(uint32_t data_len, bool is_versioned,
                                         uint32_t ssz_version_size)
{
    if (data_len == 0) {
        return 0;
    }
    if (is_versioned) {
        return data_len - ssz_version_size;
    }
    else {
        return data_len;
    }
}