// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./ssz.hh"
#include "./test.h"

#include <stdlib.h>

struct str_t {
    uint32_t offset;
};

#pragma pack(push, 1)
struct Account_t {
    uint16_t version;
    str_t    owner;
    str_t    address;
    uint8_t  status;
    uint8_t  role;
    uint64_t privilege_bitmap;
    str_t    access_pk;
    str_t    recovery_pk;
    uint64_t last_recovery_block_number;
    uint8_t  control_flag;
    uint64_t balance;
};
#pragma pack(pop)

/*
struct AccountInfo {
    version: u16
    owner: str
    address: str
    status: u8
    role: u8
    privilege_bitmap: u64
    access_pk: str
    recovery_pk: str
    last_recovery_block_number: u64
    control_flag: u8
    balance: u64
}

Account acct1{
    version,   // version: u16
    {},        // owner: str
    {1, 2, 3}, // address: str
    0,         // status: u8
    2,         // role: u8
    0,         // privilege_bitmap: u64
    {},        // access_pk: str
    {},        // recovery_pk: str
    0,         // last_recovery_block_number: u64
    0,         // control_flag: u8
    0,         // balance: u64
};

// account 00002d0000002d0000000002000000000000000030000000300000000000000000000000000000000000000000010203
// [0x00] 0000      // version: u16
// [0x02] 2d000000  // owner: str => offset: u32
// [0x06] 2d000000  // address: str => offset: u32
// [0x0a] 00        // status: u8
// [0x0b] 02        // role: u8
// [0x0c] 00000000  // privilege_bitmap: u64
// [0x10] 00000000  //
// [0x14] 30000000  // access_pk: str => offset: u32
// [0x18] 30000000  // recovery_pk: str => offset: u32
// [0x1c] 00000000  // last_recovery_block_number: u64
// [0x20] 00000000  //
// [0x21] 00        // control_flag: u8
// [0x25] 00000000  // balance: u64
// [0x29] 00000000
// ------
// [0x2d] 010203    // address: str = {1, 2, 3}
*/

TEST(szz, account) {
    const uint32_t kAccountSize = 0x2d;
    ASSERT_EQ(sizeof(Account_t), kAccountSize); // == owner: str => offset: u32

    Account_t x;
    memset(&x, 0, sizeof(x));
    x.role = 2;

    std::string x_owner = "";
    std::string x_address = "\x01\x02\x03";
    std::string x_access_pk = "";
    std::string x_recovery_pk = "";

    uint32_t offset = sizeof(Account_t);
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

        // role: u8
        hdr += ssz::encode_u8_hex(x.role);

        // privilege_bitmap: u64
        hdr += ssz::encode_u64_hex(x.privilege_bitmap);

        // access_pk: str
        hdr += ssz::encode_offset_hex(offset);
        data += ssz::encode_str_data_hex(x_access_pk);
        offset += x_access_pk.size();

        // recovery_pk: str
        hdr += ssz::encode_offset_hex(offset);
        data += ssz::encode_str_data_hex(x_recovery_pk);
        offset += x_recovery_pk.size();

        // last_recovery_block_number: u64
        hdr += ssz::encode_u64_hex(x.privilege_bitmap);

        // control_flag: u8
        hdr += ssz::encode_u8_hex(x.control_flag);

        // balance: u64
        hdr += ssz::encode_u64_hex(x.balance);
    }

    ASSERT_STREQ((hdr + data).c_str(), "00002d0000002d0000000002000000000000000030000000300000000000000000000000000000000000000000010203");
}
