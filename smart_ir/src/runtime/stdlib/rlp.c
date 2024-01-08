// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./rlp.h"
#include "./ir_type.h"

// spec: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/

size_t
rlp_encode_int(ByteStream *stream, uint64_t value);

size_t
rlp_encode_int(ByteStream *stream, uint64_t value) {
    if (value == 0) {
        return 0;
    }
    size_t count1 = rlp_encode_int(stream, value / 256);
    byte_stream_write_byte(stream, (int8_t) (value % 256));
    return count1 + 1;
}

// just little endian encode integer to bytes
// uint is encoded same as int
static size_t rlp_simple_encode_uint64_to_bytes(uint64_t value, uint8_t *buf) {
    if (value == 0) {
        return 0; // empty bytes means 0
    }
    // the wasm is little endian, so just memcpy
    const size_t max_bytes_size = sizeof(uint64_t);
    uint8_t max_buf[max_bytes_size];
    uint64_t *value_mem = (uint64_t*) __malloc(sizeof(uint64_t));
    *value_mem = value;
    memcpy(max_buf, value_mem, sizeof(uint64_t));
    size_t bytes_count = max_bytes_size; // at least 1
    for (int32_t i=max_bytes_size-1;i>0;i--) { // at least need one byte
        if (max_buf[i] != 0) {
            break;
        }
        bytes_count--;
    }
    // buf is reverse of max_buf, but only contains needed data
    for (size_t i=0;i<bytes_count;i++) {
        buf[i] = max_buf[bytes_count-i-1];
    }
    return bytes_count;
}

static size_t rlp_simple_encode_uint128_to_bytes(uint128_t value, uint8_t *buf) {
    if (value == 0) {
        return 0; // empty bytes means 0
    }
    // the wasm is little endian, so just memcpy
    const size_t max_bytes_size = sizeof(uint128_t);
    uint8_t max_buf[max_bytes_size];
    uint128_t *value_mem = (uint128_t*) __malloc(sizeof(uint128_t));
    *value_mem = value;
    memcpy(max_buf, value_mem, sizeof(uint128_t));
    size_t bytes_count = max_bytes_size; // at least 1
    for (int32_t i=max_bytes_size-1;i>0;i--) { // at least need one byte
        if (max_buf[i] != 0) {
            break;
        }
        bytes_count--;
    }
    // buf is reverse of max_buf, but only contains needed data
    for (size_t i=0;i<bytes_count;i++) {
        buf[i] = max_buf[bytes_count-i-1];
    }
    return bytes_count;
}

static size_t rlp_simple_encode_uint256_to_bytes(uint256_t value, uint8_t *buf) {
    if (value == (uint256_t) 0) {
        return 0; // empty bytes means 0
    }
    // the wasm is little endian, so just memcpy
    const size_t max_bytes_size = sizeof(uint256_t);
    uint8_t max_buf[max_bytes_size];
    uint256_t *value_mem = (uint256_t*) __malloc(sizeof(uint256_t));
    *value_mem = value;
    memcpy(max_buf, value_mem, sizeof(uint256_t));
    size_t bytes_count = max_bytes_size; // at least 1
    for (int32_t i=max_bytes_size-1;i>0;i--) { // at least need one byte
        if (max_buf[i] != 0) {
            break;
        }
        bytes_count--;
    }
    // buf is reverse of max_buf, but only contains needed data
    for (size_t i=0;i<bytes_count;i++) {
        buf[i] = max_buf[bytes_count-i-1];
    }
    return bytes_count;
}

// int128 is slower than int64, and int64 is native supported in wasm. so must ints use int64 to decode
static uint64_t rlp_simple_decode_uint64(uint8_t *buf, size_t len) {
    uint64_t sum = 0;
    uint64_t multiply = 1;
    for (int32_t i=len-1;i>=0;i--) { // i must be int32_t, otherwise loop forever
        sum += multiply * (int64_t) buf[i];
        if (i >= 1) {
            multiply *= 256; // 1 << 8
        }
    }
    return sum;
}

