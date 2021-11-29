pub mod chunk;
pub mod value;
pub mod vm;

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
