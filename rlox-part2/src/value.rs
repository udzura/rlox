use std::rc::Rc;

#[derive(Clone)]
pub struct Value(pub Rc<f64>);

use std::fmt;

impl Value {
    pub fn new(v: f64) -> Self {
        Self(Rc::new(v))
    }

    pub fn map(&self, ops: impl Fn(f64) -> f64) -> Self {
        Self(Rc::new(ops(self.0.as_ref().clone())))
    }

    pub fn map2(pair: (&Value, &Value), ops: impl Fn((f64, f64)) -> f64) -> Value {
        let (a, b) = pair;
        Value(Rc::new(ops((a.0.as_ref().clone(), b.0.as_ref().clone()))))
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}
