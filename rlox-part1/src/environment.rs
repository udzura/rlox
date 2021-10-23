use std::collections::HashMap;

use crate::errors::RuntimeError;
use crate::token::Token;
use crate::value::Value;

#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, k: impl Into<String>, v: Value) {
        self.values.insert(k.into(), v);
    }

    pub fn get(&self, name: &Token) -> Result<&Value, RuntimeError> {
        let k = &name.lexeme;
        if self.values.contains_key(k) {
            return Ok(self.values.get(k).unwrap());
        } else {
            Err(RuntimeError::raise(
                name.clone(),
                format!("Undefined variable '{}'.", k),
            ))
        }
    }
}
