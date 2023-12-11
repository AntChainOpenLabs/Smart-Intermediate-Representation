// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./qstring.h"

#ifndef CC_LIB_TEST_MOCK
#define assert(x) (0)
#else
#include <assert.h>
#endif // CC_LIB_TEST_MOCK

#define BLOOM_MASK uint64_t

#define BLOOM_WIDTH 64

#define BLOOM(mask, ch) ((mask & (1ULL << ((ch) & (BLOOM_WIDTH - 1)))))

static inline BLOOM_MASK
make_bloom_mask(struct vector *v)
{

    /* calculate simple bloom-style bitmask for a given string */

    BLOOM_MASK mask;

    mask = 0;
    for (uint32_t i = 0; i < v->len; ++i) {
        char ch = v->data[i];
        mask |= (1ULL << ((ch) & (BLOOM_WIDTH - 1)));
    }
    return mask;
}
qvector_t *
vector_split(struct vector *v, struct vector *regex, struct RuntimeContext *ctx)
{
    qvector_t *res = qvector(16, sizeof(uint32_t *),
                             QVECTOR_RESIZE_DOUBLE | QVECTOR_THREADSAFE);
    uint32_t cur_begin = 0;
    if (regex->len == 0) {
        char msg[] = "ValueError: empty separator";
        runtime_abort(msg, sizeof(msg) - 1, ctx);
        return res;
    }
    while (cur_begin <= v->len) {
        int32_t found_idx = vector_find(v, regex, cur_begin, v->len);
        int32_t sub_str_len = found_idx - cur_begin;
        struct vector *sub_str =
            vector_new(sub_str_len, sizeof(uint8_t), v->data + cur_begin);
        uint32_t *sub_str_ptr = malloc(sizeof(uint32_t));
        *sub_str_ptr = (uint32_t)sub_str;
        qvector_addlast(res, sub_str_ptr);
        cur_begin = found_idx + regex->len;
    }
    return res;
}

qvector_t * 
vector_to_bytes(struct vector *v) {
    if (v->len > 100) {
        // large string to bytes
        qvector_t *res = qvector(v->len, sizeof(uint8_t), QVECTOR_RESIZE_DOUBLE | QVECTOR_THREADSAFE);
        memcpy(res->data, v->data, v->len);
        res->num = v->len;
        return res;
    }
    qvector_t *res = qvector(16, sizeof(uint8_t), QVECTOR_RESIZE_DOUBLE | QVECTOR_THREADSAFE);
    uint32_t index = 0;
    // TODO: too slow and need too many gas when big str params to bytes
    while (index < v->len ) {
        auto elem = v->data[index];
        qvector_addlast(res, &elem);
        index ++;
    }
    return res;
}

uint32_t
vector_count(struct vector *v, struct vector *sub, int32_t begin, int32_t end)
{
    assert(begin >= 0);
    if (end > v->len) {
        end = v->len;
    }
    int32_t cmp_len = end - begin;
    if (sub->len == 0) {
        return cmp_len == 0 ? 1 : cmp_len + 1;
    }
    uint32_t result = 0;
    int32_t cur_begin = begin;
    while (cur_begin < end) {
        int32_t found_idx = vector_find(v, sub, cur_begin, end);
        if (found_idx >= v->len) {
            break;
        }
        cur_begin = found_idx + sub->len;
        result++;
    }
    return result;
}

struct vector *
vector_lower(struct vector *v)
{
    struct vector *res = vector_new(v->len, sizeof(uint8_t), v->data);
    for (int i = 0; i < v->len; ++i) {
        if (v->data[i] <= 'Z' && v->data[i] >= 'A') {
            res->data[i] += 'a' - 'A';
        }
    }
    return res;
}

struct vector *
vector_upper(struct vector *v)
{
    struct vector *res = vector_new(v->len, sizeof(uint8_t), v->data);
    for (int i = 0; i < v->len; ++i) {
        if (v->data[i] <= 'z' && v->data[i] >= 'a') {
            res->data[i] += 'A' - 'a';
        }
    }
    return res;
}

extern bool
vector_startswith(struct vector *v, struct vector *prefix, int32_t begin,
                  int32_t end)
{
    assert(begin >= 0);
    if (begin > v->len) {
        return false;
    }

    if (end > v->len) {
        end = v->len;
    }
    if (end - begin < prefix->len) {
        return false;
    }

    return __memcmp(v->data + begin, prefix->len, prefix->data, prefix->len);
}

extern bool
vector_endswith(struct vector *v, struct vector *suffix, int32_t begin,
                int32_t end)
{
    assert(begin >= 0);
    if (end > v->len) {
        end = v->len;
    }
    if (end - begin < suffix->len) {
        return false;
    }
    return __memcmp(v->data + (end - suffix->len), suffix->len, suffix->data,
                    suffix->len);
}

