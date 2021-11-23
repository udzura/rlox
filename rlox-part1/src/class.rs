use std::rc::Rc;

use crate::callable::Callable;
use crate::errors::RuntimeBreak;
use crate::instance::Instance;
use crate::interpreter::Interpreter;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub name: String,
}

impl Class {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn arity(&self) -> u8 {
        0
    }

    pub fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeBreak> {
        Ok(Value::LoxInstance(Instance::new(self.clone())))
    }
}
