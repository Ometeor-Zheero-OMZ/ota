#[derive(Debug, PartialEq, Clone)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lifetime {
    pub name: String,
}

impl Lifetime {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}