bool
vector_isalnum(struct vector *v)
{
    if (v->len == 0) {
        return false;
    }
    int32_t i = 0;
    for (i = 0; i < v->len; ++i) {
        char ch = v->data[i];
        if (ch >= 'a' && ch <= 'z') {
            continue;
        }
        if (ch >= 'A' && ch <= 'Z') {
            continue;
        }
        if (ch >= '0' && ch <= '9') {
            continue;
        }
        return false;
    }
    return true;
}

bool
vector_isalpha(struct vector *v)
{
    if (v->len == 0) {
        return false;
    }
    int32_t i = 0;
    for (i = 0; i < v->len; ++i) {
        char ch = v->data[i];
        if (ch >= 'a' && ch <= 'z') {
            continue;
        }
        if (ch >= 'A' && ch <= 'Z') {
            continue;
        }
        return false;
    }
    return true;
}

bool
vector_isdigit(struct vector *v)
{
    if (v->len == 0) {
        return false;
    }
    int32_t i = 0;
    for (i = 0; i < v->len; ++i) {
        char ch = v->data[i];
        if (ch >= '0' && ch <= '9') {
            continue;
        }
        return false;
    }
    return true;
}

uint8_t
vector_at(struct vector *v, int32_t index)
{
    if (index < 0) {
        index += v->len;
    }

    assert(index < v->len);
    return v->data[index];
}

int32_t
vector_find(struct vector *v, struct vector *regex, int32_t begin, int32_t end)
{
    if (begin < 0) {
        begin = 0;
    }

    if (end > v->len) {
        end = v->len;
    }
    for (uint32_t i = begin; i + regex->len <= end; ++i) {

        if (__memcmp(v->data + i, regex->len, regex->data, regex->len)) {
            return i;
        }
    }
    return v->len;
}

int32_t
vector_find_char(struct vector *v, char *regex, int32_t begin, int32_t end)
{
    assert(begin >= 0);
    if (end > v->len) {
        end = v->len;
    }
    for (uint32_t i = begin; i < end; ++i) {
        if (v->data[i] == *regex) {
            return i;
        }
    }
    return end;
}

extern bool
vector_islower(struct vector *v)
{
    bool with_alpha = false;
    for (uint32_t i = 0; i < v->len; ++i) {
        char ch = v->data[i];
        if (ch >= 'a' && ch <= 'z') {
            with_alpha = true;
        }
        else if (ch >= 'A' && ch <= 'Z') {
            return false;
        }
    }
    return with_alpha;
}

extern bool
vector_isupper(struct vector *v)
{
    bool with_alpha = false;
    for (uint32_t i = 0; i < v->len; ++i) {
        char ch = v->data[i];
        if (ch >= 'A' && ch <= 'Z') {
            with_alpha = true;
        }
        else if (ch >= 'a' && ch <= 'z') {
            return false;
        }
    }
    return with_alpha;
}

extern bool
vector_isspace(struct vector *v)
{
    if (v->len == 0) {
        return false;
    }
    for (uint32_t i = 0; i < v->len; ++i) {
        char ch = v->data[i];
        // \t   \n   \v   \f   \r
        // \x09 \x0a \x0b \x0c \x0d
        if (ch >= '\t' && ch <= '\r') {
            continue;
        }
        if (ch == ' ') {
            continue;
        }
        return false;
    }
    return true;
}
struct vector *
vector_xstrip(struct vector *v, struct vector *chars, enum StripKind kind)
{
    BLOOM_MASK chars_mask;
    chars_mask = make_bloom_mask(chars);
    uint32_t i = 0;
    uint32_t chars_end = chars->len;
    if (kind != RightStrip) {
        while (i < v->len) {
            char ch = v->data[i];
            if (!BLOOM(chars_mask, ch)) {
                break;
            }
            if (vector_find_char(chars, &ch, 0, chars_end) == chars_end) {
                break;
            }
            ++i;
        }
    }
    uint32_t j = v->len;
    if (kind != LeftStrip) {
        --j;
        while (j >= i) {
            char ch = v->data[j];
            if (!BLOOM(chars_mask, ch)) {
                break;
            }
            if (vector_find_char(chars, &ch, 0, chars_end) == chars_end) {
                break;
            }
            --j;
        }
        ++j;
    }
    return vector_new(j - i, sizeof(char), v->data + i);
}

struct vector *
vector_strip(struct vector *v, struct vector *chars)
{
    return vector_xstrip(v, chars, BothStrip);
}

struct vector *
vector_lstrip(struct vector *v, struct vector *chars)
{
    return vector_xstrip(v, chars, LeftStrip);
}

struct vector *
vector_rstrip(struct vector *v, struct vector *chars)
{
    return vector_xstrip(v, chars, RightStrip);
}

