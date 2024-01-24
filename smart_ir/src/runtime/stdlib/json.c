// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./json.h"
#include "./ir_type.h"
#include "./qhashtbl.h"
#include "./stdlib.h"
#include "./cJSON.h"
#include "./ssz.h"
#include "./qvector.h"
#include "math.h"

extern uint32_t
get_ir_type_size_as_element(struct IRRuntimeClass *runtime_class);

extern intptr_t
get_all_runtimes_classes_address();

void *
get_array_elem_ptr_at_idx(uint32_t runtime_class_offset, void *val,
                          uint32_t idx);

void *
get_data_ptr_of_ptr_value(uint32_t runtime_class_offset, void *val);

void *
get_data_ptr_of_ptr_value(uint32_t runtime_class_offset, void *val);

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

extern struct cJSON *
ir_type_to_cjson(uint32_t runtime_class_offset, void *val);

extern void *
cjson_to_ir_type(uint32_t runtime_class_offset, cJSON *obj);

#define IR_BUILTIN_JSON_ENCODE_INT_DECLARE(id, ty)         \
    cJSON *ir_builtin_json_encode_##id(void *val)          \
    {                                                         \
        ty v = *(ty *)val;                                    \
        cJSON *ret = cJSON_CreateNumber((uint256_t)v, (int256_t) v < (int256_t) 0); \
        return ret;                                           \
    }

cJSON *
ir_builtin_json_encode_bool(void *val)
{
    bool v = *(bool *)val;
    cJSON *ret = cJSON_CreateBool(v);
    return ret;
}

