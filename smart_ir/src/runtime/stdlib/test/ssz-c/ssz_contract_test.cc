// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./ssz.hh"
#include "./test.h"

#include <stdlib.h>

struct str_t {
    uint32_t offset;
};
struct UninitializedKV_t {
    str_t key;
    str_t value;
};
struct UninitializedKV_list_t {
    uint32_t offset;
};

#pragma pack(push, 1)
struct Contract_t {
    uint16_t version;
    str_t owner;
    str_t address;

    str_t ref_artifact;
    UninitializedKV_list_t uninitialized_kvs;
    uint64_t balance;
};
#pragma pack(pop)

/*
struct UninitializedKV {
    key: str
    value: str
}
struct Contract {
    version: u16
    owner: str
    address: str
    ref_artifact: str
    uninitialized_kvs: [UninitializedKV]
    balance: u64
}

Contract contract{
    0,
    ConstructVariableList("alice"),
    ConstructVariableList("bob"),
    ConstructVariableList("artifact"),
    {}
};

// contract:
00001a0000001f000000220000002a0000000000000000000000616c696365626f626172746966616374
//
// [0x00] 0000     // version: u16
// [0x02] 1a000000 // owner: str => offset: u32
// [0x06] 1f000000 // address: str => offset: u32
// [0x0a] 22000000 // ref_artifact: str
// [0x0e] 2a000000 // uninitialized_kvs: [UninitializedKV]
// [0x12] 00000000 // balance: u64
// [0x16] 00000000 //
// ------
// [0x1a] 616c696365       // owner => alice
// [0x1f] 626f62           // address => bob
// [0x22] 6172746966616374 // artifact
*/

TEST(szz, contract)
{
    const uint32_t kContractSize = 0x1a;
    ASSERT_EQ(sizeof(Contract_t),
              kContractSize); // == owner: str => offset: u32

    Contract_t x;
    memset(&x, 0, sizeof(x));

    std::string x_owner = "alice";
    std::string x_address = "bob";
    std::string x_ref_artifact = "artifact";

    uint32_t offset = sizeof(Contract_t);
    std::string hdr;
    std::string data;

    // encode struct
    {
        // version: u16
        hdr += ssz::encode_u16_hex(x.version);

        // owner: str
        hdr += ssz::encode_offset_hex(offset);
        data += ssz::encode_str_data_hex(x_owner);
        offset += x_owner.size();

        // address: str
        hdr += ssz::encode_offset_hex(offset);
        data += ssz::encode_str_data_hex(x_address);
        offset += x_address.size();

        // ref_artifact: str
        hdr += ssz::encode_offset_hex(offset);
        data += ssz::encode_str_data_hex(x_ref_artifact);
        offset += x_ref_artifact.size();

        // uninitialized_kvs: [UninitializedKV]
        hdr += ssz::encode_offset_hex(offset);

        // balance: u64
        hdr += ssz::encode_u64_hex(x.balance);
    }

    std::string got = hdr + data;
    std::string expect = "00001a0000001f000000220000002a00000000000000000000006"
                         "16c696365626f626172746966616374";

    ASSERT_STREQ(got.c_str(), expect.c_str());
}
