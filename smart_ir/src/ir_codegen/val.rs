// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use inkwell::values::{BasicValueEnum, IntValue, PointerValue};

use super::context::IR2LLVMCodeGenContext;

/// Impl TypedResultWalker for LLVMCodeGenContext to visit AST nodes to emit LLVM IR.
impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    /// Return the pointer to the actual bytes in the vector
    pub fn vector_bytes(&self, vector: BasicValueEnum<'ctx>) -> PointerValue<'ctx> {
        if vector.is_struct_value() {
            // slice
            let slice = vector.into_struct_value();

            self.builder
                .build_extract_value(slice, 2, "slice_data")
                .unwrap()
                .into_pointer_value()
        } else {
            self.build_call("vector_bytes", &[vector])
                .into_pointer_value()
        }
    }

    /// Number of element in a vector
    pub fn vector_len(&self, vector: BasicValueEnum<'ctx>) -> IntValue<'ctx> {
        if vector.is_struct_value() {
            // slice
            let slice = vector.into_struct_value();

            self.builder
                .build_extract_value(slice, 0, "slice_len")
                .unwrap()
                .into_int_value()
        } else {
            self.build_call("vector_len", &[vector]).into_int_value()
        }
    }

    /// Number of element in a qvector.
    #[inline]
    pub fn q_vector_len(&self, vector: BasicValueEnum<'ctx>) -> IntValue<'ctx> {
        self.build_call("qvector_size", &[vector]).into_int_value()
    }
}
