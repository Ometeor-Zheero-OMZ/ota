use crate::compiler::ast::Precedence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenType {
    // Special tokens
    Illegal,
    Eof,

    // Identifier and literal tokens
    Ident(String),
    IntLiteral(String),
    FloatLiteral(String),
    CharLiteral(String),
    StringLiteral(String),
    
    // Operator tokens
    Assign,        // =
    Plus,          // +
    Minus,         // -
    Asterisk,      // *
    Slash,         // /
    Percent,       // %
    BitOr,         // |
    BitAnd,        // &
    BitXor,        // ^
    BitNot,        // ~
    
    // Compound assignment operators
    PlusAssign,    // +=
    MinusAssign,   // -=
    MulAssign,     // *=
    DivAssign,     // /=
    ModAssign,     // %=
    BitOrAssign,   // |=
    BitAndAssign,  // &=
    BitXorAssign,  // ^=
    
    // Comparison operators
    Eq,            // ==
    NotEq,         // !=
    Lt,            // 
    Gt,            // >
    LtEq,          // <=
    GtEq,          // >=
    
    // Logical operators
    And,           // &&
    Or,            // ||
    Not,           // !
    
    // Bitwise shift operators
    LeftShift,     // 
    RightShift,    // >>
    
    // Unary operators
    Ampersand,     // &
    Dereference,   // *
    Arrow,         // ->
    
    // Parentheses and brackets
    LParen,        // (
    RParen,        // )
    LBrace,        // {
    RBrace,        // }
    LBracket,      // [
    RBracket,      // ]
    
    // Delimiters
    Semicolon,     // ;
    Colon,         // :
    DoubleColon,   // ::
    Comma,         // ,
    Dot,           // .
    
    // Special tokens like Rust
    RangeExclusive, // ..
    RangeInclusive, // ..=
    FatArrow,       // =>
    ThinArrow,      // ->
    Question,       // ?
    
    // Common keywords
    Let,
    Const,
    If,
    Else,
    While,
    For,
    Loop,
    Break,
    Continue,
    Return,
    
    // Function and type related keywords
    Fn,
    Struct,
    Enum,
    Trait,
    Impl,
    Mod,
    Use,
    Pub,
    Self_,      // self
    SelfType,   // Self
    
    // Module related keywords
    As,
    Type,
    Where,
    
    // Memory management keywords
    Unsafe,
    Mut,
    Static,
    Extern,
    Sizeof,
    
    // Pattern matching keywords
    Match,
    
    // Boolean literals
    True,
    False,
    Null,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal,
            line,
            column,
        }
    }
    
    pub fn get_precedence(&self) -> Precedence {
        match &self.token_type {
            TokenType::Assign | TokenType::PlusAssign | TokenType::MinusAssign
            | TokenType::MulAssign | TokenType::DivAssign | TokenType::ModAssign
            | TokenType::BitOrAssign | TokenType::BitAndAssign | TokenType::BitXorAssign => Precedence::Assign,
            
            TokenType::Or => Precedence::Bool,
            TokenType::And => Precedence::Bool,
            
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            
            TokenType::Lt | TokenType::Gt | TokenType::LtEq | TokenType::GtEq => Precedence::LessGreater,
            
            TokenType::LeftShift | TokenType::RightShift => Precedence::Shift,
            
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            
            TokenType::Asterisk | TokenType::Slash | TokenType::Percent => Precedence::Product,
            
            TokenType::LParen => Precedence::Call,
            TokenType::LBracket => Precedence::Index,
            TokenType::Dot => Precedence::Index,
            
            _ => Precedence::Lowest,
        }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "let" => TokenType::Let,
        "const" => TokenType::Const,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "while" => TokenType::While,
        "for" => TokenType::For,
        "loop" => TokenType::Loop,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "return" => TokenType::Return,
        "fn" => TokenType::Fn,
        "struct" => TokenType::Struct,
        "enum" => TokenType::Enum,
        "trait" => TokenType::Trait,
        "impl" => TokenType::Impl,
        "mod" => TokenType::Mod,
        "use" => TokenType::Use,
        "pub" => TokenType::Pub,
        "self" => TokenType::Self_,
        "Self" => TokenType::SelfType,
        "as" => TokenType::As,
        "type" => TokenType::Type,
        "where" => TokenType::Where,
        "unsafe" => TokenType::Unsafe,
        "mut" => TokenType::Mut,
        "static" => TokenType::Static,
        "extern" => TokenType::Extern,
        "sizeof" => TokenType::Sizeof,
        "match" => TokenType::Match,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "null" => TokenType::Null,
        _ => TokenType::Ident(ident.to_string()),
    }
}