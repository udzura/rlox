use std::rc::Rc;

use crate::class::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub class: Rc<ClassCore>,
}

impl Instance {
    pub fn new(class: Rc<ClassCore>) -> Self {
        Self { class }
    }

    pub fn get_class(&self) -> Rc<ClassCore> {
        self.class.clone()
    }
}
