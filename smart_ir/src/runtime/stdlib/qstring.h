// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef STRING_H
#define STRING_H

#include "./stdlib.h"
#include "./qvector.h"
#include "./stdlib.h"

enum StripKind {
    LeftStrip,
    RightStrip,
    BothStrip,
};

#ifdef __cplusplus
extern "C" {
#endif

int32_t
vector_find(struct vector *v, struct vector *regex, int32_t begin, int32_t end);

int32_t
vector_find_char(struct vector *v, char *regex, int32_t begin, int32_t end);

void
vector_append(struct vector *left, struct vector *right,
              struct RuntimeContext *ctx);
void
vector_appd(struct vector *v, uint8_t *bytes, uint32_t len,
            struct RuntimeContext *ctx);

struct vector *
vector_join(struct vector *v, qvector_t *seq, struct RuntimeContext *ctx);

struct vector *
vector_replace(struct vector *v, struct vector *old, struct vector *new_data,
               int32_t count, struct RuntimeContext *ctx);

struct vector *
vector_xstrip(struct vector *v, struct vector *chars, enum StripKind kind);

struct vector *
vector_strip(struct vector *v, struct vector *chars);

struct vector *
vector_lstrip(struct vector *v, struct vector *chars);

struct vector *
vector_rstrip(struct vector *v, struct vector *chars);

qvector_t *
vector_split(struct vector *v, struct vector *regex,
             struct RuntimeContext *ctx);

uint32_t
vector_count(struct vector *v, struct vector *sub, int32_t begin, int32_t end);

struct vector *
vector_lower(struct vector *v);

struct vector *
vector_upper(struct vector *v);

bool
vector_startswith(struct vector *v, struct vector *prefix, int32_t begin,
                  int32_t end);

bool
vector_endswith(struct vector *v, struct vector *suffix, int32_t begin,
                int32_t end);

bool
vector_isalnum(struct vector *v);

bool
vector_isalpha(struct vector *v);

bool
vector_isdigit(struct vector *v);

bool
vector_islower(struct vector *v);

bool
vector_isupper(struct vector *v);

bool
vector_isspace(struct vector *v);

uint8_t
vector_at(struct vector *v, int32_t index);

struct vector *
vector_substr(struct vector *v, int32_t begin, int32_t end);

struct vector *
vector_insert(struct vector *v, struct vector *sub, int32_t index,
              struct RuntimeContext *ctx);
qvector_t * 
vector_to_bytes(struct vector *v);

#ifdef __cplusplus
} // end extern "C"
#endif


#endif