static int128_t rlp_simple_decode_uint128(uint8_t *buf, size_t len) {
    uint128_t sum = 0;
    uint128_t multiply = 1;
    for (int32_t i=len-1;i>=0;i--) { // i must be int32_t, otherwise loop forever
        sum += multiply * (int128_t) buf[i];
        if (i >= 1) {
            multiply *= 256; // 1 << 8
        }
    }
    return sum;
}

static uint256_t rlp_simple_decode_uint256(uint8_t *buf, size_t len) {
    uint256_t sum = 0;
    uint256_t multiply = 1;
    for (int32_t i=len-1;i>=0;i--) { // i must be int32_t, otherwise loop forever
        sum +=(uint256_t) multiply * (uint256_t) buf[i];
        if (i >= 1) {
            multiply *= (uint256_t)256; // 1 << 8
        }
    }
    return sum;
}

static
inline void __attribute__((artificial)) __attribute__((always_inline))
rlp_encode_length(ByteStream *stream, size_t len, size_t offset) {
    if (len < 56) {
        int8_t b = (int8_t) (len + offset);
        byte_stream_write_byte(stream, b);
        return;
    }
    if (len < UINT32_MAX) { // 256 ** 8 is too large for int
        ByteStream *len_bs = new_byte_stream();
        size_t count1 = rlp_encode_int(len_bs, len);
        int8_t b = (int8_t) (count1 + offset + 55);
        byte_stream_write_byte(stream, b);
        byte_stream_write_bytes(stream, len_bs->data, len_bs->len);
        free_byte_stream(len_bs);
        return;
    }
    char msg[] = "rlp input too long";
    IR_ABORT(msg, sizeof(msg) - 1);
}

void rlp_encode_str(ByteStream *stream, struct vector *value) {
    uint8_t *value_data = (uint8_t *) value->data;
    if (value->len == 1 && (value_data[0]) < 0x80) {
        byte_stream_write_byte(stream, value_data[0]);
        return;
    }
    rlp_encode_length(stream, value->len, 0x80);
    byte_stream_write_bytes(stream, (int8_t *) value_data, value->len);
}

void rlp_encode_bytes(ByteStream *stream, qvector_t *value) {
    uint8_t *value_data = (uint8_t *) value->data;
    if (value->num == 1 && ((uint8_t) value_data[0]) < 0x80) {
        byte_stream_write_byte(stream, value_data[0]);
        return;
    }
    rlp_encode_length(stream, value->num, 0x80);
    byte_stream_write_bytes(stream, (int8_t *) value_data, value->num);
}

void rlp_encode_str_list(ByteStream *stream, qvector_t *list) {
    struct RuntimeContext ctx;
    ctx.file_name = "rlp.c";
    ctx.line = __LINE__;
    ctx.col = 0;
    ByteStream *content_bs = new_byte_stream();
    for (size_t i = 0; i < list->num; i++) {
        struct vector **item_addr = (struct vector **) qvector_getat(list, i, false, &ctx);
        struct vector *item = *item_addr;
        rlp_encode_str(content_bs, item);
    }
    rlp_encode_length(stream, content_bs->len, 0xc0);
    byte_stream_write_bytes(stream, content_bs->data, content_bs->len);
}

void rlp_encode_bytes_list(ByteStream *stream, qvector_t *list) {
    if (list->num == 0) {
        return;
    }
    struct RuntimeContext ctx;
    ctx.file_name = "rlp.c";
    ctx.line = __LINE__;
    ctx.col = 0;
    ByteStream *content_bs = new_byte_stream();
    for (size_t i = 0; i < list->num; i++) {
        qvector_t **item_addr = (qvector_t **) qvector_getat(list, i, false, &ctx);
        qvector_t *item = *item_addr;
        rlp_encode_bytes(content_bs, item);
    }
    rlp_encode_length(stream, content_bs->len, 0xc0);
    byte_stream_write_bytes(stream, content_bs->data, content_bs->len);
}

