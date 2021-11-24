use std::rc::Rc;

use crate::callable::Callable;
use crate::errors::RuntimeBreak;
use crate::instance::Instance;
use crate::interpreter::Interpreter;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    core: Rc<ClassCore>,
}

impl Class {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            core: Rc::new(ClassCore { name: name.into() }),
        }
    }

    pub fn name(&self) -> &str {
        &self.core.name
    }
}

impl Callable for Class {
    fn arity(&self) -> u8 {
        0
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeBreak> {
        Ok(Value::LoxInstance(Instance::new(self.core.clone())))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassCore {
    pub name: String,
}
