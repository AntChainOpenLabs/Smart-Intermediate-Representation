// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./chain.h"
#include "./data_stream_builtin.h"

// revert hostapi
extern void
revert(int32_t error_code, const char *error_msg, uint32_t error_msg_len);
// co_call hostapi
extern int32_t
co_call(const char *contract, uint32_t contract_length, const char *method,
        uint32_t method_length, const char *argpack, uint32_t argpack_length);

void
builtin_revert(int32_t err_code, struct vector *msg_str)
{
    qvector_t *encoded_msg_bytes =
        ir_builtin_data_stream_encode_str(msg_str);
    revert(err_code, encoded_msg_bytes->data, encoded_msg_bytes->num);
}

void
builtin_abort(const char *msg, int32_t msg_len)
{
    revert(3002, msg, msg_len);
}

int32_t
builtin_co_call(struct vector *contract_name, struct vector *method,
                struct vector *encoded_params)
{
    // TODO: optimize this
    // insert first zero before encoded_params because the first byte of params
    // is version
    struct vector *encoded_params_with_version =
        vector_new(encoded_params->len + 1, 1, NULL);
    ((uint8_t *)encoded_params_with_version->data)[0] = 0;
    memcpy(encoded_params_with_version->data + 1, encoded_params->data,
           encoded_params->len);
    return co_call(contract_name->data, contract_name->len, method->data,
                   method->len, encoded_params_with_version->data,
                   encoded_params_with_version->len);
}
