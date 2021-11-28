use std::cell::RefCell;
use std::rc::Rc;

use crate::chunk::OpCode::*;
use crate::chunk::*;

pub struct Vm {
    chunk: Option<Rc<RefCell<Chunk>>>,
    ip: usize,
}

static mut VM: Vm = Vm { chunk: None, ip: 0 };

impl Vm {
    pub fn init_vm() {
        // let vm = Vm { chunk: None };
        // unsafe { VM = vm };
    }

    pub fn interpret(chunk: Rc<RefCell<Chunk>>) -> InterpretResult {
        unsafe {
            VM.chunk = Some(chunk.clone());
            VM.ip = 0;

            VM.run()
        }?;
        Ok(())
    }

    pub fn error(reason: InterpretErrorCode) -> InterpretResult {
        Err(reason)
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction: OpCode = self.read_byte();
            match instruction {
                OP_RETURN => return Ok(()),
                _ => {}
            };
        }
        return Ok(());
    }

    pub fn read_byte(&mut self) -> OpCode {
        let ret = self.chunk.as_ref().unwrap().borrow().code[self.ip];
        self.ip += 1;
        ret.into()
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

type InterpretResult = Result<(), InterpretErrorCode>;
