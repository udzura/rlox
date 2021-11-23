use std::rc::Rc;

use crate::class::Class;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub class: Rc<Class>,
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        Self { class }
    }

    pub fn get_class(&self) -> Rc<Class> {
        self.class.clone()
    }
}