typedef enum RlpValueType {
    RLP_STR,
    RLP_LIST
} RlpValueType;

static int64_t rlp_bytes_to_int(qvector_t *bs) {
    size_t len = bs->num;
    uint8_t *bs_data = ((uint8_t *) bs->data);
    if (len == 0) {
        char msg[] = "not enough rlp bytes";
        IR_ABORT(msg, sizeof(msg) - 1);
        return 0;
    }
    if (len == 1) {
        return (int64_t) bs_data[0];
    }
    qvector_t slice;
    memcpy(&slice, bs, sizeof(qvector_t));
    slice.num -= 1;
    int64_t remaining_int = 256 * rlp_bytes_to_int(&slice);
    return (int64_t) (bs_data[len - 1]) + remaining_int;
}

static size_t rlp_decode_length(ByteStream *stream, RlpValueType *rpl_value_type /* out, not-null */) {
    size_t len = byte_stream_read_remaining_length(stream);
    if (len == 0) {
        char msg[] = "rlp decode input not enough length";
        IR_ABORT(msg, sizeof(msg) - 1);
        return 0;
    }
    uint8_t prefix = (uint8_t) (stream->data[stream->read_offset]);
    size_t prefix_size_value = (size_t) prefix;
    if (prefix <= 0x7f) {
        *rpl_value_type = RLP_STR;
        // read offset not change, not consume the first byte
        return 1;
    } else if (prefix <= 0xb7 && (len > (prefix_size_value - 0x80))) {
        *rpl_value_type = RLP_STR;
        stream->read_offset += 1; // consume first byte as length
        return prefix_size_value - 0x80;
    } else if (prefix <= 0xbf && (len > (prefix_size_value - 0xb7)) && (len > (prefix_size_value - 0xb7 +
                                                                               rlp_bytes_to_int(
                                                                                       byte_stream_read_bytes_but_not_consume(
                                                                                               stream, 1,
                                                                                               prefix_size_value -
                                                                                               0xb7))))) {
        size_t lenOfStrLen = prefix_size_value - 0xb7;
        size_t strLen = rlp_bytes_to_int(byte_stream_read_bytes_but_not_consume(stream, 1, lenOfStrLen));
        *rpl_value_type = RLP_STR;
        stream->read_offset += (1 + lenOfStrLen);
        return strLen;
    } else if (prefix <= 0xf7 && (len > (prefix_size_value - 0xc0))) {
        size_t listLen = prefix_size_value - 0xc0;
        *rpl_value_type = RLP_LIST;
        stream->read_offset += 1;
        return listLen;
    } else if (prefix_size_value <= 0xff
               && (len > prefix_size_value - 0xf7)
               && (len > (prefix_size_value - 0xf7 + rlp_bytes_to_int(
            byte_stream_read_bytes_but_not_consume(stream, 1, prefix_size_value - 0xf7))))) {
        size_t lenOfListLen = prefix_size_value - 0xf7;
        size_t listLen = rlp_bytes_to_int(byte_stream_read_bytes_but_not_consume(stream, 1, lenOfListLen));
        *rpl_value_type = RLP_LIST;
        stream->read_offset += (1 + lenOfListLen);
        return listLen;
    }
    char msg[] = "rlp decode length failed";
    IR_ABORT(msg, sizeof(msg) - 1);
    return 0;
}

qvector_t *rlp_decode(ByteStream *stream) {
    if (stream->read_offset >= stream->len) {
        char msg[] = "rlp decode empty bytes";
        IR_ABORT(msg, sizeof(msg) - 1);
        return NULL;
    }

    RlpValueType ty;
    size_t data_len = rlp_decode_length(stream, &ty);
    if (ty == RLP_STR) {
        qvector_t *result = qvector(data_len > 0 ? data_len : 1, 1, QVECTOR_RESIZE_DOUBLE);
        memcpy(result->data, stream->data + stream->read_offset, data_len);
        result->num = data_len;
        stream->read_offset += data_len;
        return result;
    } else if (ty == RLP_LIST) {
        // data_len is bytes len, not list size
        qvector_t *result = qvector(data_len > 0 ? data_len : 1, sizeof(void *), QVECTOR_RESIZE_DOUBLE);
        size_t end_offset = stream->read_offset + data_len;
        for (size_t i = 0; i < data_len && stream->read_offset < end_offset; i++) {
            qvector_t *item = rlp_decode(stream);
            qvector_addlast(result, &item);
        }
        return result;
    } else {
        char msg[] = "unknown rlp value type when decode";
        IR_ABORT(msg, sizeof(msg) - 1);
        return NULL;
    }
}

