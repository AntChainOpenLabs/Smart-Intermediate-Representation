// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "test.h"
#include "../ir_type.h"
#include "../qvector.h"
#include "../qhashtbl.h"
#include <stdio.h>
#include <cstdlib>

// enough space for test ir classes
static uint8_t all_runtime_classes_for_test[10000];

static uint32_t i32_class_bytes_offset = 0;
static uint32_t i64_class_bytes_offset = 0;
static uint32_t str_class_bytes_offset = 0;
static uint32_t i32_array_class_bytes_offset = 0;
static uint32_t str_array_class_bytes_offset = 0;
static uint32_t str_i32_map_class_bytes_offset = 0;
static uint32_t school_class_bytes_offset = 0;
static uint32_t person_class_bytes_offset = 0;
static uint32_t str_person_map_class_bytes_offset = 0;

static void init_test_ir_types() {
    ir_builtin_set_all_runtimes_classes_address((intptr_t) &all_runtime_classes_for_test);

    uint32_t offset = 0;

    {
        i32_class_bytes_offset = offset;
        IRRuntimeClass i32_class = {
                .size = 4,
                .ty = IRRuntimeType::IR_RUNTIME_TYPE_I32,
                .struct_fields = 0,
                .struct_field_names = 0,
                .array_item_ty = 0,
                .array_size = 0,
                .map_key_ty = 0,
                .map_value_ty = 0,
        };

        ::memcpy(&all_runtime_classes_for_test[offset], &i32_class, sizeof(IRRuntimeClass));
        offset += sizeof(IRRuntimeClass);
    }

    {
        i64_class_bytes_offset = offset;
        IRRuntimeClass i64_class = {
                .size = 8,
                .ty = IRRuntimeType::IR_RUNTIME_TYPE_I64,
                .struct_fields = 0,
                .struct_field_names = 0,
                .array_item_ty = 0,
                .array_size = 0,
                .map_key_ty = 0,
                .map_value_ty = 0,
        };

        ::memcpy(&all_runtime_classes_for_test[offset], &i64_class, sizeof(IRRuntimeClass));
        offset += sizeof(IRRuntimeClass);
    }

    {
        str_class_bytes_offset = offset;
        IRRuntimeClass str_class = {
                .size = 4,
                .ty = IRRuntimeType::IR_RUNTIME_TYPE_STR,
                .struct_fields = 0,
                .struct_field_names = 0,
                .array_item_ty = 0,
                .array_size = 0,
                .map_key_ty = 0,
                .map_value_ty = 0,
        };

        ::memcpy(&all_runtime_classes_for_test[offset], &str_class, sizeof(IRRuntimeClass));
        offset += sizeof(IRRuntimeClass);
    }

    {
        i32_array_class_bytes_offset = offset;
        IRRuntimeClass i32_array_class = {
                .size = 4,
                .ty = IRRuntimeType::IR_RUNTIME_TYPE_ARRAY,
                .struct_fields = 0,
                .struct_field_names = 0,
                .array_item_ty = i32_class_bytes_offset,
                .array_size = 0,
                .map_key_ty = 0,
                .map_value_ty = 0,
        };

        ::memcpy(&all_runtime_classes_for_test[offset], &i32_array_class, sizeof(IRRuntimeClass));
        offset += sizeof(IRRuntimeClass);
    }

    {
        str_array_class_bytes_offset = offset;
        IRRuntimeClass str_array_class = {
                .size = 4,
                .ty = IRRuntimeType::IR_RUNTIME_TYPE_ARRAY,
                .struct_fields = 0,
                .struct_field_names = 0,
                .array_item_ty = str_class_bytes_offset,
                .array_size = 0,
                .map_key_ty = 0,
                .map_value_ty = 0,
        };

        ::memcpy(&all_runtime_classes_for_test[offset], &str_array_class, sizeof(IRRuntimeClass));
        offset += sizeof(IRRuntimeClass);
    }

    {
        str_i32_map_class_bytes_offset = offset;
        IRRuntimeClass str_i32_map_class = {
                .size = 4,
                .ty = IRRuntimeType::IR_RUNTIME_TYPE_MAP,
                .struct_fields = 0,
                .struct_field_names = 0,
                .array_item_ty = 0,
                .array_size = 0,
                .map_key_ty = str_class_bytes_offset,
                .map_value_ty = i32_class_bytes_offset,
        };

        ::memcpy(&all_runtime_classes_for_test[offset], &str_i32_map_class, sizeof(IRRuntimeClass));
        offset += sizeof(IRRuntimeClass);
    }

    // school ir class
    // name: str
    // students_count: i32
    {
        const uint32_t field_count = 2;

        uint32_t fields_array_offset = offset;
        {
            offset += field_count * sizeof(uint32_t);
            uint32_t fields[field_count] = {str_class_bytes_offset, i32_class_bytes_offset};
            ::memcpy(&all_runtime_classes_for_test[fields_array_offset], &fields, field_count * sizeof(uint32_t));
        }

        // "name" str
        uint32_t name_str_offset = offset;
        {
            const char *str = "name";
            uint32_t str_len = 4;
            ::memcpy(&all_runtime_classes_for_test[offset], &str_len, sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 4], &str_len,
                        sizeof(str_len)); // TODO: why 2 len
            ::memcpy(&all_runtime_classes_for_test[offset + 8], str, strlen(str));
            offset += 4 + 4 + strlen(str);
        }

        // "students_count" str
        uint32_t students_count_str_offset = offset;
        {
            const char *str = "students_count";
            uint32_t str_len = 4;
            ::memcpy(&all_runtime_classes_for_test[offset], &str_len, sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 4], &str_len,
                        sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 8], str, strlen(str));
            offset += 4 + 4 + strlen(str);
        }

        uint32_t fields_names_array_offset = offset;
        {
            uint32_t fields_names[field_count] = {name_str_offset, students_count_str_offset};
            ::memcpy(&all_runtime_classes_for_test[offset], &fields_names,
                        field_count * sizeof(uint32_t));
            offset += field_count * sizeof(uint32_t);
        }

        school_class_bytes_offset = offset;
        IRRuntimeClass school_class = {
                .size = sizeof(intptr_t),
                .ty = IRRuntimeType::IR_RUNTIME_TYPE_STRUCT,
                .struct_fields = fields_array_offset,
                .struct_fields_count = field_count,
                .struct_field_names = fields_names_array_offset,
                .array_item_ty = 0,
                .array_size = 0,
                .map_key_ty = 0,
                .map_value_ty = 0,
        };

        ::memcpy(&all_runtime_classes_for_test[offset], &school_class, sizeof(IRRuntimeClass));
        offset += sizeof(IRRuntimeClass);
    }

    // person ir class:
    // name: str
    // age: i64
    // school: School
    // friends: [str]
    // balances: [i32]
    // tokens: {str:i32}
    {
        const uint32_t field_count = 6;

        uint32_t fields_array_offset = offset;
        {
            offset += field_count * sizeof(uint32_t);
            uint32_t fields[field_count] = {str_class_bytes_offset, i64_class_bytes_offset, school_class_bytes_offset,
                                            str_array_class_bytes_offset, i32_array_class_bytes_offset,
                                            str_i32_map_class_bytes_offset};
            ::memcpy(&all_runtime_classes_for_test[fields_array_offset], &fields, field_count * sizeof(uint32_t));
        }

        // "name" str
        uint32_t name_str_offset = offset;
        {
            const char *str = "name";
            uint32_t str_len = strlen(str);
            ::memcpy(&all_runtime_classes_for_test[offset], &str_len, sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 4], &str_len,
                        sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 8], str, strlen(str));
            offset += 4 + 4 + strlen(str);
        }
        // "age" str
        uint32_t age_str_offset = offset;
        {
            const char *str = "age";
            uint32_t str_len = strlen(str);
            ::memcpy(&all_runtime_classes_for_test[offset], &str_len, sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 4], &str_len,
                        sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 8], str, strlen(str));
            offset += 4 + 4 + strlen(str);
        }
        // "school" str
        {
            const char *str = "school";
            uint32_t str_len = strlen(str);
            ::memcpy(&all_runtime_classes_for_test[offset], &str_len, sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 4], &str_len,
                        sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 8], str, strlen(str));
            offset += 4 + 4 + strlen(str);
        }
        // "friends" str
        uint32_t friends_str_offset = offset;
        {
            const char *str = "friends";
            uint32_t str_len = strlen(str);
            ::memcpy(&all_runtime_classes_for_test[offset], &str_len, sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 4], &str_len,
                        sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 8], str, strlen(str));
            offset += 4 + 4 + strlen(str);
        }
        // "balances" str
        uint32_t balances_str_offset = offset;
        {
            const char *str = "balances";
            uint32_t str_len = strlen(str);
            ::memcpy(&all_runtime_classes_for_test[offset], &str_len, sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 4], &str_len,
                        sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 8], str, strlen(str));
            offset += 4 + 4 + strlen(str);
        }
        // "tokens" str
        uint32_t tokens_str_offset = offset;
        {
            const char *str = "tokens";
            uint32_t str_len = strlen(str);
            ::memcpy(&all_runtime_classes_for_test[offset], &str_len, sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 4], &str_len,
                        sizeof(str_len));
            ::memcpy(&all_runtime_classes_for_test[offset + 8], str, strlen(str));
            offset += 4 + 4 + strlen(str);
        }

        uint32_t fields_names_array_offset = offset;
        {
            uint32_t fields_names[field_count] = {name_str_offset, age_str_offset, school_class_bytes_offset,
                                                  friends_str_offset,
                                                  balances_str_offset, tokens_str_offset};
            ::memcpy(&all_runtime_classes_for_test[offset], &fields_names,
                        field_count * sizeof(uint32_t));
            offset += field_count * sizeof(uint32_t);
        }

        person_class_bytes_offset = offset;
        IRRuntimeClass person_class = {
                .size = sizeof(intptr_t),
                .ty = IRRuntimeType::IR_RUNTIME_TYPE_STRUCT,
                .struct_fields = fields_array_offset,
                .struct_fields_count = field_count,
                .struct_field_names = fields_names_array_offset,
                .array_item_ty = 0,
                .array_size = 0,
                .map_key_ty = 0,
                .map_value_ty = 0,
        };

        ::memcpy(&all_runtime_classes_for_test[offset], &person_class, sizeof(IRRuntimeClass));
        offset += sizeof(IRRuntimeClass);
    }

    {
        str_person_map_class_bytes_offset = offset;
        IRRuntimeClass str_person_map_class = {
                .size = 4,
                .ty = IRRuntimeType::IR_RUNTIME_TYPE_MAP,
                .struct_fields = 0,
                .struct_field_names = 0,
                .array_item_ty = 0,
                .array_size = 0,
                .map_key_ty = str_class_bytes_offset,
                .map_value_ty = person_class_bytes_offset,
        };

        ::memcpy(&all_runtime_classes_for_test[offset], &str_person_map_class, sizeof(IRRuntimeClass));
        offset += sizeof(IRRuntimeClass);
    }

    // TODO: asset test
}

