// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./ir_type.h"
#include "./qvector.h"
#include "./qhashtbl.h"

#ifndef CC_LIB_TEST_MOCK

#define ADDRESS_SIZE 4

#define assert(x) (0)

extern void *
__malloc(size_t size);
extern void *
memcpy(void *dest, const void *src, uint32_t length);
#else

#define ADDRESS_SIZE sizeof(intptr_t)

extern void
__memset(void *dest, uint8_t val, size_t length);
#include <assert.h>
#include <string.h>
#endif // CC_LIB_TEST_MOCK

extern void
println(const char* src, uint32_t len);

intptr_t global_all_runtimes_classes_address = 0;

void ir_builtin_set_all_runtimes_classes_address(intptr_t all_runtimes_classes_address)
{
    global_all_runtimes_classes_address = all_runtimes_classes_address;
}

extern intptr_t
get_all_runtimes_classes_address()
{
    return global_all_runtimes_classes_address;
}

void ir_builtin_print_type(uint32_t runtime_class_offset)
{
    intptr_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class = (struct IRRuntimeClass *) (all_runtimes_classes_address + runtime_class_offset);
    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8: {
            println("u8", 2);
        } break;
        case IR_RUNTIME_TYPE_U16: {
            println("u16", 3);
        } break;
        case IR_RUNTIME_TYPE_U32: {
            println("u32", 3);
        } break;
        case IR_RUNTIME_TYPE_U64: {
            println("u64", 3);
        } break;
        case IR_RUNTIME_TYPE_U128: {
            println("u128", 4);
        } break;
        case IR_RUNTIME_TYPE_U256: {
            println("i256", 4);
        } break;
        case IR_RUNTIME_TYPE_I8: {
            println("i8", 2);
        } break;
        case IR_RUNTIME_TYPE_I16: {
            println("u16", 3);
        } break;
        case IR_RUNTIME_TYPE_I32: {
            println("i32", 3);
        } break;
        case IR_RUNTIME_TYPE_I64: {
            println("i64", 3);
        } break;
        case IR_RUNTIME_TYPE_I128: {
            println("i128", 4);
        } break;
        case IR_RUNTIME_TYPE_I256: {
            println("i256", 4);
        } break;
        case IR_RUNTIME_TYPE_BOOL: {
            println("bool", 4);
        } break;
        case IR_RUNTIME_TYPE_STR: {
            println("string", 6);
        } break;
        case IR_RUNTIME_TYPE_ASSET: {
            println("asset", 5);
            println("fields:", 7);
            uint32_t* fields_offsets_array = (uint32_t*)(all_runtimes_classes_address + runtime_class->struct_fields);
            uint32_t* fields_names_array = (uint32_t*)(all_runtimes_classes_address + runtime_class->struct_field_names);
            for (uint32_t i=0;i<runtime_class->struct_fields_count;i++) {
                uint32_t fields_name_offset = fields_names_array[i];
                struct vector *fields_name_ptr = (struct vector *)(all_runtimes_classes_address + fields_name_offset);
                uint32_t fields_name_bytes_offset = (uint32_t)fields_name_ptr->data;
                uint32_t field_name_bytes = all_runtimes_classes_address + fields_name_bytes_offset;
                println((const char *)field_name_bytes, fields_name_ptr->len);
                uint32_t field_offset = fields_offsets_array[i];
                ir_builtin_print_type(field_offset);
            }
        } break;
        case IR_RUNTIME_TYPE_STRUCT: {
            println("struct", 6);
            println("fields:", 7);
            uint32_t* fields_offsets_array = (uint32_t*)(all_runtimes_classes_address + runtime_class->struct_fields);
            uint32_t* fields_names_array = (uint32_t*)(all_runtimes_classes_address + runtime_class->struct_field_names);
            for (uint32_t i=0;i<runtime_class->struct_fields_count;i++) {
                uint32_t fields_name_offset = fields_names_array[i];
                struct vector *fields_name_ptr = (struct vector *)(all_runtimes_classes_address + fields_name_offset);
                uint32_t fields_name_bytes_offset = (uint32_t)fields_name_ptr->data;
                int32_t field_name_bytes = all_runtimes_classes_address + fields_name_bytes_offset;
                println((const char *)field_name_bytes, fields_name_ptr->len);
                uint32_t field_offset = fields_offsets_array[i];
                ir_builtin_print_type(field_offset);
            }
        } break;
        case IR_RUNTIME_TYPE_ARRAY: {
            if (runtime_class->array_size != 0) {
                println("array", 5);
                println("size:", 5);
                char *array_size = builtin_i32_toa(runtime_class->array_size, 10);
                println(array_size, __strlen(array_size));
            } else {
                println("vector", 6);
            }
            println("element:", 8);
            ir_builtin_print_type(runtime_class->array_item_ty);
        } break;
        case IR_RUNTIME_TYPE_MAP: {
            println("map", 3);
            println("key:", 4);
            ir_builtin_print_type(runtime_class->map_key_ty);
            println("value:", 6);
            ir_builtin_print_type(runtime_class->map_value_ty);
        } break;
        default: {
            char msg[] = "unknown ir runtime type in print_type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

extern qvector_t *
qvector(size_t max, size_t objsize, int options);

// get the memory size of ir type as other member of struct, not memory size of current struct
extern uint32_t 
get_ir_type_size_as_element(struct IRRuntimeClass *runtime_class)
{
    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8: {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_U16: {
            return 2;
        } break;
        case IR_RUNTIME_TYPE_U32: {
            return 4;
        } break;
        case IR_RUNTIME_TYPE_U64: {
            return 8;
        } break;
        case IR_RUNTIME_TYPE_U128: {
            return 16;
        } break;
        case IR_RUNTIME_TYPE_U256: {
            return 32;
        } break;
        case IR_RUNTIME_TYPE_I8: {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_I16: {
            return 2;
        } break;
        case IR_RUNTIME_TYPE_I32: {
            return 4;
        } break;
        case IR_RUNTIME_TYPE_I64: {
            return 8;
        } break;
        case IR_RUNTIME_TYPE_I128: {
            return 16;
        } break;
        case IR_RUNTIME_TYPE_I256: {
            return 32;
        } break;
        case IR_RUNTIME_TYPE_BOOL: {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_STR: {
            return ADDRESS_SIZE; // pointer
        } break;
        case IR_RUNTIME_TYPE_ASSET: {
            return ADDRESS_SIZE; // pointer
        } break;
        case IR_RUNTIME_TYPE_STRUCT: {
            return ADDRESS_SIZE; // pointer
        } break;
        case IR_RUNTIME_TYPE_ARRAY: {
            return ADDRESS_SIZE; // pointer
        } break;
        case IR_RUNTIME_TYPE_MAP: {
            return ADDRESS_SIZE; // pointer
        } break;
        default: {
            return ADDRESS_SIZE; // pointer
        }
    }
}

// get the real malloc size of the ir-type value
size_t calculate_ir_type_size(struct IRRuntimeClass *runtime_class) {
    intptr_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8: {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_U16: {
            return 2;
        } break;
        case IR_RUNTIME_TYPE_U32: {
            return 4;
        } break;
        case IR_RUNTIME_TYPE_U64: {
            return 8;
        } break;
        case IR_RUNTIME_TYPE_U128: {
            return 16;
        } break;
        case IR_RUNTIME_TYPE_U256: {
            return 32;
        } break;
        case IR_RUNTIME_TYPE_I8: {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_I16: {
            return 2;
        } break;
        case IR_RUNTIME_TYPE_I32: {
            return 4;
        } break;
        case IR_RUNTIME_TYPE_I64: {
            return 8;
        } break;
        case IR_RUNTIME_TYPE_I128: {
            return 16;
        } break;
        case IR_RUNTIME_TYPE_I256: {
            return 32;
        } break;
        case IR_RUNTIME_TYPE_BOOL: {
            return 1;
        } break;
        case IR_RUNTIME_TYPE_STR: {
            return sizeof(struct vector);
        } break;
        case IR_RUNTIME_TYPE_ASSET: {
            // asset is a kind of struct
            size_t total = 0;
            uint32_t* fields_offsets_array = (uint32_t*)(all_runtimes_classes_address + runtime_class->struct_fields);
            for (uint32_t i=0;i<runtime_class->struct_fields_count;i++) {
                uint32_t field_offset = fields_offsets_array[i];
                struct IRRuntimeClass *field_type = (struct IRRuntimeClass *) (all_runtimes_classes_address + field_offset);
                total += get_ir_type_size_as_element(field_type);    
            }
            if (total == 0) {
                total = 4; // at least need malloc 4bytes
            }
            return total;
        } break;
        case IR_RUNTIME_TYPE_STRUCT: {
            size_t total = 0;
            uint32_t* fields_offsets_array = (uint32_t*)(all_runtimes_classes_address + runtime_class->struct_fields);

            for (uint32_t i=0;i<runtime_class->struct_fields_count;i++) {
                uint32_t field_offset = fields_offsets_array[i];
                struct IRRuntimeClass *field_type = (struct IRRuntimeClass *) (all_runtimes_classes_address + field_offset);
                total += get_ir_type_size_as_element(field_type);
            }
            if (total == 0) {
                total = 4; // at least need malloc 4bytes
            }
            return total;
        } break;
        case IR_RUNTIME_TYPE_ARRAY: {
            return sizeof(qvector_t);
        } break;
        case IR_RUNTIME_TYPE_MAP: {
            return sizeof(qhashtbl_t);
        } break;
        default: {
            char msg[] = "not supported ir type to get type size";
            IR_ABORT(msg, sizeof(msg) - 1);
            return 0;
        }
    }
}

void*
ir_builtin_create_ir_value(uint32_t runtime_class_offset)
{
    intptr_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class = (struct IRRuntimeClass *) (all_runtimes_classes_address + runtime_class_offset);
    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8: {
            return (void*) 0;
        } break;
        case IR_RUNTIME_TYPE_U16: {
            return (void*) 0;
        } break;
        case IR_RUNTIME_TYPE_U32: {
            return (void*) 0;
        } break;
        case IR_RUNTIME_TYPE_U64: {
            return (void*) 0L;
        } break;
        case IR_RUNTIME_TYPE_U128: {
            // uint128_ = uint64_t[2]
            uint64_t* value = (uint64_t*) __malloc(sizeof(uint64_t[2]));
            __memset(value, sizeof(uint64_t[2]), 0);
            return (void*) value;
        } break;
         case IR_RUNTIME_TYPE_U256: {
            uint64_t* value = (uint64_t*) __malloc(sizeof(uint64_t[4]));
            __memset(value, sizeof(uint64_t[4]), 0);
             return (void*) value;
        } break;
        case IR_RUNTIME_TYPE_I8: {
            return (void*) 0;
        } break;
        case IR_RUNTIME_TYPE_I16: {
            return (void*) 0;
        } break;
        case IR_RUNTIME_TYPE_I32: {
            return (void*) 0;
        } break;
        case IR_RUNTIME_TYPE_I64: {
            return (void*) 0L;
        } break;
        case IR_RUNTIME_TYPE_I128: {
            // iint128_ = uint64_t[2]
            uint64_t* value = (uint64_t*) __malloc(sizeof(uint64_t[2]));
            __memset(value, sizeof(uint64_t[2]), 0);
            return (void*) value;
        } break;
        case IR_RUNTIME_TYPE_I256: {
            uint64_t* value = (uint64_t*) __malloc(sizeof(uint64_t[4]));
            __memset(value, sizeof(uint64_t[4]), 0);
             return (void*) value;
        } break;
        case IR_RUNTIME_TYPE_BOOL: {
            return (void*) 0;
        } break;
        case IR_RUNTIME_TYPE_STR: {
            return vector_new(0, 1, (uint8_t*) "");
        } break;
        case IR_RUNTIME_TYPE_ASSET: {
            // malloc and memset size of asset
            // refer to the member type
            size_t value_size = calculate_ir_type_size(runtime_class);
            void* value = __malloc(value_size);
            __memset(value, 0x0, value_size);
            uint32_t* fields_offsets_array = (uint32_t*)(all_runtimes_classes_address + runtime_class->struct_fields);
            size_t offset = 0; // current attribute offset
            // initial each attr of the struct
            for (uint32_t i=0;i<runtime_class->struct_fields_count;i++) {
                uint32_t field_type_offset = fields_offsets_array[i];
                IRRuntimeClass *field_type = (IRRuntimeClass *) (all_runtimes_classes_address + field_type_offset);
                size_t field_size_as_element = get_ir_type_size_as_element(field_type); // this element memory size while as member(pointer or primitive type)
                void *field_init_value = ir_builtin_create_ir_value(field_type_offset);
                memcpy(value + offset, &field_init_value, field_size_as_element);
                offset += field_size_as_element;
            }
            return value;
        } break;
        case IR_RUNTIME_TYPE_STRUCT: {
            // malloc and memset size of struct
            // refer to the member type
            size_t value_size = calculate_ir_type_size(runtime_class);
            void* value = __malloc(value_size);
            __memset(value, 0x0, value_size);
            uint32_t* fields_offsets_array = (uint32_t*)(all_runtimes_classes_address + runtime_class->struct_fields);
            size_t offset = 0; // current attribute offset
            // initial each attr of the struct
            for (uint32_t i=0;i<runtime_class->struct_fields_count;i++) {
                uint32_t field_type_offset = fields_offsets_array[i];
                IRRuntimeClass *field_type = (IRRuntimeClass *) (all_runtimes_classes_address + field_type_offset);
                size_t field_size_as_element = get_ir_type_size_as_element(field_type); // this element memory size while as member(pointer or primitive type)
                void *field_init_value = ir_builtin_create_ir_value(field_type_offset);
                memcpy(value + offset, &field_init_value, field_size_as_element);
                offset += field_size_as_element;
            }
            return value;
        } break;
        case IR_RUNTIME_TYPE_ARRAY: {
            struct IRRuntimeClass *array_item_ty = (struct IRRuntimeClass *)(
                                                all_runtimes_classes_address + runtime_class->array_item_ty);
            size_t element_size = get_ir_type_size_as_element(array_item_ty);
            return qvector(1, element_size, 0x02 /* QVECTOR_RESIZE_DOUBLE */);
        } break;
        case IR_RUNTIME_TYPE_MAP: {
            struct IRRuntimeClass *map_key_ty = (struct IRRuntimeClass *)(all_runtimes_classes_address + runtime_class->map_key_ty);
            return qhashtbl(0, map_key_ty->ty, 0);
        } break;
        default: {
            char msg[] = "unknown ir runtime type in create ir value";
            IR_ABORT(msg, sizeof(msg) - 1);
            return 0;
        }
    }
}

// Is this type a pointer type in ir, return true when the type is str, arr,
// asset, struct, map
bool
is_pointer_type(uint32_t runtime_class_offset)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);
    switch (runtime_class->ty) {
        case IR_RUNTIME_TYPE_U8:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_U16:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_U32:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_U64:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_U128:
        {
            return false; // pointer
        } break;
        case IR_RUNTIME_TYPE_U256: {
            return false; // pointer
        } break;
        case IR_RUNTIME_TYPE_I8:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_I16:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_I32:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_I64:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_I128:
        {
            return false; // pointer
        } break;
        case IR_RUNTIME_TYPE_I256: {
            return false; // pointer
        } break;
        case IR_RUNTIME_TYPE_BOOL:
        {
            return false;
        } break;
        case IR_RUNTIME_TYPE_STR:
        {
            return true; // pointer
        } break;
        case IR_RUNTIME_TYPE_ASSET:
        {
            return true; // pointer
        } break;
        case IR_RUNTIME_TYPE_STRUCT:
        {
            return true; // pointer
        } break;
        case IR_RUNTIME_TYPE_ARRAY:
        {
            return true; // pointer
        } break;
        case IR_RUNTIME_TYPE_MAP:
        {
            return true; // pointer
        } break;
        default:
        {
            char msg[] = "unknown ir runtime type in ssz type";
            IR_ABORT(msg, sizeof(msg) - 1);
        }
    }
}

// If this type is a pointer type, returns the address(data address) pointed to
// by this ptr, otherwise returns the address of this ptr
void *
get_data_ptr_of_ptr_value(uint32_t runtime_class_offset, void *val)
{

    if (is_pointer_type(runtime_class_offset)) {
        uint32_t ptr_addr = *((uint32_t *)val);
        return (void *)ptr_addr;
    }
    else {
        return val;
    }
}

// If this type is a pointer type, returns the a ptr pointed to
// this ptr, otherwise returns the this ptr
void *
get_ptr_of_ptr_value(uint32_t runtime_class_offset, void *val)
{
    if (is_pointer_type(runtime_class_offset)) {
        void **ret = (void **)malloc(sizeof(void *));
        *ret = val;
        return ret;
    }
    else {
        return val;
    }
}

// Returns the address of the data subscripted as idx in the array
void *
get_array_elem_ptr_at_idx(uint32_t runtime_class_offset, void *val,
                          uint32_t idx)
{
    int32_t all_runtimes_classes_address = get_all_runtimes_classes_address();
    struct IRRuntimeClass *runtime_class =
        (struct IRRuntimeClass *)(all_runtimes_classes_address
                                     + runtime_class_offset);

    assert(runtime_class->ty = IR_RUNTIME_TYPE_ARRAY);

    qvector_t *_val = (qvector_t *)val;
    assert(idx < _val->num);

    uint32_t elem_ty_offset = runtime_class->array_item_ty;

    return get_data_ptr_of_ptr_value(elem_ty_offset,
                                     (_val->data + idx * _val->objsize));
}