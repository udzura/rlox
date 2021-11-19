use crate::interpreter::Interpreter;
use crate::value::Value;

pub trait Callable {
    fn arity() -> u8;
    fn call(interpreter: Interpreter, arguments: Vec<Value>) -> Value;
}