TEST(ir_type, get_ir_type_size_as_element) {
    init_test_ir_types();

    ASSERT_EQ(4, get_ir_type_size_as_element(
            reinterpret_cast<IRRuntimeClass *>(&all_runtime_classes_for_test[i32_class_bytes_offset]))); // i32
    ASSERT_EQ(8, get_ir_type_size_as_element(
            reinterpret_cast<IRRuntimeClass *>(&all_runtime_classes_for_test[i64_class_bytes_offset]))); // i64
    ASSERT_EQ(sizeof(intptr_t), get_ir_type_size_as_element(
            reinterpret_cast<IRRuntimeClass *>(&all_runtime_classes_for_test[str_class_bytes_offset]))); // str
    ASSERT_EQ(sizeof(intptr_t), get_ir_type_size_as_element(
            reinterpret_cast<IRRuntimeClass *>(&all_runtime_classes_for_test[i32_array_class_bytes_offset]))); // i32_array
    ASSERT_EQ(sizeof(intptr_t), get_ir_type_size_as_element(
            reinterpret_cast<IRRuntimeClass *>(&all_runtime_classes_for_test[str_array_class_bytes_offset]))); // str_array
    ASSERT_EQ(sizeof(intptr_t), get_ir_type_size_as_element(
            reinterpret_cast<IRRuntimeClass *>(&all_runtime_classes_for_test[str_i32_map_class_bytes_offset]))); // str_i32_map
    ASSERT_EQ(sizeof(intptr_t), get_ir_type_size_as_element(
            reinterpret_cast<IRRuntimeClass *>(&all_runtime_classes_for_test[school_class_bytes_offset]))); // struct School
    ASSERT_EQ(sizeof(intptr_t), get_ir_type_size_as_element(
            reinterpret_cast<IRRuntimeClass *>(&all_runtime_classes_for_test[person_class_bytes_offset]))); // struct Person
    ASSERT_EQ(sizeof(intptr_t), get_ir_type_size_as_element(
            reinterpret_cast<IRRuntimeClass *>(&all_runtime_classes_for_test[str_person_map_class_bytes_offset]))); // {str: struct Person}
}

