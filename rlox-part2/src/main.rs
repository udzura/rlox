extern crate rlox_part2;
use chunk::OpCode::*;
use rlox_part2::*;

fn main() {
    let mut chunk = chunk::Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OP_RETURN as u8, 123);

    chunk.disassemble("test chunk");

    return ();
}
