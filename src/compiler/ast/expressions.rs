use super::statements::BlockStatement;
use super::types::Type;
use super::common::Lifetime;
use super::memory::{AllocateExpression, CastExpression};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    String(String),
    Object(ObjectExpression),
    Array(ArrayExpression),
    Null,
    Self_,
    Identifier(String),
    Unary(UnaryExpression),
    Update(UpdateExpression),
    Binary(BinaryExpression),
    Function(FunctionExpression),
    Call(CallExpression),
    Member(MemberExpression),
    RawPointer(Box<Expression>),
    AddressOf(Box<Expression>),
    Cast(CastExpression),
    Allocate(AllocateExpression),
    Deallocate(Box<Expression>),
    NullPointer(Option<Type>),
    Dereference(Box<Expression>),
    SizeOf(Box<Expression>),
    TypeOf(Box<Expression>),
    TypeCheck(Box<Expression>, Type),
    
    // Error handling expressions
    Try(Box<Expression>),
    Ok(Box<Expression>),
    Err(Box<Expression>),
    Some(Box<Expression>),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub operator: String,
    pub right: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(operator: String, right: Box<Expression>) -> Self {
        Self { operator, right }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateExpression {
    pub operator: String,
    pub target_var_name: String,
}

impl UpdateExpression {
    pub fn new(operator: String, target_var_name: String) -> Self {
        Self { operator, target_var_name }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    ) -> Self {
        Self { left, operator, right }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionExpression {
    pub parameters: Vec<FunctionParameter>,
    pub body: BlockStatement,
    pub lifetimes: Vec<Lifetime>,
}

impl FunctionExpression {
    pub fn new(parameters: Vec<FunctionParameter>, body: BlockStatement, lifetimes: Vec<Lifetime>) -> Self {
        Self { parameters, body, lifetimes }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub default: Option<Expression>,
}

impl FunctionParameter {
    pub fn new(name: String, default: Option<Expression>) -> Self {
        Self { name, default }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl CallExpression {
    pub fn new(callee: Box<Expression>, arguments: Vec<Expression>) -> Self {
        Self { callee, arguments }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpression {
    pub properties: Vec<ObjectProperty>,
}

impl ObjectExpression {
    pub fn new(properties: Vec<ObjectProperty>) -> Self {
        Self { properties }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectProperty {
    pub key: String,
    pub val: Expression,
}

impl ObjectProperty {
    pub fn new(key: String, val: Expression) -> Self {
        Self { key, val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
}

impl MemberExpression {
    pub fn new(object: Box<Expression>, property: Box<Expression>) -> Self {
        Self { object, property }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayExpression {
    pub elements: Vec<Expression>,
}

impl ArrayExpression {
    pub fn new(elements: Vec<Expression>) -> Self {
        ArrayExpression { elements }
    }
}