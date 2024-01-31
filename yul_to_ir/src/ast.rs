// Specification of Yul
// https://docs.soliditylang.org/en/latest/yul.html#specification-of-yul

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub code: Block,
    pub inner_segments: Vec<InnerSegment>,
}

#[derive(Debug, Clone)]
pub enum InnerSegment {
    Object(Box<Object>),
    Data(Vec<DataLiteral>),
}

#[derive(Debug, Clone)]
pub enum DataLiteral {
    HexLiteral(String),
    StringLiteral(String),
}

// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

/// Block = '{' Statements* '}'
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

/// Statements = Block | FunctionDefinition | VariableDeclaration | Assign |
/// If | Expression | Switch | Forloop | BreakContinue | Leave
#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Box<Assignment>),
    VariableDeclaration(Box<VariableDeclaration>),
    If(Box<If>),
    For(Box<For>),
    Switch(Box<Switch>),
    Leave,
    Break,
    Continue,
    Block(Box<Block>),
    FunctionDefinition(Box<FunctionDefinition>),
    FunctionCall(Box<FunctionCall>),
    IfElse(Box<IfElse>),
    Comment
}

#[derive(Debug, Clone)]
pub struct IfElse {
    pub cond: Expression,
    pub body: Block,
    pub else_body: Block,
}

/// If =  'if' Expression Block
#[derive(Debug, Clone)]
pub struct If {
    pub cond: Expression,
    pub body: Block,
}

/// Assign = Identifier '=' Expression
#[derive(Debug, Clone)]
pub struct Assignment {
    pub identifiers: Vec<Identifier>,
    pub value: Expression,
}

/// VariableDeclaration = 'let' TypedIdentifierList (':=' Expression)?
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub identifiers: Vec<TypedIdentifier>,
    pub value: Option<Expression>,
}

/// Expressions = Identifier | FunctionCall | Literal
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    FunctionCall(Box<FunctionCall>),
    Literal(Literal),
}

/// Literal = (NumberLiteral | StringLiteral | TrueLiteral | FalseLiteral ) (':' TypeName)?
/// NumberLiteral = DecimalNumber | HexNumber
#[derive(Debug, Clone)]
pub enum Literal {
    TrueLiteral(Option<Identifier>),
    FalseLiteral(Option<Identifier>),
    HexNumberLiteral(HexNumber, Option<Identifier>),
    DecimalNumberLiteral(DecimalNumber, Option<Identifier>),
    StringLiteral(StringLiteral, Option<Identifier>),
}

/// Switch = 'switch' Expression (Case+ Default? | Default)
#[derive(Debug, Clone)]
pub struct Switch {
    pub condition: Expression,
    pub opt: SwitchOptions,
}

#[derive(Debug, Clone)]
pub enum SwitchOptions {
    Cases(Vec<SwitchCase>, Option<SwitchDefault>),
    Default(SwitchDefault),
}

/// Case = 'case' Literal Block
#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub case: Literal,
    pub body: Block,
}

/// Default = 'default' Block
#[derive(Debug, Clone)]
pub struct SwitchDefault {
    pub body: Block,
}

/// ForLoop =  'for' Block Expression Block Block
#[derive(Debug, Clone)]
pub struct For {
    pub init_block: Block,
    pub condition: Expression,
    pub post_block: Block,
    pub execution_block: Block,
}

/// FunctionDefinition = 'function' Identifier '(' TypedIdentifierList? ')' ('->' TypedIdentifierList)? Block
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub params: Vec<TypedIdentifier>,
    pub body: Block,
    pub returns: Vec<TypedIdentifier>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub params: Vec<TypedIdentifier>,
    pub returns: Vec<TypedIdentifier>,
}

pub type TypedIdentifierList = Vec<TypedIdentifier>;

#[derive(Debug, Clone)]
pub struct TypedIdentifier {
    pub identifier: Identifier,
    pub type_name: Option<TypeName>,
}

#[derive(Debug, Clone)]
pub struct TypeName {
    pub type_name: Identifier,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

/// FunctionCall = Identifier '(' (Expression ( ',' Expression)* )? ')'
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub id: Identifier,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub str: String,
}

#[derive(Debug, Clone)]
pub struct DecimalNumber {
    pub dec: String,
}

#[derive(Debug, Clone)]
pub struct HexNumber {
    pub hex: String,
}
