// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod parser;
pub mod translate;

pub mod unescape;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub defs: Box<Defs>,
}

#[derive(Debug, Clone)]
pub enum Defs {
    None,
    Some(Box<Def>, Box<Defs>),
}

#[derive(Debug, Clone)]
pub enum Def {
    TypeDef(Box<TypeDef>),
    FuncDef(Box<FuncDef>),
    ContDef(Box<ContDef>),
    MDDef(Box<MDDef>),
}

#[derive(Debug, Clone)]
pub enum TypeDef {
    Compound(String, Box<Fields>, Box<MetaDatas>),
    Alias(String, Box<Type>, Box<MetaDatas>),
}

#[derive(Debug, Clone)]
pub enum Type {
    Int(IntType),
    Bool,
    Str,
    Void,
    Map(Box<Type>, Box<Type>),
    Array(Box<Type>, Option<u128>),
    Pointer(Box<Type>),
    Named(String),
}

#[derive(Debug, Clone)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug, Clone)]
pub enum Fields {
    None,
    Some(Box<Field>, Box<Fields>),
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone)]
pub struct ContDef {
    pub name: String,
    pub states: Box<Fields>,
    pub funcs: Box<Defs>,
}

#[derive(Debug, Clone)]
pub enum FuncDefs {
    None,
    Some(Box<FuncDef>, Box<FuncDefs>),
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: String,
    pub params: Box<Params>,
    pub vis: Visibility,
    pub ret: Box<Type>,
    pub basic_blocks: Box<BasicBlocks>,
    pub metadatas: Box<MetaDatas>,
}

#[derive(Debug, Clone)]
pub enum Params {
    None,
    Some(Box<Param>, Box<Params>),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub id: u128,
    pub ty: Box<Type>,
}

#[derive(Debug, Clone)]
pub enum BasicBlocks {
    None,
    Some(Box<BasicBlock>, Box<BasicBlocks>),
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: u128,
    pub instrs: Box<Instrs>,
}

#[derive(Debug, Clone)]
pub enum Instrs {
    None,
    Some(Box<Instr>, Box<Instrs>),
}

#[derive(Debug, Clone)]
pub enum Instr {
    Simple(Box<SimpleInstr>),
    Declaration(u128, Option<Box<Expr>>, Box<Type>, Box<MetaDatas>),
    Assignment(u128, Box<Expr>, Box<MetaDatas>),
}

#[derive(Debug, Clone)]

pub enum MetaDatas {
    None,
    Some(Box<MetaData>, Box<MetaDatas>),
}

#[derive(Debug, Clone)]
pub struct MetaData {
    pub name: String,
    pub id: u128,
}

#[derive(Debug, Clone)]
pub struct MDDef {
    pub id: u128,
    pub data: Box<MDFields>,
}

#[derive(Debug, Clone)]
pub enum MDFields {
    None,
    Some(Box<MDField>, Box<MDFields>),
}

#[derive(Debug, Clone)]
pub enum MDField {
    Ident(u128),
    Int(Box<IntValue>),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct SimpleInstr {
    pub name: String,
    pub args: Box<Exprs>,
    pub ret: Box<Type>,
    pub metadatas: Box<MetaDatas>,
}

#[derive(Debug, Clone)]
pub enum Exprs {
    None,
    Some(Box<Expr>, Box<Exprs>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Label(u128),
    Type(Box<Type>),
    Instr(Box<SimpleInstr>),
    Invoke(String, Box<Exprs>),
    Ident(u128, Box<Type>),
    Bool(bool),
    Int(Box<IntValue>),
    Str(String),
}

#[derive(Debug, Clone)]
pub struct IntValue {
    pub val: u128,
    pub ty: Box<Type>,
    pub positive: bool,
}
