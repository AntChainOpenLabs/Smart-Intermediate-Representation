// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#pragma once
#include <cstdint>
#include <cstdlib>

#pragma once
#include <cstdint>
#include <cstdlib>
// mock hostapis used in ir c libs
// TODO: mock implementation

extern "C" void
set_call_result(int32_t arg0, int32_t arg1)
{}

extern "C" void
write_object(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
             int32_t arg4, int32_t arg5, int32_t arg6, int32_t arg7)
{}

extern "C" void
read_object(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
            int32_t arg4, int32_t arg5)
{}

extern "C" void
delete_object(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
              int32_t arg4, int32_t arg5)
{}

extern "C" int32_t
read_object_length(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
                   int32_t arg4, int32_t arg5)
{
    return 0;
}

extern "C" void
get_call_sender(int32_t arg0)
{}

extern "C" int32_t
get_call_sender_length()
{
    return 0;
}

extern "C" void
get_call_contract(int32_t arg0)
{}

extern "C" int32_t
get_call_contract_length()
{
    return 0;
}

extern "C" void
get_op_contract(int32_t arg0)
{}

extern "C" int32_t
get_op_contract_length()
{
    return 0;
}

extern "C" void
get_call_argpack(int32_t arg0)
{}

extern "C" int32_t
get_call_argpack_length()
{
    return 0;
}

extern "C" int64_t
get_block_number()
{
    return 0;
}

extern "C" int64_t
get_block_timestamp()
{
    return 0;
}

extern "C" int64_t
get_tx_timestamp()
{
    return 0;
}

extern "C" int64_t
get_tx_nonce()
{
    return 0;
}

extern "C" int32_t
get_tx_index()
{
    return 0;
}

extern "C" void
get_tx_hash(int32_t arg0)
{}

extern "C" int32_t
get_tx_hash_length()
{
    return 0;
}

extern "C" void
println(int32_t arg0, int32_t arg1)
{}

extern "C" void
log(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3, int32_t arg4)
{}

extern "C" void
issue_asset(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
            int32_t arg4, int32_t arg5, int32_t arg6, int32_t arg7,
            int32_t arg8, int32_t arg9)
{}

extern "C" void
burn_asset(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3, int32_t arg4,
           int32_t arg5, int32_t arg6)
{}

extern "C" void
destroy_asset(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
              int32_t arg4, int32_t arg5, int32_t arg6)
{}

extern "C" void
get_asset_data(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
               int32_t arg4, int32_t arg5, int32_t arg6, int32_t arg7)
{}

extern "C" int32_t
get_asset_data_length(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
                      int32_t arg4, int32_t arg5, int32_t arg6)
{
    return 0;
}

extern "C" void
set_asset_data(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
               int32_t arg4, int32_t arg5, int32_t arg6, int32_t arg7,
               int32_t arg8)
{}

extern "C" void
get_fungible_asset_balance(int32_t arg0, int32_t arg1, int32_t arg2,
                           int32_t arg3, int32_t arg4, int32_t arg5,
                           int32_t arg6)
{}

extern "C" void
get_fungible_asset_tag(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
                       int32_t arg4, int32_t arg5, int32_t arg6)
{}

extern "C" int32_t
get_fungible_asset_tag_lrngth(int32_t arg0, int32_t arg1, int32_t arg2,
                              int32_t arg3, int32_t arg4, int32_t arg5)
{
    // TODO
    return 0;
}

extern "C" void
transfer_asset(int32_t arg0, int32_t arg1, int32_t arg2, int32_t arg3,
               int32_t arg4, int32_t arg5, int32_t arg6, int32_t arg7,
               int32_t arg8, int32_t arg9, int32_t arg10, int32_t arg11,
               int32_t arg12, int32_t arg13)
{
    // TODO
}

extern "C" void
sha256(uint32_t msg, uint32_t len, uint32_t value)
{
    // TODO
}
extern "C" void
sm3(uint32_t msg, uint32_t len, uint32_t value)
{
    // TODO
}

extern "C" void
keccak256(uint32_t msg, uint32_t len, uint32_t value)
{
    // TODO
}

extern "C" unsigned long
__builtin_wasm_memory_grow(int a, unsigned long pages)
{
    // TODO
    return 0;
}
extern "C" uint32_t
get_fungible_asset_tag_length(uint32_t immut_comps,
                              uint32_t immut_comps_d1_length,
                              uint32_t immut_comps_d2_length,
                              uint32_t mut_comps, uint32_t mut_comps_d1_length,
                              uint32_t mut_comps_d2_length)
{
    // TODO
    return 0;
}