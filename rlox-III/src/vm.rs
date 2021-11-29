use std::cell::RefCell;
use std::rc::Rc;

use crate::chunk::*;
use crate::compiler;
use crate::value::Value;
use crate::OpCode;
use crate::OpCode::*;

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub struct Vm {
    chunk: Option<Rc<RefCell<Chunk>>>,
    ip: usize,
    stack: Vec<Value>,
    stack_top: usize,
}

static mut VM: Vm = Vm {
    chunk: None,
    ip: 0,
    stack: Vec::new(),
    stack_top: 0,
};

macro_rules! binary_op {
    ($self:ident, $op:tt) => {
        let b = $self.pop();
        let a = $self.pop();
        let oped = Value::map2((&a, &b), |(a, b)| a $op b );
        $self.push(&oped);
    };
}

impl Vm {
    pub fn init_vm() {
        unsafe {
            VM.stack = Vec::with_capacity(STACK_MAX);
            VM.stack_top = 0;
        };
    }

    pub fn interpret(source: String) -> InterpretResult {
        compiler::compile(source)?;
        Ok(())
    }

    pub fn error(reason: InterpretErrorCode) -> InterpretResult {
        Err(reason)
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "trace_execution")]
            unsafe {
                print!("          ");
                print!("{:?}", &VM.stack);
                println!();
                self.chunk
                    .as_ref()
                    .and_then(|chunk| Some(chunk.borrow().disassemble_instruction(self.ip)));
            }

            let instruction: OpCode = self.read_byte::<OpCode>();
            match instruction {
                OP_CONSTANT => {
                    let constant = self.read_constant();
                    self.push(&constant);
                }
                OP_ADD => {
                    binary_op!(self, +);
                }
                OP_SUBTRACT => {
                    binary_op!(self, -);
                }
                OP_MULTIPLY => {
                    binary_op!(self, *);
                }
                OP_DIVIDE => {
                    binary_op!(self, /);
                }
                OP_NEGATE => {
                    let neg = self.pop().map(|f| -f);
                    self.push(&neg);
                }
                OP_RETURN => {
                    println!("{:?}", self.pop());
                    return Ok(());
                }
                _ => {
                    break;
                }
            };
        }
        return Ok(());
    }

    fn read_byte<T>(&mut self) -> T
    where
        T: From<u8>,
    {
        let ret = self.chunk.as_ref().unwrap().borrow().code[self.ip];
        self.ip += 1;
        ret.into()
    }

    fn read_constant(&mut self) -> Value {
        let cursor: usize = self.read_byte::<usize>();
        self.chunk.as_ref().unwrap().borrow().constants[cursor].clone()
    }

    #[allow(dead_code)]
    fn reset_stack(&mut self) {
        self.stack.clear();
        self.stack_top = 0;
    }

    fn push(&mut self, value: &Value) {
        self.stack.insert(self.stack_top, value.clone());
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack.remove(self.stack_top)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpretErrorCode {
    Ok, // stub
    CompileError,
    RuntimeError,
}

use std::error::Error;
use std::fmt;

impl fmt::Display for InterpretErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InterpretErrorCode {}

pub type InterpretResult = Result<(), InterpretErrorCode>;
