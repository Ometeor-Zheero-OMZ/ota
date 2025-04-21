use super::expressions::Expression;
use super::statements::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Box<Statement>,
}

impl MatchArm {
    pub fn new(pattern: Pattern, guard: Option<Expression>, body: Box<Statement>) -> Self {
        Self { pattern, guard, body }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Literal(Expression),
    Identifier(String),
    Wildcard,
    Destructure(DestructurePattern),
    Range(Box<Expression>, Box<Expression>),
    Or(Vec<Pattern>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct DestructurePattern {
    pub kind: DestructureKind,
    pub fields: Vec<(String, Option<Pattern>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DestructureKind {
    Struct(String),
    Tuple,
    Enum(String, String),
}