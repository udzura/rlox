use crate::errors::RuntimeBreak;
use crate::interpreter::Interpreter;
use crate::value::Value;

pub trait Callable {
    fn arity(&self) -> u8;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeBreak>;
}
