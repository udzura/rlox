use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::RuntimeError;
use crate::token::Token;
use crate::value::Value;

#[derive(Debug, Default)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, k: impl Into<String>, v: Value) {
        self.values.insert(k.into(), v);
    }

    pub fn assign(&mut self, name: &Token, v: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), v);
            return Ok(());
        } else {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow_mut().assign(name, v);
            }

            Err(RuntimeError::raise(
                name.clone(),
                format!("Undefined variable '{}'.", &name.lexeme),
            ))
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        let k = &name.lexeme;
        if self.values.contains_key(k) {
            return Ok(self.values.get(k).unwrap().clone());
        } else {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow().get(name);
            }

            Err(RuntimeError::raise(
                name.clone(),
                format!("Undefined variable '{}'.", k),
            ))
        }
    }
}
