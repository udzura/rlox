use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::RuntimeBreak;
use crate::token::Token;
use crate::value::Value;

#[derive(Debug, Default)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Value>,
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

    pub fn assign(&mut self, name: &Token, v: Value) -> Result<(), RuntimeBreak> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), v);
            return Ok(());
        } else {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow_mut().assign(name, v);
            }

            Err(RuntimeBreak::raise(
                name.clone(),
                format!("Undefined variable '{}'.", &name.lexeme),
            ))
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeBreak> {
        let k = &name.lexeme;
        if self.values.contains_key(k) {
            return Ok(self.values.get(k).unwrap().clone());
        } else {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow().get(name);
            }

            Err(RuntimeBreak::raise(
                name.clone(),
                format!("Undefined variable '{}'.", k),
            ))
        }
    }

    pub fn take_enclosing(&self) -> Option<Environment> {
        match &self.enclosing {
            Some(environment) => Some(environment.take()),
            None => None,
        }
    }

    pub fn get_at(
        environment: Rc<RefCell<Self>>,
        distance: usize,
        name: &Token,
    ) -> Result<Value, RuntimeBreak> {
        unsafe { &*Self::ancestor(environment, distance) }.get(name)
    }

    pub fn assign_at(
        environment: Rc<RefCell<Self>>,
        distance: usize,
        name: &Token,
        value: Value,
    ) -> Result<(), RuntimeBreak> {
        unsafe { &mut *Self::ancestor(environment, distance) }.assign(name, value)
    }

    fn ancestor(environment: Rc<RefCell<Self>>, distance: usize) -> *mut Environment {
        let mut environment = environment.as_ptr();
        for _ in 0..distance {
            environment = unsafe { &*environment }
                .enclosing
                .as_ref()
                .expect("Too much distance")
                .as_ptr();
        }
        environment
    }
}
