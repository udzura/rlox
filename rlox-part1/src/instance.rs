use std::collections::HashMap;
use std::rc::Rc;

use crate::class::*;
use crate::errors::RuntimeBreak;
use crate::token::Token;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class: Rc<ClassCore>,
    id: u64,
}

impl Instance {
    pub fn new(class: Rc<ClassCore>) -> Self {
        let data = InstanceData::default();
        let id = class.pool.borrow().len() as u64;
        class.pool.borrow_mut().insert(id, data);

        Self { id, class }
    }

    pub fn get_class(&self) -> Rc<ClassCore> {
        self.class.clone()
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeBreak> {
        let class = self.class.clone();
        let pool = class.pool.borrow();
        let data = pool
            .get(&self.id)
            .ok_or_else(|| RuntimeBreak::raise(name.clone(), "Uninitialized instance"))?;

        data.fields.get(&name.lexeme).cloned().ok_or_else(|| {
            RuntimeBreak::raise(
                name.clone(),
                format!("Undefined property '{}'", &name.lexeme),
            )
        })
    }

    pub fn set(&self, name: &Token, value: Value) -> Result<(), RuntimeBreak> {
        let class = self.class.clone();
        let mut pool = class.pool.borrow_mut();
        let data = pool
            .get_mut(&self.id)
            .ok_or_else(|| RuntimeBreak::raise(name.clone(), "Uninitialized instance"))?;

        data.fields.insert(name.lexeme.clone(), value);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct InstanceData {
    fields: HashMap<String, Value>,
}
