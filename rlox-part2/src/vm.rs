use std::cell::RefCell;
use std::rc::Rc;

use crate::chunk::OpCode::*;
use crate::chunk::*;
use crate::value::Value;

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

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "trace_execution")]
            self.chunk
                .as_ref()
                .and_then(|chunk| Some(chunk.borrow().disassemble_instruction(self.ip)));

            let instruction: OpCode = self.read_byte::<OpCode>();
            match instruction {
                OP_CONSTANT => {
                    let constant = self.read_constant();
                    println!("{:?}", constant);
                }
                OP_RETURN => return Ok(()),
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
        self.chunk.as_ref().unwrap().borrow().constants[cursor]
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
