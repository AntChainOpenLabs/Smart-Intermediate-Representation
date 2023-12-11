// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./stream.h"

ByteStream *new_byte_stream() {
    ByteStream *s = (ByteStream *) __malloc(sizeof(ByteStream));
    __memset(s, 0x0, sizeof(ByteStream));
    s->cap = 8;
    int8_t *buf = (int8_t *) __malloc(s->cap);
    if (!buf) {
        char msg[] = "malloc for bytestream failed";
        IR_ABORT(msg, sizeof(msg) - 1);
        return NULL;
    }
    s->data = buf;
    return s;
}

ByteStream *create_byte_stream_from_ir_bytes(qvector_t *ir_bytes) {
    if (ir_bytes->num < 1) {
        return new_byte_stream();
    }
    if (ir_bytes->objsize != 1) {
        char msg[] = "only can create byte stream from str or bytes";
        IR_ABORT(msg, sizeof(msg) - 1);
        return NULL;
    }
    ByteStream *s = (ByteStream *) __malloc(sizeof(ByteStream));
    // no need to reset zeros. because have len
    s->cap = ir_bytes->num;

    int8_t *buf = (int8_t *) __malloc(s->cap);
    if (!buf) {
        char msg[] = "malloc for bytestream failed";
        IR_ABORT(msg, sizeof(msg) - 1);
        return NULL;
    }
    memcpy(buf, ir_bytes->data, ir_bytes->num);
    s->len = ir_bytes->num;
    s->data = buf;
    return s;
}

static void byte_stream_grow(ByteStream *stream, size_t min_cap) {
    size_t new_cap = stream->cap * 2;
    if (new_cap < min_cap) {
        new_cap = min_cap;
    }
    if (new_cap < 0) {
        new_cap = 1;
    }
    int8_t *buf = (int8_t *) __malloc(new_cap);
    if (!buf) {
        char msg[] = "malloc for bytestream failed";
        IR_ABORT(msg, sizeof(msg) - 1);
        return;
    }
    if (stream->len > 0) {
        memcpy(buf, stream->data, stream->len);
    }
    free(stream->data);
    stream->data = buf;
    stream->cap = new_cap;
}

static void byte_stream_write_bytes_impl(ByteStream *stream, int8_t *data, size_t len, size_t try_count) {
    const size_t add_bytes_count = len;
    if ((stream->len + add_bytes_count) <= stream->cap) {
        memcpy(stream->data + stream->len, data, len);
        stream->len += add_bytes_count;
        return;
    }
    if (try_count > 10) {
        char msg[] = "try too many times to grow stream cap";
        IR_ABORT(msg, sizeof(msg) - 1);
        return;
    }

    byte_stream_grow(stream, add_bytes_count + stream->len);

    // re-try
    byte_stream_write_bytes_impl(stream, data, len, try_count+1);
}

void byte_stream_write_bytes(ByteStream *stream, int8_t *data, size_t len) {
    if (len == 0) {
        return;
    }
    byte_stream_write_bytes_impl(stream, data, len, 0);
}

void byte_stream_write_byte(ByteStream *stream, int8_t value) {
    const size_t add_bytes_count = 1;
    if ((stream->len + add_bytes_count) <= stream->cap) {
        stream->data[stream->len] = value;
        stream->len += add_bytes_count;
        return;
    }
    byte_stream_grow(stream, add_bytes_count + stream->len);
    // re-try
    byte_stream_write_byte(stream, value);
}

qvector_t *byte_stream_read_bytes_but_not_consume(ByteStream *stream, size_t offset, size_t count) {
    size_t remaining_len = byte_stream_read_remaining_length(stream);
    if (remaining_len < (offset + count) || count == 0) {
        char msg[] = "stream not enough";
        IR_ABORT(msg, sizeof(msg) - 1);
        return NULL;
    }
    qvector_t *result = qvector(count, 1, QVECTOR_RESIZE_DOUBLE);
    memcpy(result->data, stream->data + stream->read_offset + offset, count);
    result->num = count;
    return result;
}

qvector_t *
byte_stream_to_bytes(ByteStream *stream) {
    qvector_t *result = qvector(stream->len > 0 ? stream->len : 1, 1, QVECTOR_RESIZE_DOUBLE);
    if (stream->len == 0) {
        return result;
    }
    memcpy(result->data, stream->data, stream->len);
    result->num = stream->len;
    return result;
}