IR_BUILTIN_JSON_ENCODE_INT_DECLARE(u8, uint8_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(u16, uint16_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(u32, uint32_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(u64, uint64_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(u128, uint128_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(u256, uint256_t)

IR_BUILTIN_JSON_ENCODE_INT_DECLARE(i8, int8_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(i16, int16_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(i32, int32_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(i64, int64_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(i128, int128_t)
IR_BUILTIN_JSON_ENCODE_INT_DECLARE(i256, int256_t)

cJSON *
ir_builtin_json_encode_str(void *val)
{
    struct vector *v = (struct vector *)val;
    cJSON *ret = cJSON_CreateString(v->data);
    return ret;
}

cJSON *
ir_builtin_json_encode_array(uint32_t runtime_class_offset, void *val)
{
    uint32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    qvector_t *v = (qvector_t *)val;
    uint32_t elem_ty_offset = runtime_class->array_item_ty;
    cJSON *ret = cJSON_CreateArray();
    for (uint32_t i = 0; i < v->num; i++) {
        void *elem_ptr =
            get_array_elem_ptr_at_idx(runtime_class_offset, val, i);
        cJSON *elem_cjson_obj = ir_type_to_cjson(elem_ty_offset, elem_ptr);
        cJSON_AddItemToArray(ret, elem_cjson_obj);
    }
    return ret;
}

cJSON *
ir_builtin_json_encode_struct_like_ty(uint32_t runtime_class_offset,
                                         void *val)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    uint32_t *fields_offsets_array =
        (uint32_t *)(all_runtimes_classes_address
                     + runtime_class->struct_fields);
    uint32_t *fields_names_array =
        (uint32_t *)(all_runtimes_classes_address
                     + runtime_class->struct_field_names);
    cJSON *ret = cJSON_CreateObject();

    uint32_t ptr_offset = 0;
    for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
        uint32_t field_offset = fields_offsets_array[i];

        struct IRRuntimeClass *field_type =
            (struct IRRuntimeClass *)(all_runtimes_classes_address
                                         + field_offset);

        void *field_ptr =
            get_data_ptr_of_ptr_value(field_offset, val + ptr_offset);
        cJSON *field_cjson_obj = ir_type_to_cjson(field_offset, field_ptr);

        uint32_t fields_name_offset = fields_names_array[i];
        struct vector *fields_name_ptr =
            (struct vector *)(all_runtimes_classes_address
                              + fields_name_offset);
        uint32_t fields_name_bytes_offset = (uint32_t)fields_name_ptr->data;
        uint32_t field_name_bytes =
            all_runtimes_classes_address + fields_name_bytes_offset;
        char *ptr = (char *)field_name_bytes;

        char *field_name = __malloc(fields_name_ptr->len + 1);
        memcpy(field_name, ptr, fields_name_ptr->len);
        __memset(field_name + fields_name_ptr->len, '\0', 1);

        cJSON_AddItemToObject(ret, field_name, field_cjson_obj);

        ptr_offset += get_ir_type_size_as_element(field_type);
    }
    return ret;
}

cJSON *
ir_builtin_json_encode_map(uint32_t runtime_class_offset, void *val)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    cJSON *ret = cJSON_CreateObject();
    qhashtbl_obj_t item;
    __memset((void *)&item, 0, sizeof(item));
    qhashtbl_t *v = (qhashtbl_t *)val;
    while (qhashtbl_getnext(v, &item, true)) {
        void *val_ptr =
            get_data_ptr_of_ptr_value(runtime_class->map_value_ty, item.data);
        char *key = (char *)item.key;
        if (TABLE_KEY_IS_INT(v)) {
            key = builtin_i64_toa(item.key, 10);
        }
        cJSON *val_cjson_obj =
            ir_type_to_cjson(runtime_class->map_value_ty, val_ptr);
        cJSON_AddItemToObject(ret, (char *)key, val_cjson_obj);
    }

    return ret;
}

cJSON *
ir_type_to_cjson(uint32_t runtime_class_offset, void *val)
{
    uint32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);

    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8:
        {
            return ir_builtin_json_encode_u8(val);
        } break;
        case IR_RUNTIME_TYPE_U16:
        {
            return ir_builtin_json_encode_u16(val);
        } break;
        case IR_RUNTIME_TYPE_U32:
        {
            return ir_builtin_json_encode_u32(val);
        } break;
        case IR_RUNTIME_TYPE_U64:
        {
            return ir_builtin_json_encode_u64(val);
        } break;
        case IR_RUNTIME_TYPE_U128:
        {
            return ir_builtin_json_encode_u128(val);
        } break;
        case IR_RUNTIME_TYPE_U256:
        {
            return ir_builtin_json_encode_u256(val);
        } break;
        case IR_RUNTIME_TYPE_I8:
        {
            return ir_builtin_json_encode_i8(val);
        } break;
        case IR_RUNTIME_TYPE_I16:
        {
            return ir_builtin_json_encode_i16(val);
        } break;
        case IR_RUNTIME_TYPE_I32:
        {
            return ir_builtin_json_encode_i32(val);
        } break;
        case IR_RUNTIME_TYPE_I64:
        {
            return ir_builtin_json_encode_i64(val);
        } break;
        case IR_RUNTIME_TYPE_I128:
        {
            return ir_builtin_json_encode_i128(val);
        } break;
        case IR_RUNTIME_TYPE_I256:
        {
            return ir_builtin_json_encode_i256(val);
        } break;
        case IR_RUNTIME_TYPE_BOOL:
        {
            return ir_builtin_json_encode_bool(val);
        } break;
        case IR_RUNTIME_TYPE_STR:
        {
            return ir_builtin_json_encode_str(val);
        } break;

        case IR_RUNTIME_TYPE_STRUCT:
        {
            return ir_builtin_json_encode_struct_like_ty(
                runtime_class_offset, val);
        } break;
        case IR_RUNTIME_TYPE_ASSET:
        {
            return ir_builtin_json_encode_struct_like_ty(
                runtime_class_offset, val);
        } break;

        case IR_RUNTIME_TYPE_ARRAY:
        {
            return ir_builtin_json_encode_array(runtime_class_offset, val);
        } break;

        case IR_RUNTIME_TYPE_MAP:
        {
            return ir_builtin_json_encode_map(runtime_class_offset, val);
        } break;

        default:
        {
            char msg[] = "unknown ir runtime type in json type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

#define IR_BUILTIN_JSON_DECODE_INT_DECLARE(id, ty, size, sign)             \
    void *ir_builtin_json_decode_##id(cJSON *obj)                          \
    {                                                                         \
        if (!cJSON_IsNumber(obj)) {                                           \
            char msg[] = "json decode error: not a valid number";             \
            IR_ABORT(msg, sizeof(msg) - 1);                                \
        }                                                                     \
        if (!sign && obj->negsign) {                                          \
            char msg[] = "json decode error: expect uint, but got int value"; \
            IR_ABORT(msg, sizeof(msg) - 1);                                \
        }                                                                     \
        ty *ret = malloc(size);                                               \
        *ret = (ty)cJSON_GetNumberValue(obj);                                 \
        return (void *)ret;                                                   \
    }

void *
ir_builtin_json_decode_bool(cJSON *obj)
{
    if (!cJSON_IsBool(obj)) {
        char msg[] = "json decode error: not a valid bool";
        IR_ABORT(msg, sizeof(msg) - 1);
    }
    bool *ret = malloc(1);
    *ret = (obj->type == cJSON_True);
    return ret;
}

IR_BUILTIN_JSON_DECODE_INT_DECLARE(u8, uint8_t, 1, false)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(u16, uint16_t, 2, false)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(u32, uint32_t, 4, false)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(u64, uint64_t, 8, false)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(u128, uint128_t, 16, false)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(u256, uint256_t, 32, false)

IR_BUILTIN_JSON_DECODE_INT_DECLARE(i8, int8_t, 1, true)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(i16, int16_t, 2, true)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(i32, int32_t, 4, true)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(i64, int64_t, 8, true)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(i128, int128_t, 16, true)
IR_BUILTIN_JSON_DECODE_INT_DECLARE(i256, int256_t, 32, true)

void *
ir_builtin_json_decode_str(cJSON *obj)
{
    if (!cJSON_IsString(obj)) {
        char msg[] = "json decode error: not a valid string";
        IR_ABORT(msg, sizeof(msg) - 1);
    }
    char *s = cJSON_GetStringValue(obj);
    struct vector *ret = vector_new(__strlen(s), 1, 0);
    memcpy(ret->data, s, __strlen(s));
    return (void *)ret;
}

void *
ir_builtin_json_decode_array(uint32_t runtime_class_offset, cJSON *obj)
{
    if (!cJSON_IsArray(obj)) {
        char msg[] = "json decode error: not a valid array";
        IR_ABORT(msg, sizeof(msg) - 1);
    }
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);

    assert(runtime_class->ty = IR_RUNTIME_TYPE_ARRAY);
    struct IRRuntimeClass *elem_ty =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class->array_item_ty);

    int array_size = cJSON_GetArraySize(obj);

    uint32_t elem_size = get_ir_type_size_as_element(elem_ty);
    qvector_t *ret = qvector(array_size, elem_size, QVECTOR_RESIZE_DOUBLE);
    ret->num = array_size;
    for (uint32_t i = 0; i < array_size; i++) {
        cJSON *elem_obj = cJSON_GetArrayItem(obj, i);
        void *elem =
            cjson_to_ir_type(runtime_class->array_item_ty, elem_obj);
        void *elem_ptr =
            get_ptr_of_ptr_value(runtime_class->array_item_ty, elem);
        memcpy(ret->data + i * elem_size, elem_ptr, elem_size);
    }

    return (void *)ret;
}

void *
ir_builtin_json_decode_struct_like_ty(uint32_t runtime_class_offset,
                                         cJSON *obj)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    uint32_t *fields_offsets_array =
        (uint32_t *)(all_runtimes_classes_address
                     + runtime_class->struct_fields);
    uint32_t *fields_names_array =
        (uint32_t *)(all_runtimes_classes_address
                     + runtime_class->struct_field_names);

    size_t value_size = calculate_ir_type_size(runtime_class);
    void *ret = __malloc(value_size);
    uint32_t offset = 0;

    for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
        uint32_t field_offset = fields_offsets_array[i];
        cJSON *field_obj = cJSON_GetArrayItem(obj, i);

        struct IRRuntimeClass *field_type =
            (struct IRRuntimeClass *)(all_runtimes_classes_address
                                         + field_offset);

        // compare field name
        uint32_t fields_name_offset = fields_names_array[i];
        struct vector *fields_name_ptr =
            (struct vector *)(all_runtimes_classes_address
                              + fields_name_offset);
        uint32_t fields_name_bytes_offset = (uint32_t)fields_name_ptr->data;
        uint32_t field_name_bytes =
            all_runtimes_classes_address + fields_name_bytes_offset;
        char *ptr = (char *)field_name_bytes;

        char *field_name = __malloc(fields_name_ptr->len + 1);
        memcpy(field_name, ptr, fields_name_ptr->len);
        __memset(field_name + fields_name_ptr->len, '\0', 1);

        if (strncmp(ptr, field_obj->string, __strlen(field_obj->string))) {
            char msg[] = "json decode error: struct field name not match";
            IR_ABORT(msg, sizeof(msg) - 1);
        }

        void *field = cjson_to_ir_type(field_offset, field_obj);

        void *field_ptr = get_ptr_of_ptr_value(field_offset, field);

        uint32_t field_size_as_element =
            get_ir_type_size_as_element(field_type);
        memcpy(ret + offset, field_ptr, field_size_as_element);
        offset += field_size_as_element;
    }
    return (void *)ret;
}

void *
ir_builtin_json_decode_map(uint32_t runtime_class_offset, cJSON *obj)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    struct IRRuntimeClass *key_runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class->map_key_ty);
    qhashtbl_t *ret = qhashtbl(0, key_runtime_class->ty, 0);

    struct IRRuntimeClass *value_ty =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);

    cJSON *elem = NULL;
    cJSON_ArrayForEach(elem, obj)
    {

        qhashtbl_obj_t *item =
            cjson_to_ir_type(runtime_class->map_value_ty, elem);
        char *key = elem->string;
        if (TABLE_KEY_IS_INT(ret)) {
            struct vector *int_str = vector_new(__strlen(key), 1, key);
            if (ret->key_runtime_ty <= IR_RUNTIME_TYPE_U256) {
                key = (char *)ir_builtin_str_to_u256(int_str);
            }
            else {
                key = (char *)ir_builtin_str_to_i256(int_str);
            }
        }
        qhashtbl_put(ret,(int64_t) key, (void *)item,
                     get_ir_type_size_as_element(value_ty));
    }
    return (void *)ret;
}

