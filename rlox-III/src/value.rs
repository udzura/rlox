use std::cell::Cell;

#[derive(Clone)]
pub struct Value(pub Cell<f64>);

use std::fmt;

impl Value {
    pub fn new(v: f64) -> Self {
        Self(Cell::new(v))
    }

    pub fn map(&self, ops: impl Fn(f64) -> f64) -> Self {
        let inner = self.0.get();
        Self(Cell::new(ops(inner)))
    }

    pub fn map2(pair: (&Value, &Value), ops: impl Fn((f64, f64)) -> f64) -> Value {
        let (a, b) = pair;
        Value(Cell::new(ops((a.0.get(), b.0.get()))))
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}
