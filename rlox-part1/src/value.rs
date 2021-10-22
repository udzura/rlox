use std::any::Any;

use crate::token::Literal;

#[derive(Debug)]
pub enum Value {
    Object(Box<dyn Any>),
    Nil,
    Boolean(bool),
    Number(f64),
    LoxString(String),
}

impl From<&Literal> for Value {
    fn from(from: &Literal) -> Self {
        from.value()
    }
}

impl Value {
    pub fn map_number<F>(self, f: F) -> Option<Self>
    where
        F: Fn(f64) -> f64,
    {
        if let Self::Number(n) = self {
            let n = f(n);
            Some(Self::Number(n))
        } else {
            None
        }
    }
}
