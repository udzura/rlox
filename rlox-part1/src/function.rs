use crate::callable::Callable;
use crate::environment::Environment;
use crate::errors::RuntimeBreak;
use crate::instance::Instance;
use crate::interpreter::Interpreter;
use crate::stmt::Fun;
use crate::value::Value;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    arity_nr: u8,
    native: Option<fn(&Interpreter, &[Value]) -> Value>,
    declaration: Option<Rc<Fun>>,
    closure: Option<Rc<RefCell<Environment>>>,
}

impl Function {
    pub fn new_native(
        name: impl Into<String>,
        arity_nr: u8,
        native: fn(&Interpreter, &[Value]) -> Value,
    ) -> Self {
        Self {
            name: name.into(),
            arity_nr: arity_nr,
            native: Some(native),
            declaration: None,
            closure: None,
        }
    }

    pub fn new_lox(declaration: Rc<Fun>, closure: Option<Rc<RefCell<Environment>>>) -> Self {
        let name = declaration.0.as_ref().lexeme.clone();
        let arity_nr = declaration.1.len() as u8;
        let declaration = Some(declaration);
        Self {
            name,
            arity_nr,
            native: None,
            declaration: declaration,
            closure,
        }
    }

    pub fn bind(&self, instance: Instance) -> Self {
        let mut environment = Environment::new(self.closure.as_ref().cloned());
        environment.define("this", Value::LoxInstance(instance));
        let environment = Some(Rc::new(RefCell::new(environment)));

        Self {
            name: self.name.clone(),
            arity_nr: self.arity_nr,
            native: None,
            declaration: self.declaration.as_ref().cloned(),
            closure: environment,
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

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeBreak> {
        if let Some(native) = self.native {
            Ok(native(interpreter, arguments))
        } else if let Some(declaration) = &self.declaration {
            let mut environment = Environment::new(self.closure.clone());
            for i in 0..(self.arity_nr as usize) {
                environment.define(
                    &declaration.1.get(i as usize).unwrap().lexeme,
                    arguments.get(i).unwrap().clone(),
                )
            }

            let ret = match interpreter.execute_block(&declaration.2, environment) {
                Ok(_) => Ok(Value::Nil),
                Err(RuntimeBreak::Return { value }) => Ok(value),
                Err(err) => Err(err),
            };
            ret
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
