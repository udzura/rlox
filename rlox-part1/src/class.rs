use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::callable::Callable;
use crate::errors::RuntimeBreak;
use crate::instance::*;
use crate::interpreter::Interpreter;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    core: Rc<ClassCore>,
}

impl Class {
    pub fn new(name: impl Into<String>, methods: HashMap<String, Value>) -> Self {
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
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: &[Value],
    ) -> Result<Value, RuntimeBreak> {
        Ok(Value::LoxInstance(Instance::new(self.core.clone())))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClassCore {
    pub name: String,
    pub methods: HashMap<String, Value>,
    pub pool: RefCell<HashMap<u64, InstanceData>>,
}

impl ClassCore {
    pub fn find_method(&self, key: &String) -> Option<&Value> {
        self.methods.get(key)
    }
}
