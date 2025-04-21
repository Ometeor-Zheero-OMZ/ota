pub mod ast;
// pub mod core;
pub mod lexer;

// use super::ast::Ast;

// mod lexer;
// mod parser;

// pub trait Paerser {
//     fn parse(&self, src: String) -> Ast;
// }

// pub struct BuiltinParser;

// impl Parser for BuiltinParser {
//     fn parse(&self, src: String) -> Ast {
//         let mut lexer = lexer::Lexer::new(src);
//         let mut parser = parser::Parser::new(&mut lexer);
//         parser.parse_program()
//     }
// }