use super::expressions::FunctionParameter;
use super::statements::BlockStatement;
use super::types::Type;
use super::common::Visibility;

#[derive(Debug, PartialEq, Clone)]
pub struct UseStatement {
    pub path: Vec<String>,
    pub alias: Option<String>,
}

impl UseStatement {
    pub fn new(path: Vec<String>, alias: Option<String>) -> Self {
        Self { path, alias }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleDeclaration {
    pub name: String,
    pub statements: Vec<super::statements::Statement>,
}

impl ModuleDeclaration {
    pub fn new(name: String, statements: Vec<super::statements::Statement>) -> Self {
        Self { name, statements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructDeclaration {
    pub name: String,
    pub fields: Vec<StructField>,
    pub methods: Vec<MethodDeclaration>,
}

impl StructDeclaration {
    pub fn new(name: String, fields: Vec<StructField>, methods: Vec<MethodDeclaration>) -> Self {
        Self { name, fields, methods }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructField {
    pub name: String,
    pub type_: Type,
    pub visibility: Visibility,
}

impl StructField {
    pub fn new(name: String, type_: Type, visibility: Visibility) -> Self {
        Self { name, type_, visibility }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodDeclaration {
    pub name: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Type,
    pub body: BlockStatement,
    pub visibility: Visibility,
}

impl MethodDeclaration {
    pub fn new(
        name: String,
        parameters: Vec<FunctionParameter>,
        return_type: Type,
        body: BlockStatement,
        visibility: Visibility,
    ) -> Self {
        Self { name, parameters, return_type, body, visibility }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumDeclaration {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub methods: Vec<MethodDeclaration>,
}

impl EnumDeclaration {
    pub fn new(name: String, variants: Vec<EnumVariant>, methods: Vec<MethodDeclaration>) -> Self {
        Self { name, variants, methods }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<StructField>,
}

impl EnumVariant {
    pub fn new(name: String, fields: Vec<StructField>) -> Self {
        Self { name, fields }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariantFields {
    Unnamed(Vec<Type>),      // Enum(T1, T2)
    Named(Vec<StructField>), // Enum { field1: T1, field2: T2 }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitDeclaration {
    pub name: String,
    pub method_signatures: Vec<MethodSignature>,
}

impl TraitDeclaration {
    pub fn new(name: String, method_signatures: Vec<MethodSignature>) -> Self {
        Self { name, method_signatures }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<Type>,
    pub default_impl: Option<BlockStatement>,
}

impl MethodSignature {
    pub fn new(
        name: String,
        parameters: Vec<FunctionParameter>,
        return_type: Option<Type>,
        default_impl: Option<BlockStatement>,
    ) -> Self {
        Self { name, parameters, return_type, default_impl }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitImplementation {
    pub trait_name: String,
    pub for_type: Type,
    pub methods: Vec<MethodDeclaration>,
}

impl TraitImplementation {
    pub fn new(trait_name: String, for_type: Type, methods: Vec<MethodDeclaration>) -> Self {
        Self { trait_name, for_type, methods }
    }
}