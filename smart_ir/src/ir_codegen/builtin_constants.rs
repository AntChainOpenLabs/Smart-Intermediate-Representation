// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

pub const CONTRACT_INTERNAL_METHOD_PREFIX: &str = "$ir_contract_internal";
pub const BUILTIN_FUNCTION_MANGLE_PREFIX: &str = "ir_builtin";
pub const VECTOR_NEW_FUNC_NAME: &str = "vector_new";
pub const Q_VEC_NEW_FUNC_NAME: &str = "qvector";
pub const Q_VEC_SETDATA_FUNC_NAME: &str = "qvector_setdata";
pub const Q_VEC_SIZE_FUNC_NAME: &str = "qvector_size";
pub const Q_VEC_DATA_FUNC_NAME: &str = "qvector_data";
pub const Q_VEC_SLICE_FUNC_NAME: &str = "qvector_slice";
pub const Q_MAP_NEW_FUNC_NAME: &str = "qhashtbl";
pub const RUNTIME_CONTEXT_LLVM_TY: &str = "struct.RuntimeContext";
pub const Q_VECTOR_OBJ_S: &str = "struct.qvector_obj_s";
pub const Q_VECTOR_ITER: &str = "struct.qvector_iter";
pub const Q_HASHTBL_OBJ_S: &str = "struct.qhashtbl_obj_s";
pub const Q_HASHTBL_ITER: &str = "struct.qhashtbl_iter";
pub const SSZ_ENCODE_LEN: &str = "ssz_encode_len";
pub const IR_BUILTIN_SSZ_ENCODE_VOID_PTR: &str = "ir_builtin_ssz_encode_void_ptr";
pub const IR_BUILTIN_SSZ_DECODE_VOID_PTR: &str = "ir_builtin_ssz_decode_void_ptr";
pub const MEMCMP_FUNC: &str = "__memcmp";
