use super::common::Lifetime;

#[derive(Debug, PartialEq, Clone)]
pub enum Mutability {
    Const,
    Mut,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    // Primitive types
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Bool,
    Char,
    String,

    // Function types
    Function(Vec<Type>, Box<Type>), // fn(T, U) -> V

    // Struct types
    Struct(String),

    // Enum types
    Enum(String),

    // Complex types
    Array(Box<Type>, Option<usize>),
    Slice(Box<Type>),

    // Pointer types
    RawPointer(Box<Type>, Mutability), // *const T or *mut T
    Reference(Box<Type>, Mutability),  // &T or &mut T
    ReferenceWithLifetime(Box<Type>, Mutability, Vec<Lifetime>), // &'a T or &'a mut T

    // User-defined types
    Named(String),

    Result(Box<Type>, Box<Type>),
    Option(Box<Type>),

    Unit, // ()
}