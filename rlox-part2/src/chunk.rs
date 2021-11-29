use crate::value::Value;

#[derive(Debug)]
pub struct Chunk {
    count: usize,
    #[allow(dead_code)]
    capacity: usize,
    // FIXME: Vec already has all of features above...
    pub code: Vec<u8>,
    pub lines: Vec<i32>,
    pub constants: Vec<Value>,
}

impl Chunk {
    /// Used as void initChunk(Chunk* chunk);
    pub fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
        assert_eq!(self.code.len(), self.lines.len());
        self.count = self.code.len();
    }

    pub fn add_constant(&mut self, value: f64) -> usize {
        self.constants.push(Value::new(value));
        self.constants.len() - 1
    }
}

impl Chunk {
    #[cfg(debug_assertions)]
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.count {
            offset = self.disassemble_instruction(offset);
        }
    }

    #[cfg(debug_assertions)]
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        use crate::OpCode::*;
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match instruction.into() {
            OP_CONSTANT => instructions::constant("OP_CONSTANT", self, offset),
            OP_ADD => instructions::simple("OP_ADD", offset),
            OP_SUBTRACT => instructions::simple("OP_SUBTRACT", offset),
            OP_MULTIPLY => instructions::simple("OP_MULTIPLY", offset),
            OP_DIVIDE => instructions::simple("OP_DIVIDE", offset),
            OP_NEGATE => instructions::simple("OP_NEGATE", offset),
            OP_RETURN => instructions::simple("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode: {}", &instruction);
                offset + 1
            }
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn disassemble(&self, _name: &str) {}

    #[cfg(not(debug_assertions))]
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        0
    }
}

#[cfg(debug_assertions)]
mod instructions {
    use super::*;

    pub fn simple(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    pub fn constant(name: &str, chunk: &Chunk, offset: usize) -> usize {
        let constant = chunk.code[offset + 1];
        print!("{:<16} {:4} '", name, constant);
        print!("{:?}", chunk.constants[constant as usize]);
        println!("'");
        offset + 2
    }
}
