extern crate rlox_part2;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use chunk::OpCode::*;
use rlox_part2::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut chunk = chunk::Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OP_RETURN as u8, 123);

    // chunk.disassemble("test chunk");

    let chunk = Rc::new(RefCell::new(chunk));

    vm::Vm::interpret(chunk.clone())?;

    Ok(())
}
