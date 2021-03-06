use crate::class::Class;
use crate::function::Function;
use crate::instance::Instance;
use crate::token::Literal;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Value {
    // Object(Box<dyn Any>),
    Nil,
    Boolean(bool),
    Number(f64),
    LoxString(String),
    LoxFunction(Function),
    LoxClass(Class),
    LoxInstance(Instance),
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Nil => write!(f, "nil"),
            Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Number(n) => write!(f, "{}", n),
            LoxString(s) => write!(f, "{}", s),
            LoxFunction(fun) => write!(f, "#<Function: {}>", &fun.name),
            LoxClass(class) => write!(f, "#<Class: {}>", &class.name()),
            LoxInstance(instance) => write!(f, "#<Instance of {}>", &instance.get_class().name),
        }
    }
}
