use super::expressions::Expression;
use super::patterns::MatchArm;
use super::{EnumDeclaration, ModuleDeclaration, StructDeclaration, TraitDeclaration, TraitImplementation, UseStatement};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Return(Expression),
    Expression(Expression),
    If(IfStatement),
    Block(BlockStatement),
    For(ForStatement),
    While(WhileStatement),
    Continue,
    Break,
    Unsafe(BlockStatement),
    Match(MatchStatement),
    StructDecl(StructDeclaration),
    EnumDecl(EnumDeclaration),
    TraitDecl(TraitDeclaration),
    ImplTrait(TraitImplementation),
    ModuleDecl(ModuleDeclaration),
    Use(UseStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ForInit {
    Statement(Box<Statement>),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetStatement {
    pub name: String,
    pub val: Expression,
}

impl LetStatement {
    pub fn new(name: String, val: Expression) -> Self {
        Self { name, val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstStatement {
    pub name: String,
    pub val: Expression,
}

impl ConstStatement {
    pub fn new(name: String, val: Expression) -> Self {
        Self { name, val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statement: Vec<Statement>,
}

impl BlockStatement {
    pub fn new(statement: Vec<Statement>) -> Self {
        Self { statement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub test: Expression,
    pub consequence: Box<Statement>,
    pub alternate: Box<Option<Statement>>,
}

impl IfStatement {
    pub fn new(
        test: Expression,
        consequence: Box<Statement>,
        alternate: Box<Option<Statement>>,
    ) -> Self {
        Self {
            test,
            consequence,
            alternate,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStatement {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

impl SwitchStatement {
    pub fn new(discriminant: Expression, cases: Vec<SwitchCase>) -> Self {
        Self {discriminant, cases }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}

impl SwitchCase {
    pub fn new(test: Option<Expression>, consequent: Vec<Statement>) -> Self {
        SwitchCase { test, consequent }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    pub init: Option<ForInit>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}

impl ForStatement {
    pub fn new(
        init: Option<ForInit>,
        test: Option<Expression>,
        update: Option<Expression>,
        body: Box<Statement>,
    ) -> Self {
        Self { init, test, update, body }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}

impl WhileStatement {
    pub fn new(condition: Expression, body: Box<Statement>) -> Self {
        Self { condition, body }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchStatement {
    pub value: Expression,
    pub arms: Vec<MatchArm>,
}

impl MatchStatement {
    pub fn new(value: Expression, arms: Vec<MatchArm>) -> Self {
        Self { value, arms }
    }
}