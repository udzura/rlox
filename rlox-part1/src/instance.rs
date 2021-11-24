use std::collections::HashMap;
use std::rc::Rc;

use crate::class::*;
use crate::errors::RuntimeBreak;
use crate::token::Token;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class: Rc<ClassCore>,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: Rc<ClassCore>) -> Self {
        let fields = HashMap::new();
        Self { class, fields }
    }

    pub fn get_class(&self) -> Rc<ClassCore> {
        self.class.clone()
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeBreak> {
        self.fields.get(&name.lexeme).cloned().ok_or_else(|| {
            RuntimeBreak::raise(
                name.clone(),
                format!("Undefined property '{}'.", &name.lexeme),
            )
        })
    }
}
