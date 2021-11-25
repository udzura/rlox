use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::callable::Callable;
use crate::errors::RuntimeBreak;
use crate::function::Function;
use crate::instance::*;
use crate::interpreter::Interpreter;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    core: Rc<ClassCore>,
}

impl Class {
    pub fn new(name: impl Into<String>, methods: HashMap<String, Function>) -> Self {
        Self {
            core: Rc::new(ClassCore {
                name: name.into(),
                methods,
                ..Default::default()
            }),
        }
    }

    pub fn name(&self) -> &str {
        &self.core.name
    }
}

impl Callable for Class {
    fn arity(&self) -> u8 {
        if let Some(initializer) = self.core.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeBreak> {
        let instance = Instance::new(self.core.clone());
        if let Some(initializer) = self.core.find_method("init") {
            initializer
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        }

        Ok(Value::LoxInstance(instance))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClassCore {
    pub name: String,
    pub methods: HashMap<String, Function>,
    pub pool: RefCell<HashMap<u64, InstanceData>>,
}

impl ClassCore {
    pub fn find_method(&self, key: &str) -> Option<&Function> {
        self.methods.get(key)
    }
}
