// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use compiler_base_span::fatal_error::FatalError;
use indexmap::IndexMap;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::passes::{PassManager, PassManagerBuilder};
use inkwell::targets::{CodeModel, FileType, RelocMode, TargetTriple};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{
    BasicMetadataValueEnum, BasicValueEnum, FunctionValue, GlobalValue, IntValue, PointerValue,
};

use super::error::*;
use super::traits::*;
use crate::encoding::datastream::{ParamType, DEFAULT_VERSION};
use crate::integration::hostapi::HostAPI;
use crate::ir::builder::BasicBlockId;
use crate::ir::cfg::{BinaryOp, Contract, FunctionDefinition, Type};
use crate::ir::context::IRContext;
use crate::ir::metadata::debug_info::DebugLocation;
use crate::ir_codegen::common::global::{get_extend_context, has_extend_context};
use crate::ir_config::IROptions;
use crate::linker::link;
use inkwell::{AddressSpace, IntPredicate};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::rc::Rc;
use std::str;

type IRModule = crate::ir::cfg::Module;

use super::builtin_constants::{
    BUILTIN_FUNCTION_MANGLE_PREFIX, CONTRACT_INTERNAL_METHOD_PREFIX, RUNTIME_CONTEXT_LLVM_TY,
    VECTOR_NEW_FUNC_NAME,
};
use super::class_generator::ClassGenerator;
use super::const_storage_path_generator::ConstStoragePathGenerator;
use super::encoding::MALLOC_FUNC_NAME;
#[derive(Debug, Clone, Default)]
pub struct CodeGenError {
    pub message: String,
}

/// The compiler function result
pub type CompileResult<'a> = Result<BasicValueEnum<'a>, CodeGenError>;

/// The compiler scope.
pub struct Scope<'ctx> {
    pub variables: RefCell<IndexMap<String, PointerValue<'ctx>>>,
    pub variable_tys: RefCell<IndexMap<String, BasicTypeEnum<'ctx>>>,
}

#[derive(Default)]
pub struct RuntimeContext {
    /// Information about the original source.
    pub file_name: String,
    /// The (1-based) line number.
    pub line: u32,
    /// The (0-based) column offset when displayed.
    pub col: u32,
}
/// The LLVM code generator
pub struct IR2LLVMCodeGenContext<'ctx> {
    pub ir_context: &'ctx crate::ir::context::IRContext,
    pub llvm_context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    // pub program: &'ctx ast::Program,
    pub opts: &'ctx IROptions,
    pub functions: RefCell<Vec<Rc<FunctionValue<'ctx>>>>,
    pub intrinsics: RefCell<HashMap<String, FunctionValue<'ctx>>>,
    pub imported: RefCell<HashSet<String>>,
    pub global_strings: RefCell<IndexMap<String, IndexMap<String, PointerValue<'ctx>>>>,
    pub global_bytes: RefCell<IndexMap<String, IndexMap<String, PointerValue<'ctx>>>>,
    pub class_generator: RefCell<ClassGenerator<'ctx>>,
    pub const_storage_path_generator: RefCell<ConstStoragePathGenerator>,
    // pub all_runtime_classes_bytes: RefCell<Vec<u8>>, // constant bytes to store all runtime classes
    // pub type_runtime_classes: RefCell<IndexMap<TypeId, u32>>, // type id => offset in const all_runtime_classes_bytes
    pub all_abi_names: RefCell<Vec<String>>,

    pub pkg_scopes: RefCell<HashMap<String, Vec<Rc<Scope<'ctx>>>>>,
    pub label_target: RefCell<Vec<LabelTarget<'ctx>>>,
    pub current_runtime_ctx: RefCell<RuntimeContext>,
    /// IR BasicBlock -> LLVM Basic Block
    pub bb_map: RefCell<IndexMap<BasicBlockId, BasicBlock<'ctx>>>,
    pub ir_modules: Vec<IRModule>,
    pub func_definitions: RefCell<Vec<FunctionDefinition>>,
    pub current_module: RefCell<Option<IRModule>>,
    pub current_contract: RefCell<Option<Contract>>,
    pub current_function: RefCell<Option<FunctionDefinition>>,
}

#[derive(Default)]
pub struct Session<'ctx> {
    pub l_value: bool,
    pub store_value: Option<BasicValueEnum<'ctx>>,
    // Size of storage array, avoid duplicate read from storage
    pub array_size: Option<BasicValueEnum<'ctx>>,
    /// Recode recursion depth when entering the select/subscript expr to determine load or store in select and subscript stmt
    pub expr_deps: usize,
}

pub struct LabelTarget<'ctx> {
    // pub stmt: ast::Stmt,
    pub break_target_label: BasicBlock<'ctx>,
    pub continue_target_label: BasicBlock<'ctx>,
}

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    /// Push a function call frame into the function stack
    #[inline]
    pub fn push_function(&self, function: FunctionValue<'ctx>) {
        self.functions.borrow_mut().push(Rc::new(function));
    }

    /// Get the current function
    #[inline]
    pub fn current_function(&self) -> FunctionValue<'ctx> {
        **self.functions.borrow().last().expect(INTERNAL_ERROR_MSG)
    }

    /// Creates global string in the llvm module with initializer
    pub fn native_global_string(&self, value: &str, name: &str) -> PointerValue<'ctx> {
        let mut global_string_maps = self.global_strings.borrow_mut();
        let pkgpath = "__main__";
        if !global_string_maps.contains_key(pkgpath) {
            global_string_maps.insert(pkgpath.to_string(), IndexMap::default());
        }
        let msg = format!("pkgpath {pkgpath} is not found");
        let global_strings = global_string_maps.get_mut(pkgpath).expect(&msg);
        if let Some(ptr) = global_strings.get(value) {
            *ptr
        } else {
            let gv = unsafe { self.builder.build_global_string(value, name).unwrap() };
            let ptr = self
                .ptr_cast(
                    gv.as_pointer_value().into(),
                    self.ptr_type_to(self.i8_type()),
                )
                .into_pointer_value();
            global_strings.insert(value.to_string(), ptr);
            ptr
        }
    }

    /// build global constant bytes
    ///
    /// # Safety
    ///
    /// this is unsafe
    pub unsafe fn build_global_bytes(&self, value: &Vec<u8>, name: &str) -> GlobalValue<'ctx> {
        let i8_type = self.llvm_context.i8_type();
        let global = self
            .module
            .add_global(i8_type.array_type(value.len() as u32), None, name);
        let mut i8_array: Vec<IntValue> = Vec::new();
        for item in value {
            let i8_value = i8_type.const_int(*item as u64, false);
            i8_array.push(i8_value);
        }
        let llvm_bytes = i8_type.const_array(&i8_array);
        global.set_initializer(&llvm_bytes);
        global.set_constant(true);
        global.set_unnamed_addr(true);
        global
    }

    /// Creates global constant bytes in the llvm module with initializer
    pub fn native_global_bytes(&self, value: &Vec<u8>, name: &str) -> PointerValue<'ctx> {
        let mut global_bytes_maps = self.global_bytes.borrow_mut();
        let pkgpath = "__main__";
        if !global_bytes_maps.contains_key(pkgpath) {
            global_bytes_maps.insert(pkgpath.to_string(), IndexMap::default());
        }
        let msg = format!("pkgpath {pkgpath} is not found");
        let global_bytes = global_bytes_maps.get_mut(pkgpath).expect(&msg);
        if let Some(ptr) = global_bytes.get(name) {
            *ptr
        } else {
            let gv = unsafe { self.build_global_bytes(value, name) };
            let ptr = self
                .ptr_cast(
                    gv.as_pointer_value().into(),
                    self.ptr_type_to(self.i8_type()),
                )
                .into_pointer_value();
            global_bytes.insert(name.to_string(), ptr);
            ptr
        }
    }
}

