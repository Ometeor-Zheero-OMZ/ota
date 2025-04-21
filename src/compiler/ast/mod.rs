mod expressions;
mod statements;
mod precedence;
mod types;
mod patterns;
mod declarations;
mod memory;
mod common;

pub use expressions::*;
pub use statements::*;
pub use precedence::*;
pub use types::*;
pub use patterns::*;
pub use declarations::*;
pub use memory::*;
pub use common::*;

#[derive(Default, Debug)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}