// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use std::{fmt::Display, rc::Rc};

use crate::ir::interface_type::PartialFuncName;
use core::hash::Hash;
use indexmap::IndexMap;
use num_bigint::BigInt;
use smart_ir_macro::MetaDataNode;

use super::builder::{BasicBlockId, IdentifierId, MetaDataId};

pub const IR_VECTOR_ITER_TY: &str = "ir.vector.iter";
pub const IR_MAP_ITER_TY: &str = "ir.map.iter";
pub const IR_PARAMPACK_TY: &str = "ir.builtin.parampack";
pub const IR_STORAGE_PATH_TY: &str = "ir.builtin.StoragePath";

/// SIR Expression
/// Identifier refer to a variable, Identifier(0) -> %0
/// Instr directly refer to an instruction
/// Literal represents the literal quantity of different basic types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Identifier(IdentifierId),
    Instr(Box<Instr>),
    Literal(Literal),
    /// Placeholder for an expression that wasn't semantically showed in some way, No Operation
    NOP,
}

impl From<Expr> for IdentifierId {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Identifier(id) => id,
            _ => panic!("Expected Identifier"),
        }
    }
}

impl From<Instr> for Expr {
    fn from(val: Instr) -> Self {
        Expr::Instr(Box::new(val))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Str(String),
    Bool(bool),
    Int(IntLiteral),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntLiteral {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    I256(BigInt),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(BigInt),
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Str(val) => write!(f, "{val}"),
            Literal::Bool(val) => write!(f, "{val}"),
            Literal::Int(val) => match val {
                IntLiteral::I8(val) => write!(f, "{val}"),
                IntLiteral::I16(val) => write!(f, "{val}"),
                IntLiteral::I32(val) => write!(f, "{val}"),
                IntLiteral::I64(val) => write!(f, "{val}"),
                IntLiteral::I128(val) => write!(f, "{val}"),
                IntLiteral::I256(val) => write!(f, "{val}"),
                IntLiteral::U8(val) => write!(f, "{val}"),
                IntLiteral::U16(val) => write!(f, "{val}"),
                IntLiteral::U32(val) => write!(f, "{val}"),
                IntLiteral::U64(val) => write!(f, "{val}"),
                IntLiteral::U128(val) => write!(f, "{val}"),
                IntLiteral::U256(val) => write!(f, "{val}"),
            },
        }
    }
}

impl Literal {
    pub fn literal_type(&self) -> Type {
        match self {
            Self::Str(_) => Type::Primitive(PrimitiveType::Str),
            Self::Bool(_) => Type::Primitive(PrimitiveType::Bool),
            Self::Int(val) => match val {
                IntLiteral::I8(_) => Type::Primitive(PrimitiveType::Int(IntType::I8)),
                IntLiteral::I16(_) => Type::Primitive(PrimitiveType::Int(IntType::I16)),
                IntLiteral::I32(_) => Type::Primitive(PrimitiveType::Int(IntType::I32)),
                IntLiteral::I64(_) => Type::Primitive(PrimitiveType::Int(IntType::I64)),
                IntLiteral::I128(_) => Type::Primitive(PrimitiveType::Int(IntType::I128)),
                IntLiteral::I256(_) => Type::Primitive(PrimitiveType::Int(IntType::I256)),
                IntLiteral::U8(_) => Type::Primitive(PrimitiveType::Int(IntType::U8)),
                IntLiteral::U16(_) => Type::Primitive(PrimitiveType::Int(IntType::U16)),
                IntLiteral::U32(_) => Type::Primitive(PrimitiveType::Int(IntType::U32)),
                IntLiteral::U64(_) => Type::Primitive(PrimitiveType::Int(IntType::U64)),
                IntLiteral::U128(_) => Type::Primitive(PrimitiveType::Int(IntType::U128)),
                IntLiteral::U256(_) => Type::Primitive(PrimitiveType::Int(IntType::U256)),
            },
        }
    }

    pub fn get_string(&self) -> Result<String, String> {
        if let Self::Str(val) = self {
            Ok(val.clone())
        } else {
            Err(format!(
                "can't parse {} literal to str",
                self.literal_type()
            ))
        }
    }

