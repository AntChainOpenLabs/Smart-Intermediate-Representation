// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "test.h"
#include "../qvector.h"
#include "../stdlib.h"
#include "../rlp.h"

static struct vector *create_ir_str(const char *str) {
    return vector_new(strlen(str), 1, (uint8_t *) str);
}

TEST(rlp, rlp_spec_test) {
    // tests in https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/
    // and https://github.com/ethereum/tests/blob/develop/RLPTests/rlptest.json
    {
        // "dog"
        ByteStream *bs = new_byte_stream();
        rlp_encode_str(bs, create_ir_str("dog"));
        ASSERT_EQ(bs->len, 4);
        ASSERT_EQ((uint8_t) bs->data[0], 0x83);
        ASSERT_TRUE(0 == memcmp(bs->data + 1, "dog", 3));

        // decode
        qvector_t *decoded_str_bytes = rlp_decode(bs);
        ASSERT_EQ(decoded_str_bytes->num, 3);
        ASSERT_TRUE(0 == memcmp(decoded_str_bytes->data, "dog", 3));
    }
    {
        // ["cat", "dog"]
        ByteStream *bs = new_byte_stream();
        qvector_t *list = qvector(1, sizeof(void *), QVECTOR_RESIZE_DOUBLE);
        struct vector *cat = create_ir_str("cat");
        struct vector *dog = create_ir_str("dog");
        qvector_addlast(list, &cat);
        qvector_addlast(list, &dog);
        rlp_encode_str_list(bs, list);
        ASSERT_EQ(bs->len, 9);
        ASSERT_EQ((uint8_t) bs->data[0], 0xc8);
        ASSERT_EQ((uint8_t) bs->data[1], 0x83);
        ASSERT_TRUE(0 == memcmp(bs->data + 2, "cat", 3));
        ASSERT_EQ((uint8_t) bs->data[5], 0x83);
        ASSERT_TRUE(0 == memcmp(bs->data + 6, "dog", 3));

        // decode
        qvector_t *decoded_list = rlp_decode(bs);
        ASSERT_EQ(decoded_list->num, 2);
        qvector_t *item0 = *((qvector_t **) qvector_getat(decoded_list, 0, false, NULL));
        ASSERT_EQ(item0->num, 3);
        ASSERT_TRUE(0 == memcmp(item0->data, "cat", 3));

        qvector_t *item1 = *((qvector_t **) qvector_getat(decoded_list, 1, false, NULL));
        ASSERT_EQ(item1->num, 3);
        ASSERT_TRUE(0 == memcmp(item1->data, "dog", 3));
    }
    {
        // shortListMax1
        // [ "asdf", "qwer", "zxcv", "asdf","qwer", "zxcv", "asdf", "qwer", "zxcv", "asdf", "qwer"]
        ByteStream *bs = new_byte_stream();
        qvector_t *list = qvector(1, sizeof(void *), QVECTOR_RESIZE_DOUBLE);
        {
            struct vector *cat = create_ir_str("asdf");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("qwer");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("zxcv");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("asdf");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("qwer");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("zxcv");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("asdf");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("qwer");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("zxcv");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("asdf");
            qvector_addlast(list, &cat);
        }
        {
            struct vector *cat = create_ir_str("qwer");
            qvector_addlast(list, &cat);
        }
        rlp_encode_str_list(bs, list);
        ASSERT_EQ(bs->len, 1 + 5 * 11);
        ASSERT_EQ((uint8_t) bs->data[0], 0xf7);
        for (size_t i=0;i<4;i++) {
            for (size_t j=0;j<3;j++) {
                size_t index = i * 3 + j;
                if (index >= 11) {
                    continue; // the list size=11
                }
                ASSERT_EQ((uint8_t) bs->data[1 + 5 * index], 0x84);
                const char *str;
                switch (j) {
                    case 0: str = "asdf"; break;
                    case 1: str = "qwer"; break;
                    case 2: str = "zxcv"; break;
                    default: abort();
                }
                ASSERT_TRUE(0 == memcmp(bs->data + 1 + 5 * index + 1, str, 4));
            }
        }

        // decode
        qvector_t *decoded_list = rlp_decode(bs);
        ASSERT_EQ(decoded_list->num, 11);
        qvector_t *item0 = *((qvector_t **) qvector_getat(decoded_list, 0, false, NULL));
        ASSERT_EQ(item0->num, 4);
        ASSERT_TRUE(0 == memcmp(item0->data, "asdf", 3));

        qvector_t *item1 = *((qvector_t **) qvector_getat(decoded_list, 1, false, NULL));
        ASSERT_EQ(item1->num, 4);
        ASSERT_TRUE(0 == memcmp(item1->data, "qwer", 4));

        qvector_t *item2 = *((qvector_t **) qvector_getat(decoded_list, 2, false, NULL));
        ASSERT_EQ(item2->num, 4);
        ASSERT_TRUE(0 == memcmp(item2->data, "zxcv", 4));
    }
    {
        // emtpy string
        ByteStream *bs = new_byte_stream();
        rlp_encode_str(bs, create_ir_str(""));
        ASSERT_EQ(bs->len, 1);
        ASSERT_EQ((uint8_t) bs->data[0], 0x80);

        // decode
        qvector_t *decoded_str_bytes = rlp_decode(bs);
        ASSERT_EQ(decoded_str_bytes->num, 0);
        ASSERT_EQ(decoded_str_bytes->objsize, 1);
    }
    {
        // empty list
        ByteStream *bs = new_byte_stream();
        qvector_t *list = qvector(1, sizeof(void *), QVECTOR_RESIZE_DOUBLE);
        rlp_encode_str_list(bs, list);
        ASSERT_EQ(bs->len, 1);
        ASSERT_EQ((uint8_t) bs->data[0], 0xc0);

        // decode
        qvector_t *decoded_list = rlp_decode(bs);
        ASSERT_EQ(decoded_list->num, 0);
    }
    {
        // the encoded integer 0 ('\x00')
        ByteStream *bs = new_byte_stream();
        qvector_t *num_bytes = qvector(1, 1, QVECTOR_RESIZE_DOUBLE);
        int8_t num_raw_bytes[] = {0};
        qvector_addlast(num_bytes, &num_raw_bytes[0]);
        rlp_encode_bytes(bs, num_bytes);
        ASSERT_EQ(bs->len, 1);
        ASSERT_EQ((uint8_t) bs->data[0], 0x00);

        // decode
        qvector_t *decoded_list = rlp_decode(bs);
        ASSERT_EQ(decoded_list->num, 1);
        ASSERT_TRUE(0 == memcmp(decoded_list->data, num_raw_bytes, sizeof(num_raw_bytes)));
    }
    {
        // the encoded integer 15 ('\x0f')
        ByteStream *bs = new_byte_stream();
        qvector_t *num_bytes = qvector(1, 1, QVECTOR_RESIZE_DOUBLE);
        int8_t num_raw_bytes[] = {15};
        qvector_addlast(num_bytes, &num_raw_bytes[0]);
        rlp_encode_bytes(bs, num_bytes);
        ASSERT_EQ(bs->len, 1);
        ASSERT_EQ((uint8_t) bs->data[0], 0x0f);

        // decode
        qvector_t *decoded_list = rlp_decode(bs);
        ASSERT_EQ(decoded_list->num, 1);
        ASSERT_TRUE(0 == memcmp(decoded_list->data, num_raw_bytes, sizeof(num_raw_bytes)));
    }
    {
        // the encoded integer 0x7f
        ByteStream *bs = new_byte_stream();
        qvector_t *num_bytes = qvector(1, 1, QVECTOR_RESIZE_DOUBLE);
        int8_t num_raw_bytes[] = {0x7f};
        qvector_addlast(num_bytes, &num_raw_bytes[0]);
        rlp_encode_bytes(bs, num_bytes);
        ASSERT_EQ(bs->len, 1);
        ASSERT_EQ((uint8_t) bs->data[0], 0x7f);

        // decode
        qvector_t *decoded_list = rlp_decode(bs);
        ASSERT_EQ(decoded_list->num, 1);
        ASSERT_TRUE(0 == memcmp(decoded_list->data, num_raw_bytes, sizeof(num_raw_bytes)));
    }
    {
        // the encoded integer 1024 ('\x04\x00')
        ByteStream *bs = new_byte_stream();
        qvector_t *num_bytes = qvector(2, 1, QVECTOR_RESIZE_DOUBLE);
        int8_t num_raw_bytes[] = {0x04, 0x00};
        for (size_t i = 0; i < sizeof(num_raw_bytes); i++) {
            qvector_addlast(num_bytes, &num_raw_bytes[i]);
        }
        rlp_encode_bytes(bs, num_bytes);
        ASSERT_EQ(bs->len, 3);
        ASSERT_EQ((uint8_t) bs->data[0], 0x82);
        ASSERT_EQ((uint8_t) bs->data[1], 0x04);
        ASSERT_EQ((uint8_t) bs->data[2], 0x00);

        // decode
        qvector_t *decoded_list = rlp_decode(bs);
        ASSERT_EQ(decoded_list->num, 2);
        ASSERT_TRUE(0 == memcmp(decoded_list->data, num_raw_bytes, sizeof(num_raw_bytes)));
    }
    {
        // the set theoretical representation(opens in a new tab)↗ of three,
        // [ [], [[]], [ [], [[]] ] ] = [ 0xc7, 0xc0, 0xc1, 0xc0, 0xc3, 0xc0, 0xc1, 0xc0 ]
        // ir qvector_t not supported to get element type. so not support this case
    }
    {
        // the short string 2 "Lorem ipsum dolor sit amet, consectetur adipisicing eli"
        // = [ 0xb8, 0x38, 'L', 'o', 'r', 'e', 'm', ' ', ... , 'e', 'l', 'i', 't' ]
        ByteStream *bs = new_byte_stream();
        const char *str = "Lorem ipsum dolor sit amet, consectetur adipisicing eli";
        rlp_encode_str(bs, create_ir_str(str));
        ASSERT_EQ(bs->len, 1 + strlen(str));
        ASSERT_EQ((uint8_t) bs->data[0], 0xb7);
        ASSERT_TRUE(0 == memcmp(bs->data + 1, str, strlen(str)));

        // decode
        qvector_t *decoded_str = rlp_decode(bs);
        ASSERT_EQ(decoded_str->num, strlen(str));
        ASSERT_TRUE(0 == memcmp(decoded_str->data, str, strlen(str)));
    }
    {
        // the string "Lorem ipsum dolor sit amet, consectetur adipisicing elit"
        // = [ 0xb8, 0x38, 'L', 'o', 'r', 'e', 'm', ' ', ... , 'e', 'l', 'i', 't' ]
        ByteStream *bs = new_byte_stream();
        const char *str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
        rlp_encode_str(bs, create_ir_str(str));
        ASSERT_EQ(strlen(str), 56);
        ASSERT_EQ(bs->len, 58);
        ASSERT_EQ((uint8_t) bs->data[0], 0xb8);
        ASSERT_EQ((uint8_t) bs->data[1], 0x38);
        ASSERT_TRUE(0 == memcmp(bs->data + 2, str, strlen(str)));

        // decode
        qvector_t *decoded_str = rlp_decode(bs);
        ASSERT_EQ(decoded_str->num, strlen(str));
        ASSERT_TRUE(0 == memcmp(decoded_str->data, str, strlen(str)));
    }
    {
        // the long string "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur mauris magna, suscipit sed vehicula non, iaculis faucibus tortor. Proin suscipit ultricies malesuada. Duis tortor elit, dictum quis tristique eu, ultrices at risus. Morbi a est imperdiet mi ullamcorper aliquet suscipit nec lorem. Aenean quis leo mollis, vulputate elit varius, consequat enim. Nulla ultrices turpis justo, et posuere urna consectetur nec. Proin non convallis metus. Donec tempor ipsum in mauris congue sollicitudin. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Suspendisse convallis sem vel massa faucibus, eget lacinia lacus tempor. Nulla quis ultricies purus. Proin auctor rhoncus nibh condimentum mollis. Aliquam consequat enim at metus luctus, a eleifend purus egestas. Curabitur at nibh metus. Nam bibendum, neque at auctor tristique, lorem libero aliquet arcu, non interdum tellus lectus sit amet eros. Cras rhoncus, metus ac ornare cursus, dolor justo ultrices metus, at ullamcorper volutpat"
        // = [ 0xb8, 0x38, 'L', 'o', 'r', 'e', 'm', ' ', ... , 'e', 'l', 'i', 't' ]
        ByteStream *bs = new_byte_stream();
        const char *str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur mauris magna, suscipit sed vehicula non, iaculis faucibus tortor. Proin suscipit ultricies malesuada. Duis tortor elit, dictum quis tristique eu, ultrices at risus. Morbi a est imperdiet mi ullamcorper aliquet suscipit nec lorem. Aenean quis leo mollis, vulputate elit varius, consequat enim. Nulla ultrices turpis justo, et posuere urna consectetur nec. Proin non convallis metus. Donec tempor ipsum in mauris congue sollicitudin. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Suspendisse convallis sem vel massa faucibus, eget lacinia lacus tempor. Nulla quis ultricies purus. Proin auctor rhoncus nibh condimentum mollis. Aliquam consequat enim at metus luctus, a eleifend purus egestas. Curabitur at nibh metus. Nam bibendum, neque at auctor tristique, lorem libero aliquet arcu, non interdum tellus lectus sit amet eros. Cras rhoncus, metus ac ornare cursus, dolor justo ultrices metus, at ullamcorper volutpat";
        rlp_encode_str(bs, create_ir_str(str));
        ASSERT_EQ(bs->len, 3 + strlen(str));
        ASSERT_EQ((uint8_t) bs->data[0], 0xb9);
        ASSERT_EQ((uint8_t) bs->data[1], 0x04);
        ASSERT_EQ((uint8_t) bs->data[2], 0x00);
        ASSERT_TRUE(0 == memcmp(bs->data + 3, str, strlen(str)));

        // decode
        qvector_t *decoded_str = rlp_decode(bs);
        ASSERT_EQ(decoded_str->num, strlen(str));
        ASSERT_TRUE(0 == memcmp(decoded_str->data, str, strlen(str)));
    }
}