impl<'ctx> CodeGenObject for BasicValueEnum<'ctx> {}

impl<'ctx> CodeGenObject for BasicTypeEnum<'ctx> {}

impl<'ctx> BackendTypes for IR2LLVMCodeGenContext<'ctx> {
    type Value = BasicValueEnum<'ctx>;
    type Type = BasicTypeEnum<'ctx>;
    type BasicBlock = BasicBlock<'ctx>;
    type Function = FunctionValue<'ctx>;
    type FunctionLet = FunctionType<'ctx>;
}

impl<'ctx> BuilderMethods for IR2LLVMCodeGenContext<'ctx> {
    /// SSA append a basic block named `name`.
    #[inline]
    fn append_block(&self, name: &str) -> Self::BasicBlock {
        let cur_func = self.current_function();
        self.llvm_context.append_basic_block(cur_func, name)
    }
    /// SSA ret instruction.
    #[inline]
    fn ret_void(&self) {
        self.builder.build_return(None).unwrap();
    }
    /// SSA ret instruction with returned value.
    #[inline]
    fn ret(&self, v: Self::Value) {
        self.builder.build_return(Some(&v)).unwrap();
    }
    /// SSA br instruction.
    #[inline]
    fn br(&self, dest: Self::BasicBlock) {
        self.builder.build_unconditional_branch(dest).unwrap();
    }
    /// SSA cond br instruction.
    #[inline]
    fn cond_br(&self, cond: Self::Value, then_bb: Self::BasicBlock, else_bb: Self::BasicBlock) {
        self.builder
            .build_conditional_branch(cond.into_int_value(), then_bb, else_bb)
            .unwrap();
    }
    /// SSA load instruction.
    #[inline]
    fn load(&self, pointee_ty: Self::Type, ptr: Self::Value, name: &str) -> Self::Value {
        self.builder
            .build_load(pointee_ty, ptr.into_pointer_value(), name)
            .unwrap()
    }
    /// SSA cast int to pointer.
    #[inline]
    fn int_to_ptr(&self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        self.builder
            .build_int_to_ptr(val.into_int_value(), dest_ty.into_pointer_type(), "")
            .unwrap()
            .into()
    }
    /// SSA bit cast.
    #[inline]
    fn bit_cast(&self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        self.builder.build_bitcast(val, dest_ty, "").unwrap()
    }
    /// SSA int cast.
    #[inline]
    fn int_cast(&self, val: Self::Value, dest_ty: Self::Type, _is_signed: bool) -> Self::Value {
        self.builder
            .build_int_cast(val.into_int_value(), dest_ty.into_int_type(), "")
            .unwrap()
            .into()
    }
    /// SSA pointer cast.
    #[inline]
    fn ptr_cast(&self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        self.builder
            .build_pointer_cast(val.into_pointer_value(), dest_ty.into_pointer_type(), "")
            .unwrap()
            .into()
    }
    /// Lookup a known function named `name`.
    fn lookup_function(&self, name: &str) -> Self::Function {
        self.module
            .get_function(name)
            .unwrap_or_else(|| panic!("known function '{name}' is not found"))
    }
}

impl<'ctx> TypeCodeGen for IR2LLVMCodeGenContext<'ctx> {}

/* Type methods */

impl<'ctx> BaseTypeMethods for IR2LLVMCodeGenContext<'ctx> {
    /// Native i8 type
    fn i8_type(&self) -> Self::Type {
        self.llvm_context.i8_type().into()
    }
    /// Native i32 type
    fn i32_type(&self) -> Self::Type {
        self.llvm_context.i32_type().into()
    }
    /// Native i64 type
    fn i64_type(&self) -> Self::Type {
        self.llvm_context.i64_type().into()
    }
    /// Native pointer type of `ty`.
    #[inline]
    fn ptr_type_to(&self, ty: Self::Type) -> Self::Type {
        self.ptr_type_to_ext(ty, crate::ir_codegen::abi::AddressSpace::DATA)
    }
    /// Native pointer type of `ty` with the address space.
    #[inline]
    fn ptr_type_to_ext(
        &self,
        ty: Self::Type,
        address_space: crate::ir_codegen::abi::AddressSpace,
    ) -> Self::Type {
        let address_space = AddressSpace::try_from(address_space.0).expect(INTERNAL_ERROR_MSG);
        let ptr_type = match ty {
            BasicTypeEnum::ArrayType(a) => a.ptr_type(address_space),
            BasicTypeEnum::FloatType(f) => f.ptr_type(address_space),
            BasicTypeEnum::IntType(i) => i.ptr_type(address_space),
            BasicTypeEnum::PointerType(p) => p.ptr_type(address_space),
            BasicTypeEnum::StructType(s) => s.ptr_type(address_space),
            BasicTypeEnum::VectorType(v) => v.ptr_type(address_space),
        };
        ptr_type.into()
    }
    /// Retrieves the bit width of the integer type `self`.
    #[inline]
    fn int_width(&self, ty: Self::Type) -> usize {
        ty.into_int_type().get_bit_width() as usize
    }
    /// Native function type
    #[inline]
    fn function_let(&self, args: &[Self::Type], ret: Self::Type) -> Self::FunctionLet {
        let args: Vec<BasicMetadataTypeEnum> = args.iter().map(|v| (*v).into()).collect();
        ret.fn_type(&args, false)
    }
    /// Native function type
    #[inline]
    fn void_function_let(&self, args: &[Self::Type]) -> Self::FunctionLet {
        let args: Vec<BasicMetadataTypeEnum> = args.iter().map(|v| (*v).into()).collect();
        self.void_type().fn_type(&args, false)
    }
}

impl<'ctx> DerivedTypeMethods for IR2LLVMCodeGenContext<'ctx> {}