void *
cjson_to_ir_type(uint32_t runtime_class_offset, cJSON *obj)
{
    uint32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);

    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8:
        {
            return ir_builtin_json_decode_u8(obj);
        } break;
        case IR_RUNTIME_TYPE_U16:
        {
            return ir_builtin_json_decode_u16(obj);
        } break;
        case IR_RUNTIME_TYPE_U32:
        {
            return ir_builtin_json_decode_u32(obj);
        } break;
        case IR_RUNTIME_TYPE_U64:
        {
            return ir_builtin_json_decode_u64(obj);
        } break;
        case IR_RUNTIME_TYPE_U128:
        {
            return ir_builtin_json_decode_u128(obj);
        } break;
        case IR_RUNTIME_TYPE_U256:
        {
            return ir_builtin_json_decode_u256(obj);
        } break;
        case IR_RUNTIME_TYPE_I8:
        {
            return ir_builtin_json_decode_i8(obj);
        } break;
        case IR_RUNTIME_TYPE_I16:
        {
            return ir_builtin_json_decode_i16(obj);
        } break;
        case IR_RUNTIME_TYPE_I32:
        {
            return ir_builtin_json_decode_i32(obj);
        } break;
        case IR_RUNTIME_TYPE_I64:
        {
            return ir_builtin_json_decode_i64(obj);
        } break;
        case IR_RUNTIME_TYPE_I128:
        {
            return ir_builtin_json_decode_i128(obj);
        } break;
        case IR_RUNTIME_TYPE_I256:
        {
            return ir_builtin_json_decode_i256(obj);
        } break;
        case IR_RUNTIME_TYPE_BOOL:
        {
            return ir_builtin_json_decode_bool(obj);
        } break;
        case IR_RUNTIME_TYPE_STR:
        {
            return ir_builtin_json_decode_str(obj);
        } break;

        case IR_RUNTIME_TYPE_STRUCT:
        {
            return ir_builtin_json_decode_struct_like_ty(
                runtime_class_offset, obj);
        } break;
        case IR_RUNTIME_TYPE_ASSET:
        {
            return ir_builtin_json_decode_struct_like_ty(
                runtime_class_offset, obj);
        } break;

        case IR_RUNTIME_TYPE_ARRAY:
        {
            return ir_builtin_json_decode_array(runtime_class_offset, obj);
        } break;

        case IR_RUNTIME_TYPE_MAP:
        {
            return ir_builtin_json_decode_map(runtime_class_offset, obj);
        } break;

        default:
        {
            char msg[] = "unknown ir runtime type in json type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

// json encode a ir type to str
extern struct vector *
ir_builtin_json_encode(uint32_t runtime_class_offset, void *val)
{
    cJSON *cjson_obj = ir_type_to_cjson(runtime_class_offset, val);
    char *s = cJSON_Print(cjson_obj);
    struct vector *ret = vector_new(__strlen(s), 1, 0);
    memcpy(ret->data, s, ret->len);
    return ret;
}

extern void *
ir_builtin_json_decode(uint32_t runtime_class_offset, struct vector *val)
{
    cJSON *cjson_obj = cJSON_Parse(val->data);

    void *ret = cjson_to_ir_type(runtime_class_offset, cjson_obj);
    return ret;
}
