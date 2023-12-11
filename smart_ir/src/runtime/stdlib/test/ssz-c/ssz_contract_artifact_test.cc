// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./ssz.hh"
#include "./test.h"

#include <stdlib.h>

struct str_t {
    uint32_t offset;
};
struct u64_list_t {
    uint32_t offset;
};

#pragma pack(push, 1)
struct Artifact_t {
    uint16_t version;
    str_t owner;
    str_t address;
    uint8_t status;
    uint8_t abi_hash[32];
    str_t abi;
    uint16_t attribute;
    u64_list_t update_log;
};
#pragma pack(pop)

/*
struct Artifact {
    version: u16
    owner: str
    address: str
    status: u8

    abi_hash: [u8; 32]
    abi: str
    attribute: u16
    update_log: [u64]
}
Artifact artifact{
    0,
    ConstructVariableList("alice"),
    ConstructVariableList("bob"),
    0,
    ConstructFixedList<32>("abi_hash"),
    abi,
    0,
    {uint64_t(0)},
};

// artifact
0000350000003a000000006162695f686173680000000000000000000000000000000000000000000000003d00000000005b000000616c696365626f6200001e0000001e0000001e0000001e0000001e0000001e0000001e0000000000000000000000

// [0x00] 0000     // version: u16
// [0x02] 35000000 // owner: str => offset: u32
// [0x06] 3a000000 // address: str => offset: u32
// [0x0a] 00       // status: u8
// [0x0b] 6162695f // abi_hash: [u8; 32]
// [0x..] 68617368 // ...
// [0x..] 00000000 // ...
// [0x..] 00000000 // ...
// [0x..] 00000000 // ...
// [0x..] 00000000 // ...
// [0x..] 00000000 // ...
// [0x..] 00000000 // abi_hash: [u8; 32]
// [0x2b] 3d000000 // abi: str => offset: u32
// [0x2f] 0000     // attribute: u16
// [0x31] 5b000000 // update_log: [u64] => offset: u32
// ------
// [0x35] 616c696365 // owner => alice
// [0x3a] 626f62     // address => bob
// [0x3d] 00001e0000001e0000001e0000001e0000001e0000001e0000001e000000
// [0x5b] 0000000000000000
*/

TEST(szz, artifact)
{
    const uint32_t kArtifactSize = 0x35;
    ASSERT_EQ(sizeof(Artifact_t),
              kArtifactSize); // == owner: str => offset: u32

    Artifact_t x;
    memset(&x, 0, sizeof(x));

    std::string x_owner = "alice";
    std::string x_address = "bob";

    // [0, 0, 30, 0, 0, 0, 30, 0, 0, 0, 30, 0, 0, 0, 30, 0, 0, 0, 30, 0, 0, 0,
    // 30, 0, 0, 0, 30, 0, 0, 0]
    std::string x_abi = ssz::from_hex(
        "00001e0000001e0000001e0000001e0000001e0000001e0000001e000000");

    std::string x_update_log = ssz::from_hex("0000000000000000");

    std::string abi_hash_hex;
    abi_hash_hex += "6162695f";
    abi_hash_hex += "68617368";
    abi_hash_hex += "00000000";
    abi_hash_hex += "00000000";
    abi_hash_hex += "00000000";
    abi_hash_hex += "00000000";
    abi_hash_hex += "00000000";
    abi_hash_hex += "00000000";

    uint32_t offset = sizeof(Artifact_t);
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

        // status: u8
        hdr += ssz::encode_u8_hex(x.status);

        // abi_hash: [u8; 32]
        hdr += abi_hash_hex;

        // abi: str
        hdr += ssz::encode_offset_hex(offset);
        data += ssz::encode_str_data_hex(x_abi);
        offset += x_abi.size();

        // attribute: u16
        hdr += ssz::encode_u16_hex(x.attribute);

        // update_log: [u64]
        hdr += ssz::encode_offset_hex(offset);
        data += ssz::encode_str_data_hex(x_update_log);
        offset += x_update_log.size();
    }

    std::string got = hdr + data;
    std::string expect =
        "0000350000003a000000006162695f6861736800000000000000000000000000000000"
        "00000000000000003d00000000005b000000616c696365626f6200001e0000001e0000"
        "001e0000001e0000001e0000001e0000001e0000000000000000000000";

    ASSERT_STREQ(got.c_str(), expect.c_str());
}
