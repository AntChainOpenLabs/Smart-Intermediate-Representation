// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#ifndef __IR_TYPE_H_
#define __IR_TYPE_H_

#include "./stdlib.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum IRRuntimeType {
    // make sure ints first, so can easily check is integer
    IR_RUNTIME_TYPE_U8 = 0,
    IR_RUNTIME_TYPE_U16 = 1,
    IR_RUNTIME_TYPE_U32 = 2,
    IR_RUNTIME_TYPE_U64 = 3,
    IR_RUNTIME_TYPE_U128 = 4,

    IR_RUNTIME_TYPE_I8 = 5,
    IR_RUNTIME_TYPE_I16 = 6,
    IR_RUNTIME_TYPE_I32 = 7,
    IR_RUNTIME_TYPE_I64 = 8,
    IR_RUNTIME_TYPE_I128 = 9,

    IR_RUNTIME_TYPE_BOOL = 10,
    IR_RUNTIME_TYPE_STR = 11,
    IR_RUNTIME_TYPE_ASSET = 12,
    IR_RUNTIME_TYPE_STRUCT = 13,
    IR_RUNTIME_TYPE_ARRAY = 14,
    IR_RUNTIME_TYPE_MAP = 15,

    IR_RUNTIME_TYPE_U256 = 16,
    IR_RUNTIME_TYPE_I256 = 17,
} IRRuntimeType;

#define DefaultIRRuntimeIntegerTypeMaxEnum IR_RUNTIME_TYPE_I128

inline bool
__attribute__((artificial)) __attribute__((always_inline)) 
is_pointer_ir_type(IRRuntimeType ty) {
    switch (ty) {
        case IR_RUNTIME_TYPE_STR:
        case IR_RUNTIME_TYPE_ASSET:
        case IR_RUNTIME_TYPE_STRUCT:
        case IR_RUNTIME_TYPE_ARRAY:
        case IR_RUNTIME_TYPE_MAP:
            return true;
        default: return false;
    }
}

typedef struct IRRuntimeClass {
    uint32_t size; // 4bytes, size of ir value in memory
    uint32_t ty; // enum IRRuntimeType
    uint32_t struct_fields; // 4bytes, offset of array of (offset of IRRuntimeClass* begin from all_runtime_classes)
    uint32_t struct_fields_count;
    uint32_t struct_field_names; // 4bytes, offset of array of fields name
    uint32_t array_item_ty; // 4bytes, offset of IRRuntimeClass* begin from all_runtime_classes
    uint32_t array_size; // 4bytes, size of array type. 0 denotes vector and N donetes size of type `arr[T;N]`
    uint32_t map_key_ty; // 4bytes, offset of IRRuntimeClass* begin from all_runtime_classes
    uint32_t map_value_ty; // 4bytes, offset of IRRuntimeClass* begin from all_runtime_classes
} IRRuntimeClass;


// print type info of ir value
// all_runtimes_classes_address + runtime_class_offset = struct IRRuntimeClass *runtime_class
extern void
ir_builtin_print_type(uint32_t runtime_class_offset);
extern void*
ir_builtin_create_ir_value(uint32_t runtime_class_offset);

// intptr_t is int32 when target=wasm32
extern void
ir_builtin_set_all_runtimes_classes_address(intptr_t all_runtimes_classes_address);

extern uint32_t 
get_ir_type_size_as_element(struct IRRuntimeClass *runtime_class);

// intptr_t is int32 when target=wasm32
extern intptr_t 
get_all_runtimes_classes_address();

bool
is_pointer_type(uint32_t runtime_class_offset);

// If this type is a pointer type, returns the address(data address) pointed to
// by this ptr, otherwise returns the address of this ptr
void *
get_data_ptr_of_ptr_value(uint32_t runtime_class_offset, void *val);

// If this type is a pointer type, returns the a ptr pointed to
// this ptr, otherwise returns the this ptr
void *
get_ptr_of_ptr_value(uint32_t runtime_class_offset, void *val);

// Returns the address of the data subscripted as idx in the array
void *
get_array_elem_ptr_at_idx(uint32_t runtime_class_offset, void *val,
                          uint32_t idx);
// Get the real malloc size of the ir-type value
size_t calculate_ir_type_size(struct IRRuntimeClass *runtime_class);

#ifdef __cplusplus
} // end "C"
#endif

#endif // __IR_TYPE_H_