impl<'ctx> CodeGenContext for IR2LLVMCodeGenContext<'ctx> {
    /// Generate LLVM IR of ast module.
    fn emit(&self, opts: &IROptions) -> Result<Vec<u8>, Box<dyn Error>> {
        self.emit_code()?;
        let code = self.link_code(opts)?;
        if cfg!(debug_assertions) {
            std::fs::write("./a.out.wasm", &code)?;
        }
        Ok(code)
    }
}

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    /// New aa IR2LLVMCodeGenContext using the LLVM Context and AST Program
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ir_context: &'ctx IRContext,
        llvm_context: &'ctx Context,
        module: Module<'ctx>,
        opts: &'ctx IROptions,
        all_abi_names: RefCell<Vec<String>>,
    ) -> IR2LLVMCodeGenContext<'ctx> {
        IR2LLVMCodeGenContext {
            ir_context,
            llvm_context,
            module,
            builder: llvm_context.create_builder(),
            opts,
            global_strings: RefCell::new(IndexMap::default()),
            global_bytes: RefCell::new(IndexMap::default()),
            class_generator: RefCell::new(ClassGenerator::new(ir_context)),
            const_storage_path_generator: RefCell::new(ConstStoragePathGenerator::new()),
            // all_runtime_classes_bytes: RefCell::new(Vec::new()),
            // type_runtime_classes: RefCell::new(IndexMap::default()),
            all_abi_names,
            imported: RefCell::new(HashSet::default()),
            pkg_scopes: RefCell::new(HashMap::new()),
            label_target: RefCell::new(vec![]),
            functions: RefCell::new(Vec::new()),
            current_runtime_ctx: RefCell::new(RuntimeContext::default()),
            current_function: RefCell::new(None),
            current_module: RefCell::new(None),
            current_contract: RefCell::new(None),
            ir_modules: ir_context
                .modules
                .borrow()
                .iter()
                .map(|(_, m)| m.clone())
                .collect(),
            bb_map: RefCell::new(IndexMap::new()),
            func_definitions: RefCell::new(Vec::new()),
            intrinsics: RefCell::new(HashMap::new()),
        }
    }

    /// Init data stream api functions.
    fn init_encoding_stdlibs(&self) {
        /* Data stream */

        // (value, bytes, offset) -> len
        self.module.add_function(
            &ParamType::Bool.get_encode_func_name(),
            self.function_let(
                &[
                    self.llvm_type(&Type::bool()),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            &ParamType::U8.get_encode_func_name(),
            self.function_let(
                &[
                    self.llvm_type(&Type::u8()),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );

        // (value*, bytes, offset) -> offset
        self.module.add_function(
            &ParamType::Bool.get_decode_func_name(),
            self.function_let(
                &[
                    self.ptr_type_to(self.llvm_type(&Type::u8())),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            &ParamType::U8.get_decode_func_name(),
            self.function_let(
                &[
                    self.ptr_type_to(self.llvm_type(&Type::u8())),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            &ParamType::U64.get_decode_func_name(),
            self.function_let(
                &[
                    self.ptr_type_to(self.llvm_type(&Type::u64())),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            &ParamType::U128.get_decode_func_name(),
            self.function_let(
                &[
                    self.ptr_type_to(self.llvm_type(&Type::u128())),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            &ParamType::Str.get_decode_func_name(),
            self.function_let(
                &[self.vec_ptr_type(), self.i8_ptr_type(), self.i32_type()],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );

        /* SSZ */

        // (value, bytes, offset) -> offset
        self.module.add_function(
            "ssz_encode_bool",
            self.function_let(
                &[
                    self.llvm_type(&Type::bool()),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            "ssz_encode_u8",
            self.function_let(
                &[
                    self.llvm_type(&Type::u8()),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            "ssz_encode_u64",
            self.function_let(
                &[
                    self.llvm_type(&Type::u64()),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            "ssz_encode_u128",
            self.function_let(
                &[
                    self.llvm_type(&Type::u128()),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );

        // (value*, bytes, offset) -> len
        self.module.add_function(
            "ssz_decode_bool",
            self.function_let(
                &[
                    self.ptr_type_to(self.llvm_type(&Type::u8())),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            "ssz_decode_u64",
            self.function_let(
                &[
                    self.ptr_type_to(self.llvm_type(&Type::u64())),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
        self.module.add_function(
            "ssz_decode_u128",
            self.function_let(
                &[
                    self.ptr_type_to(self.llvm_type(&Type::u128())),
                    self.i8_ptr_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "ssz_encode_str",
            self.function_let(
                &[
                    self.vec_ptr_type(),
                    self.i8_ptr_type(),
                    self.i32_type(),
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );
    }

    /// Init data stream api functions.
    fn init_host_apis(&self) {
        if has_extend_context() {
            get_extend_context().init_extend_host_apis(self);
        }

        /*
        void write_object(const uint8_t** immut_comps,
            uint32_t immut_comps_d1_length,
            const uint32_t* immut_comps_d2_length,
            const uint8_t** mut_comps,
            uint32_t mut_comps_d1_length,
            const uint32_t* mut_comps_d2_length,
            const uint8_t* value,
            const uint8_t* value,
            uint32_t value_length);
        */
        self.module.add_function(
            HostAPI::WriteObject.name(),
            self.void_function_let(&[
                // immutable_keys
                self.ptr_type_to(self.i8_ptr_type()),
                self.i32_type(),
                self.i32_ptr_type(),
                // mutable_keys
                self.ptr_type_to(self.i8_ptr_type()),
                self.i32_type(),
                self.i32_ptr_type(),
                // value and value length
                self.i8_ptr_type(),
                self.i32_type(),
            ]),
            Some(Linkage::External),
        );
        /*
        void read_object(const uint8_t** immut_comps,
            uint32_t immut_comps_d1_length,
            const uint32_t* immut_comps_d2_length,
            const uint8_t** mut_comps,
            uint32_t mut_comps_d1_length,
            const uint32_t* mut_comps_d2_length,
            uint8_t* value);
         */
        self.module.add_function(
            HostAPI::ReadObject.name(),
            self.void_function_let(&[
                // immutable_keys
                self.ptr_type_to(self.i8_ptr_type()),
                self.i32_type(),
                self.i32_ptr_type(),
                // mutable_keys
                self.ptr_type_to(self.i8_ptr_type()),
                self.i32_type(),
                self.i32_ptr_type(),
                // value
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );
        /*
        void delete_object(const uint8_t** immut_comps,
            uint32_t immut_comps_d1_length,
            const uint32_t* immut_comps_d2_length,
            const uint8_t** mut_comps,
            uint32_t mut_comps_d1_length,
            const uint32_t* mut_comps_d2_length,);
         */
        self.module.add_function(
            HostAPI::DeleteObject.name(),
            self.void_function_let(&[
                // immutable_keys
                self.ptr_type_to(self.i8_ptr_type()),
                self.i32_type(),
                self.i32_ptr_type(),
                // mutable_keys
                self.ptr_type_to(self.i8_ptr_type()),
                self.i32_type(),
                self.i32_ptr_type(),
            ]),
            Some(Linkage::External),
        );
        /*
        int32_t read_object_length(const uint8_t** immut_comps,
            uint32_t immut_comps_d1_length,
            const uint32_t* immut_comps_d2_length,
            const uint8_t** mut_comps,
            uint32_t mut_comps_d1_length,
            const uint32_t* mut_comps_d2_length,
        );
        */
        self.module.add_function(
            HostAPI::ReadObjectLength.name(),
            self.function_let(
                &[
                    // immutable_keys
                    self.ptr_type_to(self.i8_ptr_type()),
                    self.i32_type(),
                    self.i32_ptr_type(),
                    // mutable_keys
                    self.ptr_type_to(self.i8_ptr_type()),
                    self.i32_type(),
                    self.i32_ptr_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );

        /*
        void get_call_argpack(char * data);
        */
        self.module.add_function(
            HostAPI::GetCallArgPack.name(),
            self.void_function_let(&[
                // data u8 ptr.
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        uint32_t get_call_argpack_length()
        */
        self.module.add_function(
            HostAPI::GetCallArgPackLength.name(),
            self.function_let(&[], self.i32_type()),
            Some(Linkage::External),
        );

        /*
        void get_call_sender(char * data);
        */
        self.module.add_function(
            HostAPI::GetCallSender.name(),
            self.void_function_let(&[
                // data u8 ptr.
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        uint32_t get_call_sender_length()
        */
        self.module.add_function(
            HostAPI::GetCallSenderLength.name(),
            self.function_let(&[], self.i32_type()),
            Some(Linkage::External),
        );

        /*
        uint32_t get_call_contract_length()
        */
        self.module.add_function(
            HostAPI::GetCallContractLength.name(),
            self.function_let(&[], self.i32_type()),
            Some(Linkage::External),
        );

        /*
        void get_call_contract(char * data);
        */
        self.module.add_function(
            HostAPI::GetCallContract.name(),
            self.void_function_let(&[
                // data u8 ptr.
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        uint64_t get_call_gas_left();
        */
        self.module.add_function(
            HostAPI::GetCallGasLeft.name(),
            self.function_let(&[], self.i64_type()),
            Some(Linkage::External),
        );

        /*
        uint64_t get_call_gas_limit();
        */
        self.module.add_function(
            HostAPI::GetCallGasLimit.name(),
            self.function_let(&[], self.i64_type()),
            Some(Linkage::External),
        );

        /*
        uint32_t get_op_contract_length()
        */
        self.module.add_function(
            HostAPI::GetOpContractLength.name(),
            self.function_let(&[], self.i32_type()),
            Some(Linkage::External),
        );

        /*
        void get_op_contract(char * data);
        */
        self.module.add_function(
            HostAPI::GetOpContract.name(),
            self.void_function_let(&[
                // data u8 ptr.
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        void set_call_result(const uint8_t* data, uint32_t length);
        */
        self.module.add_function(
            HostAPI::SetCallResult.name(),
            self.void_function_let(&[
                // data u8 ptr.
                self.i8_ptr_type(),
                // data length.
                self.i32_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        uint64_t GetBlockNumber();
        */
        self.module.add_function(
            HostAPI::GetBlockNumber.name(),
            self.function_let(&[], self.i64_type()),
            Some(Linkage::External),
        );

        /*
        uint64_t get_block_timestamp();
        */
        self.module.add_function(
            HostAPI::GetBlockTimestamp.name(),
            self.function_let(&[], self.i64_type()),
            Some(Linkage::External),
        );

        /*
        void get_block_random_seed(uint8_t* data);
        */
        self.module.add_function(
            HostAPI::GetBlockRandomSeed.name(),
            self.void_function_let(&[self.i8_ptr_type()]),
            Some(Linkage::External),
        );

        /*
        uint64_t get_tx_timestamp();
        */
        self.module.add_function(
            HostAPI::GetTxTimestamp.name(),
            self.function_let(&[], self.i64_type()),
            Some(Linkage::External),
        );

        /*
        uint64_t get_tx_nonce();
        */
        self.module.add_function(
            HostAPI::GetTxNonce.name(),
            self.function_let(&[], self.i64_type()),
            Some(Linkage::External),
        );

        /*
        uint32_t get_tx_index();
        */
        self.module.add_function(
            HostAPI::GetTxIndex.name(),
            self.function_let(&[], self.i32_type()),
            Some(Linkage::External),
        );

        /*
        void get_tx_hash(char *hash32);
        */
        self.module.add_function(
            HostAPI::GetTxHash.name(),
            self.void_function_let(&[
                // data u8 ptr.
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        uint32_t get_tx_hash_length()
        */
        self.module.add_function(
            HostAPI::GetTxHashLength.name(),
            self.function_let(&[], self.i32_type()),
            Some(Linkage::External),
        );

        /*
        void get_call_sender(char * data);
        */
        self.module.add_function(
            HostAPI::GetTxSender.name(),
            self.void_function_let(&[
                // data u8 ptr.
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        uint32_t get_call_sender_length()
        */
        self.module.add_function(
            HostAPI::GetTxSenderLength.name(),
            self.function_let(&[], self.i32_type()),
            Some(Linkage::External),
        );

        /*
        uint64_t get_tx_gas_limit()
        */
        self.module.add_function(
            HostAPI::GetTxGasLimit.name(),
            self.function_let(&[], self.i64_type()),
            Some(Linkage::External),
        );

        /*
        void abort(char * msg, uint32_t msg_length);
        */
        self.module.add_function(
            HostAPI::Abort.name(),
            self.void_function_let(&[
                // msg u8 ptr.
                self.i8_ptr_type(),
                // msg length.
                self.i32_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        void println(char * msg, uint32_t msg_length);
        */
        self.module.add_function(
            HostAPI::Println.name(),
            self.void_function_let(&[
                // msg u8 ptr.
                self.i8_ptr_type(),
                // msg length.
                self.i32_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        void log(const char** topics, uint32_t topics_d1_length, const uint32_t* topics_length, const char* desc, uint32_t desc_length);
        */
        self.module.add_function(
            HostAPI::Log.name(),
            self.void_function_let(&[
                // topics
                self.ptr_type_to(self.i8_ptr_type()),
                // topics_d1_length
                self.i32_type(),
                // topics_length
                self.i32_ptr_type(),
                // desc
                self.i8_ptr_type(),
                // desc_length
                self.i32_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        void sha256(const char* msg, uint32_t msg_length, char* value);
        */
        self.module.add_function(
            HostAPI::SHA256.name(),
            self.void_function_let(&[
                // msg
                self.i8_ptr_type(),
                // msg_length
                self.i32_type(),
                // value
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        void sm3(const char* msg, uint32_t msg_length, char* value);
        */
        self.module.add_function(
            HostAPI::SM3.name(),
            self.void_function_let(&[
                // msg
                self.i8_ptr_type(),
                // msg_length
                self.i32_type(),
                // value
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );
        /*
        void keccak(const char* msg, uint32_t msg_length, char* value);
        */
        self.module.add_function(
            HostAPI::KECCAK256.name(),
            self.void_function_let(&[
                // msg
                self.i8_ptr_type(),
                // msg_length
                self.i32_type(),
                // value
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        uint32_t eth_secp256k1_recovery(const char* hash,
            const char* v,
            const char* r,
            const char* s,
            char* addr);
        */
        self.module.add_function(
            HostAPI::EthSecp256k1Recovery.name(),
            self.function_let(
                &[
                    // hash
                    self.i8_ptr_type(),
                    // v
                    self.i8_ptr_type(),
                    // r
                    self.i8_ptr_type(),
                    // s
                    self.i8_ptr_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );

        /*
        int32_t co_call(
             const char *contract,
             uint32_t contract_length,
             const char *method,
             uint32_t method_length,
             const char *argpack,
             uint32_t argpack_length);
        */
        self.module.add_function(
            HostAPI::CoCall.name(),
            self.function_let(
                &[
                    // contract
                    self.i8_ptr_type(),
                    // contract_length
                    self.i32_type(),
                    // method
                    self.i8_ptr_type(),
                    // method_length
                    self.i32_type(),
                    // argpack
                    self.i8_ptr_type(),
                    // argpack_length
                    self.i32_type(),
                ],
                self.i32_type(),
            ),
            Some(Linkage::External),
        );

        /*
        void revert(int32_t error_code, const char* error_msg, uint32_t error_msg_len);
        */
        self.module.add_function(
            HostAPI::Revert.name(),
            self.void_function_let(&[
                self.i32_type(),    // error code
                self.i8_ptr_type(), // error_msg
                self.i32_type(),    // error_msg_len
            ]),
            Some(Linkage::External),
        );

        /*
        void get_call_result(char *result);
        */
        self.module.add_function(
            HostAPI::GetCallResult.name(),
            self.void_function_let(&[
                // result
                self.i8_ptr_type(),
            ]),
            Some(Linkage::External),
        );

        /*
        int32_t get_call_result_length();
        */
        self.module.add_function(
            HostAPI::GetCallResultLength.name(),
            self.function_let(&[], self.i32_type()),
            Some(Linkage::External),
        );
    }

    /// Init runtime api functions.
    fn init_runtime_apis(&self) {}

    #[allow(dead_code)]
    fn run_llvm_passes(&self) {
        let opt_level = self.opts.opt_level.into();
        let pass_manager_builder = PassManagerBuilder::create();
        pass_manager_builder.set_optimization_level(opt_level);
        let mpm = PassManager::create(());
        pass_manager_builder.populate_module_pass_manager(&mpm);
        if !self.opts.no_inline {
            pass_manager_builder.set_inliner_with_threshold(512);
            mpm.add_always_inliner_pass();
        }

        mpm.add_loop_unroll_and_jam_pass();
        mpm.add_loop_unroll_pass();
        assert!(mpm.run_on(&self.module));
    }

    fn alloca_vars(&self, function: &FunctionDefinition, ll_func: FunctionValue<'ctx>) {
        let mut vars_list = IndexMap::new();
        for (i, param) in ll_func.get_params().iter().enumerate() {
            let ty = function.params.get(i).expect(INTERNAL_ERROR_MSG);
            let name = i.to_string();
            let ty = self.llvm_type(ty);
            let ptr = self.builder.build_alloca(ty, "").unwrap();
            self.builder.build_store(ptr, *param).unwrap();
            self.add_variable(&name, ptr, ty);
            vars_list.insert(name, ty);
        }
    }

    /// Generate LLVM IR of ast module.
    pub fn emit_code(self: &IR2LLVMCodeGenContext<'ctx>) -> Result<String, Box<dyn Error>> {
        self.init_encoding_stdlibs();
        self.init_host_apis();
        self.init_runtime_apis();
        self.init_scope("__main__");
        // WASM entry function with the memory heap init.

        // for module in self.program.pkgs.get(ast::MAIN_PKG).unwrap() {
        //     self.compile_module_import(module);
        // }

        for ir_module in &self.ir_modules {
            // scan all module's func defintions to use in other modules
            for (_, function) in &ir_module.functions {
                self.func_definitions.borrow_mut().push(function.clone());
            }
            if let Some(contract) = ir_module.contract.as_ref() {
                for (_, function) in &contract.functions {
                    self.func_definitions.borrow_mut().push(function.clone());
                }
            }
        }

        for ir_module in &self.ir_modules {
            {
                let mut cur_module = self.current_module.borrow_mut();
                *cur_module = Some(ir_module.clone());
                let mut cur_contract = self.current_contract.borrow_mut();
                *cur_contract = ir_module.contract.clone();
            }
            self.build_functions(&ir_module.name, &ir_module.functions);

            if let Some(contract) = ir_module.contract.as_ref() {
                self.build_functions(&ir_module.name, &contract.functions);
                if self.in_main_module() {
                    for (name, function) in &contract.functions {
                        {
                            let mut cur_function = self.current_function.borrow_mut();
                            *cur_function = Some(function.clone());
                        }
                        // Generate contract dispatch interface function.

                        // Dispatch contract methods have no any parameters and returns.
                        if function.is_external {
                            let internal_function_name = self.get_internal_function_name(
                                name,
                                &function.params,
                                &function.ret,
                                // this is internal func, not abi func
                                false,
                            );
                            let name = name.split('.').last().unwrap();
                            // the abi entry function. no-params and no-returns. it decode args from hostapi, call internal func, then set_call_result
                            let ll_func = self.module.get_function(name).unwrap();
                            let block = self.llvm_context.append_basic_block(ll_func, ENTRY_NAME);
                            self.push_function(ll_func);
                            self.enter_scope();
                            self.builder.position_at_end(block);
                            let data_stream_bytes_len =
                                self.build_call(HostAPI::GetCallArgPackLength.name(), &[]);
                            let data_stream_bytes =
                                self.build_call(MALLOC_FUNC_NAME, &[data_stream_bytes_len]);
                            self.build_void_call(
                                HostAPI::GetCallArgPack.name(),
                                &[data_stream_bytes],
                            );

                            // Check the data stream first version byte.
                            let (ver_ptr, _) = self.data_stream_decode(
                                &Type::i8(),
                                data_stream_bytes,
                                self.i32_value(0),
                                data_stream_bytes_len,
                                "",
                            );

                            let ver = self
                                .builder
                                .build_load(self.i8_type(), ver_ptr, "")
                                .unwrap();

                            let is_valid_ver = self
                                .builder
                                .build_int_compare(
                                    IntPredicate::EQ,
                                    ver.into_int_value(),
                                    self.i8_value(DEFAULT_VERSION as u64).into_int_value(),
                                    "",
                                )
                                .unwrap();

                            let then_block = self.append_block("datastream_decode_arg_value_block");
                            let else_block = self.append_block("datastream_version_error_block");
                            let end_block = self.append_block("");

                            self.walk_cmp_body(is_valid_ver.into(), then_block, else_block);

                            self.builder.position_at_end(then_block);
                            let mut offset = self.i32_value(1);

                            for (name, ty) in function.params.iter().enumerate() {
                                let name = name.to_string();
                                let (ptr, next_offset) = self.data_stream_decode(
                                    ty,
                                    data_stream_bytes,
                                    offset,
                                    data_stream_bytes_len,
                                    &name,
                                );
                                offset = next_offset;
                                let llvm_ty = self.llvm_type(ty);
                                if ty.is_reference_type() || ty.is_string() {
                                    let ptr_to_value =
                                        self.builder.build_alloca(llvm_ty, "").unwrap();
                                    self.builder.build_store(ptr_to_value, ptr).unwrap();
                                    self.add_variable(&name, ptr_to_value, llvm_ty);
                                } else {
                                    self.add_variable(&name, ptr, llvm_ty);
                                }
                            }
                            //check the end offset
                            self.build_void_call(
                                "check_end_offset",
                                &[offset, data_stream_bytes_len],
                            );
                            let args: Vec<BasicValueEnum> = function
                                .params
                                .iter()
                                .enumerate()
                                .map(|(name, _)| {
                                    self.get_variable(&name.to_string())
                                        .expect(INTERNAL_ERROR_MSG)
                                })
                                .collect();

                            if function.ret.is_void() {
                                self.build_void_call(&internal_function_name, &args);
                                let length = self.i32_value(0);
                                let data = self.builder.build_alloca(self.i8_type(), "").unwrap();
                                self.build_void_call(
                                    HostAPI::SetCallResult.name(),
                                    &[data.into(), length],
                                );
                                if self.opts.coverage {
                                    self.build_call_mycov_call_dump_log(
                                        &[self.i32_value(1)],
                                        &Type::void(),
                                    );
                                }
                            } else {
                                let value = self.build_call(&internal_function_name, &args);

                                if self.opts.coverage {
                                    self.build_call_mycov_call_dump_log(
                                        &[self.i32_value(1)],
                                        &Type::void(),
                                    );
                                }

                                let expr_ty = function.ret.clone();
                                let (bytes, offset) =
                                    self.data_stream_encode(&[expr_ty], &[value], false);
                                self.build_void_call(
                                    HostAPI::SetCallResult.name(),
                                    &[bytes, offset],
                                );
                            }
                            self.br(end_block);

                            self.builder.position_at_end(else_block);
                            let (msg, length) = self.str_value(
                                "Datastream decode version error: the version must be 0x00",
                            );
                            self.build_void_call("abort", &[msg.into(), length]);
                            self.br(end_block);

                            self.builder.position_at_end(end_block);
                            // Dispatch contract methods have no any parameters and returns.
                            self.ret_void();

                            self.leave_scope();
                        }
                    }
                }
            }
        }
        // constant bytes of each type object will be set after all functions compiled. We need to put the constant bytes global const variable to llvm ir.
        self.finalize_runtime_classes();
        // constant bytes of const storage path data.
        self.finalize_const_storage_path();
        // must to be created after finalize_runtime_classes. Because the. start function need some constants values when codegen.
        self.create_start_function();

        // create an empty _start function to let chain gas-meter to use
        self.create_empty_function();

        // Run LLVM pass on the LLVM module.
        self.run_llvm_passes();
        if self.opts.verbose {
            self.module.print_to_file("./a.out.ll").unwrap();
            // this need llc installed for user
            if self.opts.use_llvm_toolchain {
                let ret = std::process::Command::new("llc")
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .args(["a.out.ll"])
                    .output()
                    .expect("llc failed");
                if !ret.status.success() {
                    FatalError.raise();
                }
            }
        }

        Ok(self.module.print_to_string().to_string())
    }
    fn in_main_module(self: &IR2LLVMCodeGenContext<'ctx>) -> bool {
        self.current_module.borrow().as_ref().unwrap().name == self.ir_context.main_module
    }

    fn build_functions(
        self: &IR2LLVMCodeGenContext<'ctx>,
        _module_name: &str,
        functions: &IndexMap<String, FunctionDefinition>,
    ) {
        // First scan to store all function definition for order independent function call
        for (name, function) in functions {
            // Generate contract internal function.
            let internal_function_name = self.get_internal_function_name(
                name,
                &function.params,
                &function.ret,
                // this is internal func, not abi func
                false,
            );
            let ty = self.llvm_function_type(function);
            self.module.add_function(&internal_function_name, ty, None);
            let ty = self.void_function_let(&[]);
            if self.in_main_module() {
                let name = name.split('.').last().unwrap();
                self.module.add_function(name, ty, None);
            }
        }

        // Second scan to load and build function definition
        for (name, function) in functions {
            // Generate contract internal function.
            // get_internal_function_name should use func qualified name with module && contract
            let internal_function_name = self.get_internal_function_name(
                name,
                &function.params,
                &function.ret,
                false, // contract.is_contract && function.is_abi_entry(),
            );

            {
                let mut cur_function = self.current_function.borrow_mut();
                *cur_function = Some(function.clone());

                // self.func_definitions.borrow_mut().push(function.clone());
            }

            let ll_func = self
                .module
                .get_function(internal_function_name.as_str())
                .unwrap();
            self.push_function(ll_func);
            let block = self.llvm_context.append_basic_block(ll_func, ENTRY_NAME);
            self.enter_scope();
            self.builder.position_at_end(block);
            self.alloca_vars(function, ll_func);
            let entry = function.cfg.entry;

            {
                let mut bb_map = self.bb_map.borrow_mut();
                for (id, _) in &function.cfg.basic_blocks {
                    let block = self.append_block("");
                    bb_map.insert(*id, block);
                }
            }

            {
                let bb_map = self.bb_map.borrow();

                let entry_bb = *bb_map.get(&entry).unwrap();
                self.br(entry_bb);
            }

            for bb in function.cfg.basic_blocks.values() {
                let _ = self.walk_bb(bb);
            }
            self.leave_scope();
        }
    }

    fn create_start_function(&self) {
        let ret = self.void_type();
        let fn_type = ret.fn_type(&[], false);
        let inner_start_function = self.module.add_function("_inner_start", fn_type, None);

        let entry = self
            .llvm_context
            .append_basic_block(inner_start_function, "entry");

        self.builder.position_at_end(entry);

        // Init runtime before heap
        self.build_void_call("init_runtime", &[]);

        // Init const storage path
        self.build_void_call("init_storage_path", &[]);

        // Init wasm heap
        self.build_void_call("__init_heap", &[]);
        self.ret_void();
    }

    fn create_empty_function(&self) {
        let ret = self.void_type();
        let fn_type = ret.fn_type(&[], false);
        let start_function = self.module.add_function("_start", fn_type, None);

        let entry = self
            .llvm_context
            .append_basic_block(start_function, "entry");

        self.builder.position_at_end(entry);
        self.ret_void();
    }

    /// LLVM Target triple
    fn llvm_target_triple(&self) -> TargetTriple {
        TargetTriple::create("wasm32-unknown-unknown-wasm")
    }

    /// LLVM Target triple
    fn llvm_features(&self) -> &'static str {
        ""
    }

    /// LLVM Target name
    fn llvm_target_name(&self) -> &'static str {
        "wasm32"
    }

    /// Compile the bin and return the code as bytes. The result is
    /// cached, since this function can be called multiple times (e.g. one for
    /// each time a bin of this type is created).
    /// Pass our module to llvm for optimization and compilation
    pub fn link_code(&self, opts: &IROptions) -> Result<Vec<u8>, String> {
        let target = inkwell::targets::Target::from_name(self.llvm_target_name()).unwrap();
        let level = opts.opt_level.into();
        let target_machine = target
            .create_target_machine(
                &self.llvm_target_triple(),
                "",
                self.llvm_features(),
                level,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        match target_machine.write_to_memory_buffer(&self.module, FileType::Object) {
            Ok(out) => {
                let slice = out.as_slice();
                let ir_module = self.current_module.borrow();
                let contract = ir_module.as_ref().unwrap().contract.as_ref().unwrap();
                let mut export_names = contract
                    .functions
                    .iter()
                    .map(|(name, _)| name.split('.').last().unwrap().to_string())
                    .collect::<Vec<String>>();
                export_names.push("_inner_start".to_string());
                let bs = link(
                    slice,
                    "",
                    &export_names,
                    crate::ir_config::Target::Wasm,
                    &opts.opt_level,
                    opts.no_contract,
                    opts,
                );

                Ok(bs)
            }
            Err(s) => Err(s.to_string()),
        }
    }

    /// Init a scope named `pkgpath` with all builtin functions
    fn init_scope(&self, pkgpath: &str) {
        {
            let mut pkg_scopes = self.pkg_scopes.borrow_mut();
            if pkg_scopes.contains_key(pkgpath) {
                return;
            }
            let scopes = vec![Rc::new(Scope {
                variables: RefCell::new(IndexMap::default()),
                variable_tys: RefCell::new(IndexMap::default()),
            })];
            pkg_scopes.insert(String::from(pkgpath), scopes);
        }
        self.enter_scope();
    }

    /// Enter scope
    pub fn enter_scope(&self) {
        let current_pkgpath = self.current_pkgpath();
        let mut pkg_scopes = self.pkg_scopes.borrow_mut();
        let msg = format!("pkgpath {current_pkgpath} is not found");
        let scopes = pkg_scopes.get_mut(&current_pkgpath).expect(&msg);
        let scope = Rc::new(Scope {
            variables: RefCell::new(IndexMap::default()),
            variable_tys: RefCell::new(IndexMap::default()),
        });
        scopes.push(scope);
    }

    /// Leave scope
    pub fn leave_scope(&self) {
        let current_pkgpath = self.current_pkgpath();
        let mut pkg_scopes = self.pkg_scopes.borrow_mut();
        let msg = format!("pkgpath {current_pkgpath} is not found");
        let scopes = pkg_scopes.get_mut(&current_pkgpath).expect(&msg);
        scopes.pop();
    }

    fn current_pkgpath(&self) -> String {
        "__main__".to_string()
    }

    /// Append a variable into the scope
    pub fn add_variable(
        &self,
        name: &str,
        pointer: PointerValue<'ctx>,
        pointee_ty: BasicTypeEnum<'ctx>,
    ) {
        let current_pkgpath = self.current_pkgpath();
        let mut pkg_scopes = self.pkg_scopes.borrow_mut();
        let msg = format!("pkgpath {current_pkgpath} is not found");
        let scopes = pkg_scopes.get_mut(&current_pkgpath).expect(&msg);
        if let Some(last) = scopes.last_mut() {
            let mut variables = last.variables.borrow_mut();
            let mut variable_tys = last.variable_tys.borrow_mut();
            if !variables.contains_key(name) {
                variables.insert(name.to_string(), pointer);
                variable_tys.insert(name.to_string(), pointee_ty);
            }
        }
    }

    /// Get the variable value named `name` from the scope, return Err when not found
    pub fn get_variable(&self, name: &str) -> CompileResult<'ctx> {
        let current_pkgpath = self.current_pkgpath();
        self.get_variable_in_pkgpath(name, &current_pkgpath)
    }

    /// Get the variable value named `name` from the scope named `pkgpath`, return Err when not found
    pub fn get_variable_in_pkgpath(&self, name: &str, pkgpath: &str) -> CompileResult<'ctx> {
        let mut result = Err(CodeGenError {
            message: format!("{name} is not defined"),
        });
        let pkg_scopes = self.pkg_scopes.borrow_mut();
        // User pkgpath
        let scopes = pkg_scopes
            .get(pkgpath)
            .unwrap_or_else(|| panic!("package {pkgpath} is not found"));
        // Scopes 0 is builtin scope, Scopes 1 is the global scope, Scopes 2~ are the local scopes

        let scopes_len = scopes.len();
        for i in 0..scopes_len {
            let index = scopes_len - i - 1;
            let variables_mut = scopes[index].variables.borrow_mut();
            let variable_tys = scopes[index].variable_tys.borrow();

            if let Some(ptr) = variables_mut.get(&name.to_string()) {
                if let Some(ty) = variable_tys.get(&name.to_string()) {
                    let value = self.builder.build_load(*ty, *ptr, name).unwrap();
                    result = Ok(value);
                    break;
                }
            }
        }
        result
    }

    pub fn get_variable_ptr(&self, name: &str) -> Option<PointerValue<'ctx>> {
        let pkgpath = self.current_pkgpath();
        let pkg_scopes = self.pkg_scopes.borrow_mut();
        // User pkgpath
        let scopes = pkg_scopes
            .get(&pkgpath)
            .unwrap_or_else(|| panic!("package {pkgpath} is not found"));
        // Scopes 0 is builtin scope, Scopes 1 is the global scope, Scopes 2~ are the local scopes
        let scopes_len = scopes.len();
        for i in 0..scopes_len {
            let index = scopes_len - i - 1;
            let variables_mut = scopes[index].variables.borrow_mut();
            if let Some(var) = variables_mut.get(&name.to_string()) {
                return Some(*var);
            }
        }

        None
    }

    pub fn get_variable_ty(&self, name: &str) -> Option<BasicTypeEnum<'ctx>> {
        let pkgpath = self.current_pkgpath();
        let pkg_scopes = self.pkg_scopes.borrow_mut();
        // User pkgpath
        let scopes = pkg_scopes
            .get(&pkgpath)
            .unwrap_or_else(|| panic!("package {pkgpath} is not found"));
        // Scopes 0 is builtin scope, Scopes 1 is the global scope, Scopes 2~ are the local scopes
        let scopes_len = scopes.len();
        for i in 0..scopes_len {
            let index = scopes_len - i - 1;
            let variable_tys = scopes[index].variable_tys.borrow();
            if let Some(var) = variable_tys.get(&name.to_string()) {
                return Some(*var);
            }
        }

        None
    }

    pub fn build_or_get_variable(&self, name: &str, var_ty: &Type) -> PointerValue<'ctx> {
        let ptr = self.get_variable_ptr(name);
        if ptr.is_none() {
            let new_ptr = self
                .builder
                .build_alloca(self.llvm_type(var_ty), name)
                .unwrap();
            self.add_variable(name, new_ptr, self.llvm_type(var_ty));
            return new_ptr;
        }

        ptr.unwrap()
    }

    pub fn walk_cmp_body(
        &self,
        cond_value: BasicValueEnum<'ctx>,
        then_block: BasicBlock<'ctx>,
        else_block: BasicBlock<'ctx>,
    ) {
        let cond_type = cond_value.get_type();
        let int_width: usize = self.int_width(cond_type);

        let int_type = self
            .llvm_context
            .custom_width_int_type((int_width as u64).try_into().unwrap());
        let mhdrut = int_type.const_int(0, false);
        if int_width == 1 {
            self.cond_br(cond_value, then_block, else_block);
        } else {
            let is_truth = self
                .builder
                .build_int_compare(
                    inkwell::IntPredicate::NE,
                    cond_value.into_int_value(),
                    mhdrut,
                    "",
                )
                .unwrap()
                .into();
            self.cond_br(is_truth, then_block, else_block);
        }
    }

    pub fn get_internal_function_name(
        &self,
        name: &str,
        params: &[Type],
        ret: &Type,
        is_abi_entry: bool,
    ) -> String {
        if is_abi_entry {
            return name.to_string();
        }
        let mut internal_function_name = format!("{CONTRACT_INTERNAL_METHOD_PREFIX}_{name}");
        for val in params {
            internal_function_name = format!("{internal_function_name}_{}", val.func_sign_ty_str());
        }
        format!("{internal_function_name}_{ret}")
    }
}

impl<'ctx> IR2LLVMCodeGenContext<'ctx> {
    /// Get compiler default ok result
    #[inline]
    pub fn ok_result(&self) -> CompileResult<'ctx> {
        let i32_type = self.llvm_context.i32_type();
        Ok(i32_type.const_int(0u64, false).into())
    }

    /// Build a void function call
    #[inline]
    pub fn build_void_call(&self, name: &str, args: &[BasicValueEnum]) {
        let args: Vec<BasicMetadataValueEnum> = args.iter().map(|v| (*v).into()).collect();
        self.builder
            .build_call(self.lookup_function(name), &args, "")
            .unwrap();
    }

    /// Build a function call with the return value
    #[inline]
    pub fn build_call(&self, name: &str, args: &[BasicValueEnum<'ctx>]) -> BasicValueEnum<'ctx> {
        let args: Vec<BasicMetadataValueEnum> = args.iter().map(|v| (*v).into()).collect();
        self.builder
            .build_call(self.lookup_function(name), &args, "")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap_or_else(|| panic!("{FUNCTION_RETURN_VALUE_NOT_FOUND_MSG}: {name}"))
    }
    pub fn integer_overflow_check(
        &self,
        left: IntValue<'ctx>,
        right: IntValue<'ctx>,
        left_ty: &Type,
        op: &BinaryOp,
    ) -> BasicValueEnum<'ctx> {
        let check_name = format!(
            "llvm.{}{}.with.overflow.i{}",
            if left_ty.is_signed_int() { "s" } else { "u" },
            op,
            left.get_type().get_bit_width(),
        );

        if self.module.get_function(&check_name).is_none() {
            let ret_ty = self.llvm_context.struct_type(
                &[
                    left.get_type().into(),
                    self.llvm_context.custom_width_int_type(1).into(),
                ],
                false,
            );
            let int_ty = self.llvm_type(left_ty);
            self.module.add_function(
                &check_name,
                ret_ty.fn_type(&[int_ty.into(), int_ty.into()], false),
                None,
            );
        }

        let op_res = self
            .build_call(&check_name, &[left.into(), right.into()])
            .into_struct_value();

        let overflow = self
            .builder
            .build_extract_value(op_res, 1, "overflow")
            .unwrap()
            .into_int_value();

        let success_block = self.append_block("success");
        let error_block = self.append_block("error");

        self.builder
            .build_conditional_branch(overflow, error_block, success_block)
            .unwrap();

        self.builder.position_at_end(error_block);
        let msg = format!("math int {op} overflow");
        self.build_void_call(
            HostAPI::Abort.name(),
            &[
                self.native_global_string(&msg, "").into(),
                self.i32_value(msg.len() as u64),
            ],
        );
        self.builder.build_unreachable().unwrap();

        self.builder.position_at_end(success_block);

        self.builder
            .build_extract_value(op_res, 0, "res")
            .unwrap()
            .into_int_value()
            .into()
    }

    /// Build qhashtbl_put call
    /// param key: i64
    /// param value: i8*, pointer to the value
    /// value_size: value size(than can copy it)
    #[inline]
    pub fn build_hashtbl_put(
        &self,
        map_ptr: PointerValue<'ctx>,
        key: IntValue<'ctx>,
        value: PointerValue<'ctx>,
        value_size: IntValue<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        self.build_call(
            "qhashtbl_put",
            &[map_ptr.into(), key.into(), value.into(), value_size.into()],
        )
    }

    #[inline]
    pub fn build_hashtbl_remove(
        &self,
        map_ptr: PointerValue<'ctx>,
        key: IntValue<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        self.build_call("qhashtbl_remove", &[map_ptr.into(), key.into()])
    }

    #[inline]
    pub fn build_hashtbl_getnext(
        &self,
        map_ptr: PointerValue<'ctx>,
        iter_obj: PointerValue<'ctx>,
        is_newmem: bool,
    ) -> BasicValueEnum<'ctx> {
        self.build_call(
            "qhashtbl_getnext",
            &[
                map_ptr.into(),
                iter_obj.into(),
                self.i1_value(u64::from(is_newmem)),
            ],
        )
    }

    #[inline]
    pub fn build_hashtbl_get(
        &self,
        map_ptr: PointerValue<'ctx>,
        key: IntValue<'ctx>,
        size_ptr: Option<PointerValue<'ctx>>,
        is_newmem: bool,
    ) -> PointerValue<'ctx> {
        self.build_call(
            "qhashtbl_get",
            &[
                map_ptr.into(),
                key.into(),
                if let Some(size_ptr) = size_ptr {
                    size_ptr.into()
                } else {
                    self.i32_ptr_type().const_zero()
                }, /* int32_t *size */
                self.i1_value(u64::from(is_newmem)), /* newmem = false */
            ],
        )
        .into_pointer_value()
    }

    #[inline]
    pub fn build_hashtbl_contains_key(
        &self,
        map_ptr: PointerValue<'ctx>,
        table_key: IntValue<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        self.build_call("qhashtbl_contains_key", &[map_ptr.into(), table_key.into()])
    }

    pub fn build_pow(
        &self,
        base: IntValue<'ctx>,
        expr: IntValue<'ctx>,
        left_ty: &Type,
    ) -> BasicValueEnum<'ctx> {
        let pow_name = format!("{BUILTIN_FUNCTION_MANGLE_PREFIX}_pow_{left_ty}");
        self.build_call(&pow_name, &[base.into(), expr.into()])
    }

    pub fn get_runtime_ctx(&self) -> BasicValueEnum<'ctx> {
        let runtime_context_type = self
            .module
            .get_struct_type(RUNTIME_CONTEXT_LLVM_TY)
            .unwrap_or_else(|| panic!("can not find struct {RUNTIME_CONTEXT_LLVM_TY}"));
        // C struct:
        // struct RuntimeContext{
        //    file_name : i8 *,
        //    line : i32,
        //    col : i32,
        // }
        let runtime_context = self.current_runtime_ctx.borrow();
        let line = self.i32_value(runtime_context.line as u64);
        let col = self.i32_value(runtime_context.col as u64);
        let file_name = self.native_global_string(&runtime_context.file_name, "");
        let runtime_context_val =
            runtime_context_type.const_named_struct(&[file_name.into(), line, col]);
        let global_runtime_context = self.module.add_global(runtime_context_type, None, "");
        global_runtime_context.set_initializer(&runtime_context_val);
        global_runtime_context.set_constant(true);
        global_runtime_context.set_unnamed_addr(true);
        global_runtime_context.as_pointer_value().into()
    }

    #[allow(dead_code)]
    pub fn update_runtime_ctx(&self, loc: Option<DebugLocation>) {
        if let Some(loc) = loc {
            let mut filename_with_null = loc.get_file();
            filename_with_null.push('\0');
            self.current_runtime_ctx.replace(RuntimeContext {
                file_name: filename_with_null,
                line: loc.get_start_line(),
                col: 0,
            });
        }
    }

    #[allow(dead_code)]
    pub fn build_log(&self, info: &str) {
        let size = self.i32_value(1);
        let length = self.i32_value(info.len() as u64);
        let val = self.build_call(
            VECTOR_NEW_FUNC_NAME,
            &[length, size, self.native_global_string(info, "").into()],
        );
        self.build_void_call("ir_builtin_print", &[val]);
    }

    pub fn memset_struct_ptr(
        &self,
        ptr: PointerValue<'ctx>,
        pointee_ty: BasicTypeEnum<'ctx>,
        val: i8,
    ) {
        let void_ptr = self
            .builder
            .build_bitcast(ptr, self.i8_ptr_type(), "")
            .unwrap();
        self.build_void_call(
            "__memset",
            &[
                void_ptr,
                self.i8_value(val as u64),
                pointee_ty
                    .size_of()
                    .unwrap()
                    .const_cast(self.i32_type().into_int_type(), false)
                    .into(),
            ],
        );
    }

    /// Get LLVM i8 zero value
    pub fn native_i8(&self, v: i8) -> IntValue<'ctx> {
        let i8_type = self.llvm_context.i8_type();
        i8_type.const_int(v as u64, false)
    }

    /// Construct a LLVM int value using i1.
    pub fn i1_value(&self, v: u64) -> BasicValueEnum<'ctx> {
        let i1_type = self.llvm_context.bool_type();
        i1_type.const_int(v, false).into()
    }

    /// Construct a LLVM int value using i8
    pub fn i8_value(&self, v: u64) -> BasicValueEnum<'ctx> {
        let i8_type = self.llvm_context.i8_type();
        i8_type.const_int(v, false).into()
    }

    /// Construct a LLVM int value using i32
    pub fn i32_value(&self, v: u64) -> BasicValueEnum<'ctx> {
        let i32_type = self.llvm_context.i32_type();
        i32_type.const_int(v, false).into()
    }

    /// Construct a LLVM str value and str length using String
    pub fn str_value(&self, s: &str) -> (PointerValue<'ctx>, BasicValueEnum<'ctx>) {
        let data_str = s.to_string().into_bytes();

        let arr_length = self.i32_value(data_str.len().try_into().unwrap());

        let arr_value = self
            .build_call(MALLOC_FUNC_NAME, &[arr_length])
            .into_pointer_value();

        for (i, item) in data_str.iter().enumerate() {
            let ch = unsafe {
                self.builder
                    .build_in_bounds_gep(
                        self.i8_type(),
                        arr_value,
                        &[self.native_i8(i.try_into().unwrap())],
                        "",
                    )
                    .unwrap()
            };
            self.builder
                .build_store(ch, self.i8_value(*item as u64))
                .unwrap();
        }

        (arr_value, arr_length)
    }

    /// write all_runtime_classes_bytes to global constant bytes
    fn finalize_runtime_classes(&self) {
        let types_global_name = self
            .class_generator
            .borrow()
            .get_ir_runtime_classes_global_name();
        let global_value_pointer = self.native_global_bytes(
            self.class_generator
                .borrow()
                .all_runtime_classes_bytes
                .borrow()
                .as_ref(),
            types_global_name.as_str(),
        );
        let global_value = global_value_pointer.const_to_int(self.llvm_context.i32_type());

        // create init_runtime function. need call it in _start func
        let init_runtime_func =
            self.module
                .add_function("init_runtime", self.void_function_let(&[]), None);
        self.push_function(init_runtime_func);
        let block = self
            .llvm_context
            .append_basic_block(init_runtime_func, ENTRY_NAME);
        self.enter_scope();
        self.builder.position_at_end(block);
        self.build_void_call(
            "ir_builtin_set_all_runtimes_classes_address",
            &[self.bit_cast(global_value.into(), self.i32_type())],
        );
        self.ret_void();
        self.leave_scope();
    }

    // write all const storage path data to global constant bytes
    fn finalize_const_storage_path(&self) {
        let (csp_list_offset, csp_list_length) = self
            .const_storage_path_generator
            .borrow_mut()
            .finalize_const_storage_path_data();
        let path_global_name = self
            .const_storage_path_generator
            .borrow()
            .get_const_storage_path_global_name();

        let path_global_value_pointer = self.native_global_bytes(
            self.const_storage_path_generator
                .borrow()
                .const_storage_path_bytes
                .borrow()
                .as_ref(),
            &path_global_name,
        );
        let pointer_value = path_global_value_pointer.const_to_int(self.llvm_context.i32_type());

        // create init_storage_path function. need call it in _start func
        let init_storage_path_func =
            self.module
                .add_function("init_storage_path", self.void_function_let(&[]), None);
        self.push_function(init_storage_path_func);
        let block = self
            .llvm_context
            .append_basic_block(init_storage_path_func, ENTRY_NAME);
        self.enter_scope();
        self.builder.position_at_end(block);
        self.build_void_call(
            "builtin_init_storage_path",
            &[
                self.bit_cast(pointer_value.into(), self.i32_type()),
                self.bit_cast(self.i32_value(csp_list_offset.into()), self.i32_type()),
                self.bit_cast(self.i32_value(csp_list_length.into()), self.i32_type()),
            ],
        );
        self.ret_void();
        self.leave_scope();
    }
}