void
vector_append(struct vector *left, struct vector *right,
              struct RuntimeContext *ctx)
{
    vector_appd(left, right->data, right->len, ctx);
}

void
vector_appd(struct vector *v, uint8_t *bytes, uint32_t len,
            struct RuntimeContext *ctx)
{
    if (len == 0) {
        return;
    }
    uint64_t extended_cap = v->cap ? v->cap : 1;

    const uint64_t max_u32 = 0xffffffff;
    while (extended_cap < v->len + len + 1) {

        if (extended_cap > max_u32) {
            char msg[] = "vector_appd error: vector length overflow (greater"
                         "than 0xffffffff)";
            runtime_abort(msg, sizeof(msg) - 1, ctx);
            return;
        }

        extended_cap *= 2;
    }

    if (extended_cap > v->cap) {
        uint8_t *temp_cell = malloc(extended_cap * sizeof(uint8_t));
        memcpy(temp_cell, v->data, v->len);
        memcpy(temp_cell + v->len, bytes, len);
        if (v->cap) {
            free(v->data);
        }
        v->data = temp_cell;
    }
    else {
        memcpy(v->data + v->len, bytes, len);
    }
    v->len += len;
    v->cap = extended_cap;

    // set last byte to 0
    v->data[v->len] = 0;
}

struct vector *
vector_join(struct vector *v, qvector_t *seq, struct RuntimeContext *ctx)
{
    struct vector *res = vector_new(0, sizeof(uint8_t), NULL);
    qvector_obj_t obj;
    __memset((void *)&obj, 0, sizeof(obj));
    if (qvector_getnext(seq, &obj, false)) {
        uint32_t elem_addr = *((uint32_t *)obj.data);
        struct vector *elem = (struct vector *)elem_addr;
        vector_appd(res, elem->data, elem->len, ctx);
    }
    else {
        return res;
    }
    while (qvector_getnext(seq, &obj, false)) {
        vector_appd(res, v->data, v->len, ctx);
        uint32_t elem_addr = *((uint32_t *)obj.data);
        struct vector *elem = (struct vector *)elem_addr;
        vector_appd(res, elem->data, elem->len, ctx);
    }
    return res;
}

struct vector *
vector_replace(struct vector *v, struct vector *old, struct vector *new,
               int32_t count, struct RuntimeContext *ctx)
{
    struct vector *res = vector_new(0, sizeof(uint8_t), NULL);
    int32_t replace_cnt = 0;
    // negative count means greater than i32 Maximum,so we just convert i32 to
    // u32
    uint32_t fixed_count = (uint32_t)count;
    uint32_t i = 0;
    if (old->len == 0) {
        for (i = 0; i < v->len && replace_cnt < fixed_count;
             ++i, ++replace_cnt) {
            vector_appd(res, new->data, new->len, ctx);
            vector_appd(res, v->data + i, 1, ctx);
        }
        if (replace_cnt < fixed_count) {
            vector_appd(res, new->data, new->len, ctx);
        }
        else {
            vector_appd(res, v->data + i, v->len - i, ctx);
        }
    }
    else {
        while (i <= v->len) {
            int32_t found_idx = vector_find(v, old, i, v->len);
            if (found_idx >= v->len || replace_cnt >= fixed_count) {
                vector_appd(res, v->data + i, v->len - i, ctx);
                break;
            }
            vector_appd(res, v->data + i, found_idx - i, ctx);
            vector_appd(res, new->data, new->len, ctx);
            i = found_idx + old->len;
            replace_cnt++;
        }
    }
    return res;
}

struct vector *
vector_substr(struct vector *v, int32_t begin, int32_t end)
{
    if (begin < 0) {
        begin = 0;
    }

    if (end > v->len) {
        end = v->len;
    }

    if (begin >= v->len) {
        return vector_new(0, sizeof(char), NULL);
    }

    return vector_new(end - begin, sizeof(char), v->data + begin);
}

struct vector *
vector_insert(struct vector *v, struct vector *sub, int32_t index,
              struct RuntimeContext *ctx)
{
    struct vector *res = vector_new(0, sizeof(uint8_t), NULL);
    if (sub->len == 0) {
        return v;
    }

    if (index == 0 || index < 0) {
        vector_appd(res, sub->data, sub->len, ctx);
        vector_appd(res, v->data, v->len, ctx);
    }
    else if (index >= v->len) {
        vector_appd(res, v->data, v->len, ctx);
        vector_appd(res, sub->data, sub->len, ctx);
    }
    else {
        vector_appd(res, v->data, index, ctx);
        vector_appd(res, sub->data, sub->len, ctx);
        vector_appd(res, v->data + index, v->len - index, ctx);
    }

    return res;
}