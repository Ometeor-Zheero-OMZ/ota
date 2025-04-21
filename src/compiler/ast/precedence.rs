#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Precedence {
    Lowest,
    Assign,         // = += -= *= /= %= |= &= ^=
    Range,          // .. ..=
    Or,             // ||
    And,            // &&
    Equals,         // == !=
    LessGreater,    // > < >= <=
    BitOr,          // |
    BitXor,         // ^
    BitAnd,         // &
    Shift,          // << >>
    Sum,            // + -
    Product,        // * / %
    Prefix,         // -X !X ~X &X *X
    Call,           // myFunction(X)
    Index,          // array[index], object.property
    Bool,           // true false
}