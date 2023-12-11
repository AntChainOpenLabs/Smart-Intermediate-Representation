// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#ifndef __STREAM_H
#define __STREAM_H

#include "./stdlib.h"
#include "./qvector.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ByteStream {
    size_t read_offset;
    size_t len;
    size_t cap;
    int8_t *data;
} ByteStream;

ByteStream *new_byte_stream();

inline void __attribute__((artificial)) __attribute__((always_inline))
free_byte_stream(ByteStream *stream) {
    free(stream->data);
    free(stream);
}

ByteStream *create_byte_stream_from_ir_bytes(qvector_t *ir_bytes);

void byte_stream_write_bytes(ByteStream *stream, int8_t *data, size_t len);

void byte_stream_write_byte(ByteStream *stream, int8_t value);

inline size_t __attribute__((artificial)) __attribute__((always_inline))
byte_stream_read_remaining_length(ByteStream *stream) {
    if (stream->read_offset >= stream->len) {
        return 0;
    }
    return stream->len - stream->read_offset;
}

qvector_t *byte_stream_read_bytes_but_not_consume(ByteStream *stream, size_t offset, size_t count);

inline void __attribute__((artificial)) __attribute__((always_inline))
byte_stream_free_read_bytes(qvector_t *bs) {
    qvector_free(bs);
}

extern qvector_t *
byte_stream_to_bytes(ByteStream *stream);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // __STREAM_H