    pub fn get_bool(&self) -> Result<bool, String> {
        if let Self::Bool(val) = self {
            Ok(*val)
        } else {
            Err(format!(
                "can't parse {} literal to bool",
                self.literal_type()
            ))
        }
    }

    pub fn get_i8(&self) -> Result<i8, String> {
        if let Self::Int(IntLiteral::I8(val)) = self {
            Ok(*val)
        } else {
            Err(format!("can't parse {} literal to i8", self.literal_type()))
        }
    }

    pub fn get_i16(&self) -> Result<i16, String> {
        if let Self::Int(IntLiteral::I16(val)) = self {
            Ok(*val)
        } else {
            Err(format!(
                "can't parse {} literal to i16",
                self.literal_type()
            ))
        }
    }

    pub fn get_i32(&self) -> Result<i32, String> {
        if let Self::Int(IntLiteral::I32(val)) = self {
            Ok(*val)
        } else {
            Err(format!(
                "can't parse {} literal to i32",
                self.literal_type()
            ))
        }
    }

    pub fn get_i64(&self) -> Result<i64, String> {
        if let Self::Int(IntLiteral::I64(val)) = self {
            Ok(*val)
        } else {
            Err(format!(
                "can't parse {} literal to i64",
                self.literal_type()
            ))
        }
    }

    pub fn get_i128(&self) -> Result<i128, String> {
        if let Self::Int(IntLiteral::I128(val)) = self {
            Ok(*val)
        } else {
            Err(format!(
                "can't parse {} literal to i128",
                self.literal_type()
            ))
        }
    }

    pub fn get_u8(&self) -> Result<u8, String> {
        if let Self::Int(IntLiteral::U8(val)) = self {
            Ok(*val)
        } else {
            Err(format!("can't parse {} literal to u8", self.literal_type()))
        }
    }

    pub fn get_u16(&self) -> Result<u16, String> {
        if let Self::Int(IntLiteral::U16(val)) = self {
            Ok(*val)
        } else {
            Err(format!(
                "can't parse {} literal to u16",
                self.literal_type()
            ))
        }
    }

    pub fn get_u32(&self) -> Result<u32, String> {
        if let Self::Int(IntLiteral::U32(val)) = self {
            Ok(*val)
        } else {
            Err(format!(
                "can't parse {} literal to u32",
                self.literal_type()
            ))
        }
    }

    pub fn get_u64(&self) -> Result<u64, String> {
        if let Self::Int(IntLiteral::U64(val)) = self {
            Ok(*val)
        } else {
            Err(format!(
                "can't parse {} literal to u64",
                self.literal_type()
            ))
        }
    }

    pub fn get_u128(&self) -> Result<u128, String> {
        if let Self::Int(IntLiteral::U128(val)) = self {
            Ok(*val)
        } else {
            Err(format!(
                "can't parse {} literal to u128",
                self.literal_type()
            ))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeDefinitionKind {
    Struct,
    Enum,
    Builtin,
    Alias,
}

#[derive(Clone, Debug, Eq, MetaDataNode)]
pub struct TypeDefinition {
    pub name: String,
    pub kind: TypeDefinitionKind,
    pub ty: Rc<Type>,
    pub metadata: IndexMap<String, MetaDataId>,
}

impl Hash for TypeDefinition {
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.hash(state);
        }
    }

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.kind.hash(state);
        self.ty.hash(state);
    }
}

impl PartialEq for TypeDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.kind == other.kind && self.ty == other.ty
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub name: String,
    pub ty: Rc<Type>,
}

impl Field {
    pub fn new(name: String, ty: Rc<Type>) -> Self {
        Field { name, ty }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Primitive(PrimitiveType),
    Map { key: Rc<Type>, value: Rc<Type> },
    Array { elem: Rc<Type>, len: Option<u32> },
    Compound(Rc<Vec<Field>>),
    Pointer(Rc<Type>),
    Def(Rc<TypeDefinition>),
    Builtin(BuiltinType),
}

impl Type {
    /// Construct a void type.
    #[inline]
    pub const fn void() -> Type {
        Type::Primitive(PrimitiveType::Void)
    }