void common_rlp_encode(ByteStream *stream, uint32_t runtime_class_offset, void *val) {
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
            (struct IRRuntimeClass *) (all_runtimes_classes_address
                                          + runtime_class_offset);
    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8: {
            uint8_t ival = *((uint8_t *) val);
            qvector_t *bytes = qvector(8, 1, QVECTOR_RESIZE_DOUBLE);
            int32_t n = rlp_simple_encode_uint64_to_bytes(ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_U16: {
            uint16_t ival = *((uint16_t *) val);
            qvector_t *bytes = qvector(8, 1, QVECTOR_RESIZE_DOUBLE);
            int32_t n = rlp_simple_encode_uint64_to_bytes(ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_U32: {
            uint32_t ival = *((uint32_t *) val);
            qvector_t *bytes = qvector(8, 1, QVECTOR_RESIZE_DOUBLE);
            int32_t n = rlp_simple_encode_uint64_to_bytes(ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_U64: {
            uint64_t ival = *((uint64_t *) val);
            qvector_t *bytes = qvector(sizeof(uint64_t), 1, QVECTOR_RESIZE_DOUBLE);
            int32_t n = rlp_simple_encode_uint64_to_bytes(ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_U128: {
            uint128_t ival = *((uint128_t *) val);
            qvector_t *bytes = qvector(sizeof(uint128_t), 1, QVECTOR_RESIZE_DOUBLE);
            int32_t n = rlp_simple_encode_uint128_to_bytes(ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_U256: {
            uint256_t ival = *((uint256_t *) val);
            qvector_t *bytes = qvector(sizeof(uint256_t), 1, QVECTOR_RESIZE_DOUBLE);
            int32_t n = rlp_simple_encode_uint256_to_bytes(ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_I8: {
            int8_t ival = *((int8_t *) val);
            qvector_t *bytes = qvector(8, 1, QVECTOR_RESIZE_DOUBLE);
            // when negative, uext to i64
            int32_t n = rlp_simple_encode_uint64_to_bytes((uint8_t) ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_I16: {
            int16_t ival = *((int16_t *) val);
            qvector_t *bytes = qvector(8, 1, QVECTOR_RESIZE_DOUBLE);
            // when negative, uext to i64
            int32_t n = rlp_simple_encode_uint64_to_bytes((uint16_t) ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_I32: {
            int32_t ival = *((int32_t *) val);
            qvector_t *bytes = qvector(8, 1, QVECTOR_RESIZE_DOUBLE);
            // when negative, uext to i64
            int32_t n = rlp_simple_encode_uint64_to_bytes((uint32_t) ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_I64: {
            int64_t ival = *((int64_t *) val);
            qvector_t *bytes = qvector(sizeof(int64_t), 1, QVECTOR_RESIZE_DOUBLE);
            // when negative, uext to i64
            int32_t n = rlp_simple_encode_uint128_to_bytes((uint64_t) ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_I128: {
            int128_t ival = *((int128_t *) val);
            qvector_t *bytes = qvector(sizeof(int128_t), 1, QVECTOR_RESIZE_DOUBLE);
            // TODO: use uint128
            int32_t n = rlp_simple_encode_uint128_to_bytes(ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_I256: {
            int256_t ival = *((int256_t *) val);
            qvector_t *bytes = qvector(sizeof(int256_t), 1, QVECTOR_RESIZE_DOUBLE);
            // TODO: use uint256
            int32_t n = rlp_simple_encode_uint256_to_bytes(ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_BOOL: {
            bool bool_val = *((bool *) val);
            uint8_t ival = bool_val ? 1 : 0;
            qvector_t *bytes = qvector(8, 1, QVECTOR_RESIZE_DOUBLE);
            int32_t n = rlp_simple_encode_uint128_to_bytes(ival, bytes->data);
            bytes->num = n;
            rlp_encode_bytes(stream, bytes);
        }
            break;
        case IR_RUNTIME_TYPE_STR: {
            struct vector *str_val = (struct vector *) val;
            rlp_encode_str(stream, str_val);
        }
            break;
        case IR_RUNTIME_TYPE_ASSET: {
            char msg[] = "asset not supported in ir rlp";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
            break;
        case IR_RUNTIME_TYPE_STRUCT: {
            uint32_t offset = 0;
            uint32_t *fields_offsets_array =
                    (uint32_t *) (all_runtimes_classes_address
                                  + runtime_class->struct_fields);
            // struct is encoded as list in rlp
            ByteStream *content_bs = new_byte_stream();
            for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
                uint32_t field_offset = fields_offsets_array[i];
                struct IRRuntimeClass *field_type =
                        (struct IRRuntimeClass *) (all_runtimes_classes_address
                                                      + field_offset);
                void *field_ptr = val + offset;
                void *field_ptr_value = field_ptr;
                if (field_type->ty == IR_RUNTIME_TYPE_STRUCT || field_type->ty == IR_RUNTIME_TYPE_ASSET) {
                    field_ptr_value = *((void**)field_ptr);
                } else {
                    field_ptr_value = get_data_ptr_of_ptr_value(field_offset, val + offset);
                }
                
                common_rlp_encode(content_bs, field_offset, field_ptr_value);
                offset += get_ir_type_size_as_element(field_type);
            }
            rlp_encode_length(stream, content_bs->len, 0xc0);
            byte_stream_write_bytes(stream, content_bs->data, content_bs->len);
        }
            break;
        case IR_RUNTIME_TYPE_ARRAY: {
            qvector_t *_val = (qvector_t *) val;
            uint32_t elem_ty_offset = runtime_class->array_item_ty;
            struct IRRuntimeClass *elem_type =
                                    (struct IRRuntimeClass *) (all_runtimes_classes_address
                                                                  + elem_ty_offset);
            // if is encode [u8], treat like encode bytes
            // empty array rlp encoded like empty bytes
            if (elem_type->ty == IR_RUNTIME_TYPE_U8 || elem_type->ty == IR_RUNTIME_TYPE_I8) {
                rlp_encode_bytes(stream, _val);
                break;
            }

            ByteStream *content_bs = new_byte_stream();
            for (uint32_t i = 0; i < _val->num; i++) {
                void *elem_ptr =
                        get_array_elem_ptr_at_idx(runtime_class_offset, val, i);
                common_rlp_encode(content_bs, elem_ty_offset, elem_ptr);
            }
            rlp_encode_length(stream, content_bs->len, 0xc0);
            byte_stream_write_bytes(stream, content_bs->data, content_bs->len);
        }
            break;
        case IR_RUNTIME_TYPE_MAP: {
            char msg[] = "map type not supported in ir rlp";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
            break;
        default: {
            char msg[] = "unknown ir runtime type in rlp type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

qvector_t *
ir_builtin_rlp_encode(uint32_t runtime_class_offset, void *val) {
    ByteStream *stream = new_byte_stream();
    common_rlp_encode(stream, runtime_class_offset, val);
    qvector_t *result = byte_stream_to_bytes(stream);
    return result;
}

void *common_rlp_decode(ByteStream *stream, uint32_t runtime_class_offset) {
    uint32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);

    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8: {
            qvector_t *int_bytes = rlp_decode(stream);
            uint32_t value = rlp_simple_decode_uint64(int_bytes->data, int_bytes->num);
            uint8_t *final_value = (uint8_t *) __malloc(sizeof(uint8_t));
            *final_value = value;
            return final_value;
        } break;
        case IR_RUNTIME_TYPE_U16: {
            qvector_t *int_bytes = rlp_decode(stream);
            uint32_t value = rlp_simple_decode_uint64(int_bytes->data, int_bytes->num);
            uint16_t *final_value = (uint16_t *) __malloc(sizeof(uint16_t));
            *final_value = value;
            return final_value;
        } break;
        case IR_RUNTIME_TYPE_U32:
        {
            qvector_t *int_bytes = rlp_decode(stream);
            uint32_t *value = (uint32_t *) __malloc(sizeof(uint32_t));
            *value = rlp_simple_decode_uint64(int_bytes->data, int_bytes->num);
            return value;
        } break;
        case IR_RUNTIME_TYPE_U64:
        {
            qvector_t *int_bytes = rlp_decode(stream);
            uint64_t *value = (uint64_t *) __malloc(sizeof(uint64_t));
            *value = rlp_simple_decode_uint64(int_bytes->data, int_bytes->num);
            return value;
        } break;
        case IR_RUNTIME_TYPE_U128:
        {
            qvector_t *int_bytes = rlp_decode(stream);
            uint128_t *value = (uint128_t *) __malloc(sizeof(uint128_t));
            *value = rlp_simple_decode_uint128(int_bytes->data, int_bytes->num);
            return value;
        } break;
        case IR_RUNTIME_TYPE_U256:
        {
            qvector_t *int_bytes = rlp_decode(stream);
            uint256_t *value = (uint256_t *) __malloc(sizeof(uint256_t));
            *value = rlp_simple_decode_uint256(int_bytes->data, int_bytes->num);
            return value;
        } break;
        case IR_RUNTIME_TYPE_I8: {
            qvector_t *int_bytes = rlp_decode(stream);
            int32_t value = rlp_simple_decode_uint64(int_bytes->data, int_bytes->num);
            int8_t *final_value = (int8_t *) __malloc(sizeof(int8_t));
            *final_value = value;
            return final_value;
        } break;
        case IR_RUNTIME_TYPE_I16: {
            qvector_t *int_bytes = rlp_decode(stream);
            int32_t value = rlp_simple_decode_uint64(int_bytes->data, int_bytes->num);
            int16_t *final_value = (int16_t *) __malloc(sizeof(int16_t));
            *final_value = value;
            return final_value;
        } break;
         case IR_RUNTIME_TYPE_I32:
        {
            qvector_t *int_bytes = rlp_decode(stream);
            int32_t *value = (int32_t *) __malloc(sizeof(int32_t));
            *value = rlp_simple_decode_uint64(int_bytes->data, int_bytes->num);
            return value;
        } break;
        case IR_RUNTIME_TYPE_I64:
        {
            qvector_t *int_bytes = rlp_decode(stream);
            int64_t *value = (int64_t *) __malloc(sizeof(int64_t));
            *value = rlp_simple_decode_uint64(int_bytes->data, int_bytes->num);
            return value;
        } break;
        case IR_RUNTIME_TYPE_I128:
        {
            qvector_t *int_bytes = rlp_decode(stream);
            int128_t *value = (int128_t *) __malloc(sizeof(int128_t));
            *value = rlp_simple_decode_uint128(int_bytes->data, int_bytes->num);
            return value;
        } break;
        case IR_RUNTIME_TYPE_I256:
        {
            qvector_t *int_bytes = rlp_decode(stream);
            int256_t *value = (int256_t *) __malloc(sizeof(int256_t));
            *value = rlp_simple_decode_uint256(int_bytes->data, int_bytes->num);
            return value;
        } break;
        case IR_RUNTIME_TYPE_BOOL:
        {
            qvector_t *int_bytes = rlp_decode(stream);
            int32_t value = rlp_simple_decode_uint64(int_bytes->data, int_bytes->num);
            bool *bool_value = (bool *) __malloc(sizeof(bool));
            *bool_value = (value != 0);
            return bool_value;
        } break;
        case IR_RUNTIME_TYPE_STR:
        {
            qvector_t *decoded_bytes = rlp_decode(stream);
            return vector_new(decoded_bytes->num, 1, decoded_bytes->data);
        } break;
        case IR_RUNTIME_TYPE_ASSET:
        {
            char msg[] = "asset not supported in ir rlp";
            IR_ABORT(msg, sizeof(msg) - 1);
            return NULL;
        } break;
        case IR_RUNTIME_TYPE_STRUCT:
        {
            uint32_t offset = 0;
            uint32_t *fields_offsets_array =
                    (uint32_t *) (all_runtimes_classes_address
                                  + runtime_class->struct_fields);
            void *result = ir_builtin_create_ir_value(runtime_class_offset);
            // struct encoded as list in rlp. so need read lenght first
            RlpValueType ty;
                        size_t data_len = rlp_decode_length(stream, &ty);
            if (ty != RLP_LIST) {
                char msg[] = "invalid rlp type to decode struct";
                IR_ABORT(msg, sizeof(msg) - 1);
                return NULL;
            }
            for (uint32_t i = 0; i < runtime_class->struct_fields_count; i++) {
                uint32_t field_offset = fields_offsets_array[i];
                struct IRRuntimeClass *field_type =
                        (struct IRRuntimeClass *) (all_runtimes_classes_address
                                                      + field_offset);
                void *field_ptr = result + offset;
                void* field_new_value = common_rlp_decode(stream, field_offset);
                size_t elem_in_struct_size = get_ir_type_size_as_element(field_type);
                if (is_pointer_type(field_offset)) {
                    *((void**)(field_ptr)) = field_new_value;
                } else {
                    memcpy(field_ptr, field_new_value, elem_in_struct_size);
                }
                
                offset += elem_in_struct_size;
            }
            return result;
        } break;
        case IR_RUNTIME_TYPE_ARRAY:
        {
            uint32_t elem_ty_offset = runtime_class->array_item_ty;
            IRRuntimeClass *elem_ty =
                (IRRuntimeClass *)(all_runtimes_classes_address
                                      + elem_ty_offset);

            size_t array_size = runtime_class->array_size;
            if (elem_ty->ty == IR_RUNTIME_TYPE_U8 || elem_ty->ty == IR_RUNTIME_TYPE_I8) {
                // fixed length bytes, treat like integers/bytes
                qvector_t *int_bytes = rlp_decode(stream);
                return int_bytes;
            }

            RlpValueType ty;
            size_t data_len = rlp_decode_length(stream, &ty); // bytes count, not elements count
            if (ty == RLP_LIST) {
                // data_len is bytes len, not list size
                size_t array_element_size = get_ir_type_size_as_element(elem_ty);
                qvector_t *result = qvector(data_len > 0 ? data_len : 1, array_element_size, QVECTOR_RESIZE_DOUBLE);
                size_t end_offset = stream->read_offset + data_len;
                size_t count = 0;
                while(stream->read_offset < end_offset) {
                    void *item = common_rlp_decode(stream, elem_ty_offset);
                    if (is_pointer_type(elem_ty_offset)) {
                        qvector_addlast(result, &item);
                    } else {
                        qvector_addlast(result, item);
                    }
                    count++;
                }
                result->num = count;
                return result;
            } else {
                char msg[] = "unknown rlp value type when decode";
                IR_ABORT(msg, sizeof(msg) - 1);
                return NULL;
            }
        } break;
        default:
        {
            char msg[] = "unknown ir runtime type in ir rlp type";
            IR_ABORT(msg, sizeof(msg) - 1);
            return NULL;
        }
    }
}

void *
ir_builtin_rlp_decode(uint32_t runtime_class_offset, qvector_t *val) {
    ByteStream *stream = create_byte_stream_from_ir_bytes(val);
    return common_rlp_decode(stream, runtime_class_offset);
}
