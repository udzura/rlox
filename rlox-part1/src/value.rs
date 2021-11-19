use crate::callable::Callable;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Literal;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Value {
    // Object(Box<dyn Any>),
    Nil,
    Boolean(bool),
    Number(f64),
    LoxString(String),
    LoxFunction(Function),
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
        }
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    arity_nr: u8,
    native: Option<fn(&Interpreter, &[Value]) -> Value>,
    declaration: Option<Rc<Stmt>>,
}

impl Function {
    pub fn new_native(
        name: impl Into<String>,
        arity_nr: u8,
        native: fn(&Interpreter, &[Value]) -> Value,
    ) -> Self {
        Function {
            name: name.into(),
            arity_nr: arity_nr,
            native: Some(native),
            declaration: None,
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.name)
            .field(&self.arity_nr)
            .field(&format!("<has native fn: {}>", self.native.is_some()))
            .field(&self.declaration)
            .finish()
    }
}

impl Callable for Function {
    fn arity(&self) -> u8 {
        self.arity_nr
    }

    fn call(&self, interpreter: &Interpreter, arguments: &[Value]) -> Value {
        if self.native.is_some() {
            self.native.unwrap()(interpreter, arguments)
        } else if self.declaration.is_some() {
            todo!()
        } else {
            panic!("[BUG] invalid function decleration")
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