    /// Construct a bool type.
    #[inline]
    pub const fn bool() -> Type {
        Type::Primitive(PrimitiveType::Bool)
    }
    /// Construct a string type.
    #[inline]
    pub const fn str() -> Type {
        Type::Primitive(PrimitiveType::Str)
    }
    /// Construct a u8 type.
    #[inline]
    pub const fn u8() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::U8))
    }
    /// Construct a u16 type.
    #[inline]
    pub const fn u16() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::U16))
    }
    /// Construct a u32 type.
    #[inline]
    pub const fn u32() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::U32))
    }
    /// Construct a u64 type.
    #[inline]
    pub const fn u64() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::U64))
    }
    /// Construct a u128 type.
    #[inline]
    pub const fn u128() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::U128))
    }
    /// Construct a u256 type.
    #[inline]
    pub const fn u256() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::U256))
    }

    /// Construct a i8 type.
    #[inline]
    pub const fn i8() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::I8))
    }
    /// Construct a u16 type.
    #[inline]
    pub const fn i16() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::I16))
    }
    /// Construct a i32 type.
    #[inline]
    pub const fn i32() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::I32))
    }
    /// Construct a i64 type.
    #[inline]
    pub const fn i64() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::I64))
    }
    /// Construct a i128 type.
    #[inline]
    pub const fn i128() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::I128))
    }
    /// Construct a i256 type.
    #[inline]
    pub const fn i256() -> Type {
        Type::Primitive(PrimitiveType::Int(IntType::I256))
    }

    pub const fn vec_iter() -> Type {
        Type::Builtin(BuiltinType::VectorIter)
    }

    pub const fn map_iter() -> Type {
        Type::Builtin(BuiltinType::MapIter)
    }

    pub const fn storage_path() -> Type {
        Type::Builtin(BuiltinType::StoragePath)
    }
    /// Is it an reference type (pointer, array, mapping, some builtin type).
    pub fn is_reference_type(&self) -> bool {
        matches!(
            self,
            Type::Pointer(..)
                | Type::Array { .. }
                | Type::Map { .. }
                | Type::Builtin(BuiltinType::MapIter)
                | Type::Builtin(BuiltinType::VectorIter)
                | Type::Builtin(BuiltinType::Parampack)
        )
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::Pointer(..))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Str))
    }

    pub fn is_void(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Void))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Int(..)))
    }

    pub fn is_signed_int(&self) -> bool {
        matches!(
            self,
            Type::Primitive(PrimitiveType::Int(IntType::I8))
                | Type::Primitive(PrimitiveType::Int(IntType::I16))
                | Type::Primitive(PrimitiveType::Int(IntType::I32))
                | Type::Primitive(PrimitiveType::Int(IntType::I64))
                | Type::Primitive(PrimitiveType::Int(IntType::I128))
        )
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Bool))
    }

    pub fn is_parampack(&self) -> bool {
        matches!(self, Type::Builtin(BuiltinType::Parampack))
    }

    pub fn is_storage_path(&self) -> bool {
        matches!(self, Type::Builtin(BuiltinType::StoragePath))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Type::Array { .. })
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Type::Map { .. })
    }

    pub fn func_sign_ty_str(&self) -> String {
        if let Self::Array { elem, len: _ } = self {
            format!("[{}]", elem.func_sign_ty_str())
        } else {
            self.to_string()
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Primitive(prim_ty) => match prim_ty {
                PrimitiveType::Str => write!(f, "string"),
                PrimitiveType::Bool => write!(f, "bool"),
                PrimitiveType::Void => write!(f, "void"),
                PrimitiveType::Int(int_ty) => match int_ty {
                    IntType::I8 => write!(f, "i8"),
                    IntType::I16 => write!(f, "i16"),
                    IntType::I32 => write!(f, "i32"),
                    IntType::I64 => write!(f, "i64"),
                    IntType::I128 => write!(f, "i128"),
                    IntType::I256 => write!(f, "i256"),
                    IntType::U8 => write!(f, "u8"),
                    IntType::U16 => write!(f, "u16"),
                    IntType::U32 => write!(f, "u32"),
                    IntType::U64 => write!(f, "u64"),
                    IntType::U128 => write!(f, "u128"),
                    IntType::U256 => write!(f, "u256"),
                },
            },
            Type::Map { key, value } => write!(f, "{{{key}:{value}}}"),
            Type::Array { elem, len } => match len {
                Some(len) => write!(f, "[{elem};{len}]"),
                None => write!(f, "[{elem}]"),
            },
            Type::Compound(_) => unimplemented!(),
            Type::Pointer(elem) => write!(f, "{elem}*"),
            Type::Def(def) => write!(f, "{}", def.name),
            Type::Builtin(builtin_ty) => match builtin_ty {
                BuiltinType::VectorIter => write!(f, "{IR_VECTOR_ITER_TY}"),
                BuiltinType::MapIter => write!(f, "{IR_MAP_ITER_TY}"),
                BuiltinType::Parampack => write!(f, "{IR_PARAMPACK_TY}"),
                BuiltinType::StoragePath => write!(f, "{IR_STORAGE_PATH_TY}"),
            },
        }
    }
}

