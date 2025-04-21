use super::expressions::Expression;
use super::types::Type;

#[derive(Debug, PartialEq, Clone)]
pub struct AllocateExpression {
    pub type_: Type,
    pub size: Option<Box<Expression>>,
}

impl AllocateExpression {
    pub fn new(type_: Type, size: Option<Box<Expression>>) -> Self {
        Self { type_, size }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CastExpression {
    pub expression: Box<Expression>,
    pub target_type: Type,
}

impl CastExpression {
    pub fn new(expression: Box<Expression>, target_type: Type) -> Self {
        Self { expression, target_type }
    }
}