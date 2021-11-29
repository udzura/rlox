use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error as IoError;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub mod chunk;
pub mod compiler;
pub mod scanner;
pub mod value;
pub mod vm;

use vm::InterpretErrorCode;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OP_CONSTANT,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NEGATE,
    OP_RETURN,

    UNKNOWN,
}

impl From<u8> for OpCode {
    fn from(from: u8) -> Self {
        use self::OpCode::*;
        match from {
            0 => OP_CONSTANT,
            1 => OP_ADD,
            2 => OP_SUBTRACT,
            3 => OP_MULTIPLY,
            4 => OP_DIVIDE,
            5 => OP_NEGATE,
            6 => OP_RETURN,
            _ => UNKNOWN,
        }
    }
}

pub fn repl() -> Result<(), IoError> {
    let mut reader = BufReader::new(std::io::stdin());

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len == 0 {
            println!();
        }
        match vm::Vm::interpret(line) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

pub fn run_file(path: &Path) -> Result<(), IoError> {
    let mut f = File::open(path)?;
    let mut bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut bytes)?;
    let code = String::from_utf8(bytes).expect("invalid string");
    match vm::Vm::interpret(code) {
        Ok(_) => Ok(()),
        Err(e) => match e {
            InterpretErrorCode::CompileError => {
                eprintln!("{}", e);
                exit(65)
            }
            InterpretErrorCode::RuntimeError => {
                eprintln!("{}", e);
                exit(66)
            }
            _ => Ok(()),
        },
    }
}