TEST(ir_type, is_pointer_ir_type) {
    init_test_ir_types();

    ASSERT_TRUE(is_pointer_ir_type(IR_RUNTIME_TYPE_STR));
    ASSERT_TRUE(is_pointer_ir_type(IR_RUNTIME_TYPE_ASSET));
    ASSERT_TRUE(is_pointer_ir_type(IR_RUNTIME_TYPE_STRUCT));
    ASSERT_TRUE(is_pointer_ir_type(IR_RUNTIME_TYPE_ARRAY));
    ASSERT_TRUE(is_pointer_ir_type(IR_RUNTIME_TYPE_MAP));
    ASSERT_TRUE(!is_pointer_ir_type(IR_RUNTIME_TYPE_I16));
}

TEST(ir_type, create_ir_value) {
    init_test_ir_types();

    {
        // test create i32
        auto value = ir_builtin_create_ir_value(i32_class_bytes_offset);
        ASSERT_TRUE(nullptr == value);
    }
    {
        // test create i64
        auto value = ir_builtin_create_ir_value(i64_class_bytes_offset);
        ASSERT_TRUE(nullptr == value);
    }
    {
        // test create str
        auto value = ir_builtin_create_ir_value(str_class_bytes_offset);
        auto str_value = reinterpret_cast<struct vector *>(value);
        ASSERT_EQ(str_value->len, 0);
    }
    {
        // test create i32-array
        auto value = ir_builtin_create_ir_value(i32_array_class_bytes_offset);
        auto vector_value = reinterpret_cast<qvector_t *>(value);
        ASSERT_EQ(vector_value->num, 0);
    }
    {
        // test create str-array
        auto value = ir_builtin_create_ir_value(str_array_class_bytes_offset);
        auto vector_value = reinterpret_cast<qvector_t *>(value);
        ASSERT_EQ(vector_value->num, 0);
    }
    {
        // test create str-i32-map
        auto value = ir_builtin_create_ir_value(str_i32_map_class_bytes_offset);
        auto map_value = reinterpret_cast<qhashtbl_t *>(value);
        ASSERT_EQ(map_value->num, 0);
    }
    struct school {
        intptr_t name_offset;
        uint32_t students_count;
    };
    struct person {
        intptr_t name_offset;
        int64_t age;
        intptr_t school;
        intptr_t friends_offset; // vector
        intptr_t balances_offset; // vector
        intptr_t tokens_offset; // map
    };
    {
        // test create struct School
        auto value = (uint8_t *) ir_builtin_create_ir_value(school_class_bytes_offset);
        auto school_value = reinterpret_cast<school *>(value);
        auto school_name = reinterpret_cast<vector *>(school_value->name_offset);
        ASSERT_EQ(school_name->len, 0);
        ASSERT_EQ(school_value->students_count, 0);
    }
    {
        // test create struct Person
        auto value = (uint8_t *) ir_builtin_create_ir_value(person_class_bytes_offset);
        auto person_value = reinterpret_cast<person *>(value);
        auto person_name = reinterpret_cast<vector *>(person_value->name_offset);
        ASSERT_EQ(person_name->len, 0);
        ASSERT_EQ(person_value->age, 0);
        auto school_value = reinterpret_cast<school *>(person_value->school);
        auto school_name = reinterpret_cast<vector *>(school_value->name_offset);
        ASSERT_EQ(school_name->len, 0);
        ASSERT_EQ(school_value->students_count, 0);
        auto friends = reinterpret_cast<qvector_t *>(person_value->friends_offset);
        ASSERT_EQ(friends->num, 0);
        auto balances = reinterpret_cast<qvector_t *>(person_value->balances_offset);
        ASSERT_EQ(balances->num, 0);
        auto tokens = reinterpret_cast<qhashtbl_t *>(person_value->tokens_offset);
        ASSERT_EQ(tokens->num, 0);
    }
    {
        // test create {str,struct Person}
        auto value = ir_builtin_create_ir_value(str_person_map_class_bytes_offset);
        auto map_value = reinterpret_cast<qhashtbl_t *>(value);
        ASSERT_EQ(map_value->num, 0);
    }
}