extern crate rlox_part2;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use rlox_part2::*;
use OpCode::*;

fn main() -> Result<(), Box<dyn Error>> {
    vm::Vm::init_vm();

    let mut chunk = chunk::Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OP_ADD as u8, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OP_MULTIPLY as u8, 123);
    chunk.write(OP_NEGATE as u8, 123);

    chunk.write(OP_RETURN as u8, 123);

    // chunk.disassemble("test chunk");

    let chunk = Rc::new(RefCell::new(chunk));

    vm::Vm::interpret(chunk.clone())?;

    Ok(())
}