pub(crate) fn parse_builtin_type(ty_name: &str) -> Option<BuiltinType> {
    match ty_name {
        IR_VECTOR_ITER_TY => Some(BuiltinType::VectorIter),
        IR_MAP_ITER_TY => Some(BuiltinType::MapIter),
        IR_PARAMPACK_TY => Some(BuiltinType::Parampack),
        IR_STORAGE_PATH_TY => Some(BuiltinType::StoragePath),
        _ => None,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    VectorIter,
    MapIter,
    Parampack,
    StoragePath,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Str,
    Bool,
    Void,
    Int(IntType),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
    I128,
    I256,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Module {
    pub name: String,
    pub types: IndexMap<String, Rc<TypeDefinition>>,
    pub functions: IndexMap<String, FunctionDefinition>,
    pub contract: Option<Contract>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]

pub struct Contract {
    pub name: String,
    pub states: IndexMap<String, Type>,
    pub functions: IndexMap<String, FunctionDefinition>,
}

#[derive(Clone, Debug, PartialEq, Eq, MetaDataNode)]
pub struct FunctionDefinition {
    pub name: String,
    pub params: Vec<Type>,
    pub vars: IndexMap<IdentifierId, Type>,
    pub ret: Type,
    pub is_external: bool,
    pub cfg: ControlFlowGraph,
    pub metadata: IndexMap<String, MetaDataId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ControlFlowGraph {
    pub entry: BasicBlockId,
    pub basic_blocks: IndexMap<BasicBlockId, BasicBlock>,
}

impl ControlFlowGraph {
    pub fn new(entry_id: BasicBlockId) -> Self {
        let mut basic_blocks = IndexMap::new();
        // build and add entry block
        let bb = BasicBlock {
            id: entry_id,
            instrs: vec![],
        };
        basic_blocks.insert(entry_id, bb);
        ControlFlowGraph {
            entry: entry_id,
            basic_blocks,
        }
    }

    pub fn append_new_block(&mut self, id: BasicBlockId) -> BasicBlock {
        let bb = BasicBlock { id, instrs: vec![] };
        self.basic_blocks.insert(id, bb.clone());
        bb
    }

    pub fn update_block(&mut self, id: BasicBlockId, block: BasicBlock) {
        self.basic_blocks.insert(id, block);
    }

    pub fn delete_block(&mut self, id: BasicBlockId) {
        self.basic_blocks.remove(&id);
    }

    pub fn get_block(&self, id: BasicBlockId) -> Option<&BasicBlock> {
        self.basic_blocks.get(&id)
    }

    pub fn get_blocks(&self) -> &IndexMap<BasicBlockId, BasicBlock> {
        &self.basic_blocks
    }

    pub fn get_entry(&self) -> BasicBlockId {
        self.entry
    }

    pub fn get_entry_block(&self) -> BasicBlock {
        self.get_block(self.entry).unwrap().clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub instrs: Vec<Instr>,
}

impl BasicBlock {
    pub fn get_id(&self) -> BasicBlockId {
        self.id
    }

    pub fn insert_instr(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }

    pub fn get_instrs(&self) -> &Vec<Instr> {
        &self.instrs
    }
}

#[derive(Clone, Debug, PartialEq, Eq, MetaDataNode)]

pub struct Instr {
    pub inner: InstrDescription,
    pub metadata: IndexMap<String, MetaDataId>,
    // pub(crate) metadata: IndexMap<String, MetaDataId>,
}

impl Instr {
    pub fn new(desc: InstrDescription) -> Self {
        Self {
            inner: desc,
            metadata: IndexMap::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstrDescription {
    Declaration {
        id: IdentifierId,
        init_val: Option<Expr>,
        ty: Type,
    },
    Assignment {
        id: IdentifierId,
        val: Expr,
    },
    Ret {
        val: Option<Expr>,
    },
    Br {
        target: IdentifierId,
    },
    BrIf {
        cond: Expr,
        then_bb: BasicBlockId,
        else_bb: BasicBlockId,
    },
    Match {
        val: Expr,
        otherwise: BasicBlockId,
        jump_table: IndexMap<u32, u32>,
    },
    Not {
        op: Expr,
    },
    BitNot {
        op: Expr,
    },
    Binary {
        op_code: BinaryOp,
        op1: Expr,
        op2: Expr,
    },
    Cmp {
        op_code: CmpOp,
        op1: Expr,
        op2: Expr,
    },
    Alloca {
        ty: Type,
    },
    Malloc {
        ty: Type,
    },
    Free {
        ptr: Expr,
    },
    GetField {
        ptr: Expr,
        field_path: Vec<u32>,
        field_ty: Type,
    },
    SetField {
        ptr: Expr,
        val: Expr,
        field_path: Vec<u32>,
    },
    GetStoragePath {
        storage_path: Vec<Expr>,
    },
    StorageLoad {
        storage_path: Expr,
        load_ty: Type,
    },
    StorageStore {
        storage_path: Expr,
        store_val: Expr,
    },
    Call {
        func_name: PartialFuncName,
        args: Vec<Expr>,
        ret_ty: Type,
    },
    IntCast {
        val: Expr,
        target_ty: Type,
    },
}

impl InstrDescription {
    pub fn declaration(id: IdentifierId, init_val: Option<Expr>, ty: Type) -> Self {
        Self::Declaration { id, init_val, ty }
    }
    pub fn assignment(id: IdentifierId, val: Expr) -> Self {
        Self::Assignment { id, val }
    }
    pub fn ret(val: Option<Expr>) -> Self {
        Self::Ret { val }
    }
    pub fn br(target: BasicBlockId) -> Self {
        Self::Br { target }
    }
    pub fn br_if(cond: Expr, then_bb: BasicBlockId, else_bb: BasicBlockId) -> Self {
        Self::BrIf {
            cond,
            then_bb,
            else_bb,
        }
    }
    pub fn r#match(val: Expr, otherwise: BasicBlockId, jump_table: IndexMap<u32, u32>) -> Self {
        Self::Match {
            val,
            otherwise,
            jump_table,
        }
    }
    pub fn not(op: Expr) -> Self {
        Self::Not { op }
    }

    pub fn bit_not(op: Expr) -> Self {
        Self::BitNot { op }
    }

    pub fn add(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Add,
            op1,
            op2,
        }
    }

    pub fn sub(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Sub,
            op1,
            op2,
        }
    }

    pub fn mul(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Mul,
            op1,
            op2,
        }
    }

    pub fn div(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Div,
            op1,
            op2,
        }
    }

    pub fn r#mod(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Mod,
            op1,
            op2,
        }
    }

    pub fn exp(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Exp,
            op1,
            op2,
        }
    }

    pub fn and(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::And,
            op1,
            op2,
        }
    }

    pub fn bit_and(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::BitAnd,
            op1,
            op2,
        }
    }

    pub fn or(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Or,
            op1,
            op2,
        }
    }

    pub fn bit_or(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::BitOr,
            op1,
            op2,
        }
    }

    pub fn bit_xor(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::BitXor,
            op1,
            op2,
        }
    }

    pub fn shl(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Shl,
            op1,
            op2,
        }
    }

    pub fn shr(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Shr,
            op1,
            op2,
        }
    }

    pub fn sar(op1: Expr, op2: Expr) -> Self {
        Self::Binary {
            op_code: BinaryOp::Sar,
            op1,
            op2,
        }
    }

    pub fn eq(op1: Expr, op2: Expr) -> Self {
        Self::Cmp {
            op_code: CmpOp::Eq,
            op1,
            op2,
        }
    }

    pub fn ne(op1: Expr, op2: Expr) -> Self {
        Self::Cmp {
            op_code: CmpOp::Ne,
            op1,
            op2,
        }
    }

    pub fn gt(op1: Expr, op2: Expr) -> Self {
        Self::Cmp {
            op_code: CmpOp::Gt,
            op1,
            op2,
        }
    }

    pub fn ge(op1: Expr, op2: Expr) -> Self {
        Self::Cmp {
            op_code: CmpOp::Ge,
            op1,
            op2,
        }
    }

    pub fn lt(op1: Expr, op2: Expr) -> Self {
        Self::Cmp {
            op_code: CmpOp::Lt,
            op1,
            op2,
        }
    }

    pub fn le(op1: Expr, op2: Expr) -> Self {
        Self::Cmp {
            op_code: CmpOp::Le,
            op1,
            op2,
        }
    }

    pub fn alloca(ty: Type) -> Self {
        Self::Alloca { ty }
    }

    pub fn malloc(ty: Type) -> Self {
        Self::Malloc { ty }
    }

    pub fn free(ptr: Expr) -> Self {
        Self::Free { ptr }
    }

    pub fn get_field(ptr: Expr, field_path: Vec<u32>, field_ty: Type) -> Self {
        Self::GetField {
            ptr,
            field_path,
            field_ty,
        }
    }

    pub fn set_field(ptr: Expr, field_path: Vec<u32>, val: Expr) -> Self {
        Self::SetField {
            ptr,
            field_path,
            val,
        }
    }

    pub fn get_storage_path(storage_path: Vec<Expr>) -> Self {
        Self::GetStoragePath { storage_path }
    }

    pub fn storage_load(storage_path: Expr, load_ty: Type) -> Self {
        Self::StorageLoad {
            storage_path,
            load_ty,
        }
    }

    pub fn storage_store(storage_path: Expr, store_val: Expr) -> Self {
        Self::StorageStore {
            storage_path,
            store_val,
        }
    }

    pub fn call(func_name: PartialFuncName, args: Vec<Expr>, ret_ty: Type) -> Self {
        Self::Call {
            func_name,
            args,
            ret_ty,
        }
    }

    pub fn int_cast(val: Expr, target_ty: Type) -> Self {
        Self::IntCast { val, target_ty }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    And,
    BitAnd,
    Or,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Sar,
}
impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "add"),
            BinaryOp::Sub => write!(f, "sub"),
            BinaryOp::Mul => write!(f, "mul"),
            BinaryOp::Div => write!(f, "div"),
            BinaryOp::Mod => write!(f, "mod"),
            BinaryOp::Exp => write!(f, "exp"),
            BinaryOp::And => write!(f, "and"),
            BinaryOp::BitAnd => write!(f, "bit_and"),
            BinaryOp::Or => write!(f, "or"),
            BinaryOp::BitOr => write!(f, "bit_or"),
            BinaryOp::BitXor => write!(f, "bit_xor"),
            BinaryOp::Shl => write!(f, "shl"),
            BinaryOp::Shr => write!(f, "shr"),
            BinaryOp::Sar => write!(f, "sar"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct MetaData {
    pub data: Vec<Literal>,
    // pub(crate) data: Vec<Literal>,
}

impl MetaData {
    pub fn push_field(&mut self, val: Literal) {
        self.data.push(val)
    }
    pub fn get_operand(&self, index: u32) -> &Literal {
        &self.data[index as usize]
    }
}

/// MDNode indicates the IR node from which metadata can be extracted
pub trait MetaDataNode {
    fn get_metadata(&self) -> &IndexMap<String, u32>;
    fn get_metadata_mut(&mut self) -> &mut IndexMap<String, u32>;
}

impl Module {
    pub fn insert_types(&mut self, type_name: &str, type_def: &TypeDefinition) {
        self.types
            .insert(type_name.to_owned(), Rc::new(type_def.clone()));
    }
}
