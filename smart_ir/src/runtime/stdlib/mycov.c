// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./mycov.h"
#include "./call_log.h"
#include "./data_stream_builtin.h"
#include "./math.h"

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

// maximum bb id support, avoid counters overflow
const uint32_t MAX_BB_ID_SUPPORT = 102400;
// bbId => countOfBbId(uint32). bbId is index
static qvector_t *global_counters = NULL;

static qvector_t *
get_singleton_counters()
{
    if (NULL == global_counters) {
        // qvector max can't be zero
        global_counters = qvector(1, sizeof(uint32_t), 2 /* double mode */);
    }
    return global_counters;
}

static void
append_str_to_qvector(qvector_t *buf, const char *str)
{
    for (uint32_t i = 0; i < __strlen(str); i++) {
        // must get the address, save the value of pointer
        // but qvector_addlast arg should save the pointer of value
        qvector_addlast(buf, &str[i]);
    }
}

const char *MYGCNA_VERSION = "0.1.0";

// dump counters array to mygcna file
static qvector_t *
dump_counters_to_mygcna(struct RuntimeContext *ctx)
{
    // qvector max can't be zero
    qvector_t *buf = qvector(1, sizeof(int8_t), 2 /* double mode */);
    append_str_to_qvector(buf, "{\"version\":\"");
    append_str_to_qvector(buf, MYGCNA_VERSION);
    append_str_to_qvector(buf, "\",\"counters\":{");

    // add "bb_id":count [,]
    qvector_t *counters = get_singleton_counters();
    println((int32_t) "mycoverage counters size", 24);
    char *counters_str = builtin_i32_toa(qvector_size(counters), 10);
    println((int32_t)counters_str, __strlen(counters_str));

    bool found_first_used_bb = false;
    for (uint32_t i = 0; i < qvector_size(counters); i++) {
        uint32_t count = *((uint32_t *)qvector_getat(counters, i, false, ctx));
        if (count == 0) {
            continue;
        }
        if (found_first_used_bb) {
            append_str_to_qvector(buf, ",");
        }
        found_first_used_bb = true;
        append_str_to_qvector(buf, "\"");
        // itoa
        uint32_t bb_id = i;
        char *bb_id_str = builtin_i32_toa(bb_id, 10);
        append_str_to_qvector(buf, (const char *)bb_id_str);
        append_str_to_qvector(buf, "\":");
        // itoa
        char *bb_count_str = builtin_i32_toa(count, 10);
        append_str_to_qvector(buf, (const char *)bb_count_str);
    }

    append_str_to_qvector(buf, "}}");
    println((int32_t) "mycoverage mygcna generated", 27);
    return buf;
}

void
ir_builtin_add_coverage_counter(int32_t bb_id)
{
    if (bb_id < 0) {
        char msg[] = "invalid cov bb id(< 0)";
        IR_ABORT(msg, sizeof(msg) - 1);
        return;
    }
    struct RuntimeContext ctx = { NULL, 0, 0 };

    qvector_t *counters = get_singleton_counters();

    if (bb_id >= counters->max) {
        uint32_t old_max = counters->max;
        if (!qvector_resize(counters, bb_id > 0 ? (bb_id * 2) : 1)) {
            char msg[] = "cov bb vector resize failed";
            IR_ABORT(msg, sizeof(msg) - 1);
            return;
        }
        __memset(counters->data + old_max * sizeof(uint32_t), 0x0,
                 (counters->max - old_max) * sizeof(uint32_t));
    }
    if (bb_id >= counters->num) {
        // make sure the counters size not exceed when get and set by index
        counters->num = bb_id + 1;
    }
    // 存取的是值的内存地址
    uint32_t old_count =
        *((uint32_t *)qvector_getat(counters, bb_id, false, &ctx));
    uint32_t new_count = old_count + 1;
    qvector_setat(counters, bb_id, &new_count, &ctx);
}

qvector_t *
ir_builtin_get_coverage_counters(struct RuntimeContext *ctx)
{
    return get_singleton_counters();
}

// // call.this_contract hostapi
// extern int32_t get_call_contract_length();
// extern void get_call_contract(uint8_t* data);

static bool
is_same_contract(uint8_t *a_bytes, int32_t a_len, uint8_t *b_bytes,
                 int32_t b_len)
{
    if (a_len != b_len || a_len < 0 || b_len < 0) {
        return false;
    }
    for (int i = 0; i < a_len; i++) {
        if (a_bytes[i] != b_bytes[i]) {
            return false;
        }
    }
    return true;
}

void
ir_builtin_call_coverage_log(struct RuntimeContext *ctx, int32_t success)
{
    // while contracts call contracts, each time of exiting the contract call stack should record a coverage
    // log, and record the address.because the losing of linear memory when contract return
    const char *event = "MyCoverage";
    struct vector *event_name =
        vector_new(__strlen(event), 1, (uint8_t *)event);
    qvector_t *event_name_topic =
        ir_builtin_data_stream_encode_str(event_name);

    qvector_t *topics = qvector(1, sizeof(qvector_t *), 2 /** double mode */);
    qvector_addlast(topics,
                    &event_name_topic); // must get the address, save the value of pointer
                                        // but qvector_addlast arg should save the pointer of value

    // desc. xxx.mygcna file data bytes
    qvector_t *desc_data = dump_counters_to_mygcna(ctx);
    qvector_t *desc = ir_builtin_data_stream_encode_i8array(desc_data);
    // call_log
    ir_builtin_call_log(topics, desc);
}
