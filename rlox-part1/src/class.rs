#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub name: String,
}

impl Class {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
