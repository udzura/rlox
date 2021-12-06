use crate::chunk::Chunk;
use crate::scanner::*;
use crate::value::Value;
use crate::vm::*;
use crate::OpCode;

use crate::scanner::TokenType::*;

use std::cell::Cell;
use std::cell::RefCell;
use std::mem;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precedence {
    NONE,
    ASSIGNMENT, // =
    OR,         // or
    AND,        // and
    EQUALITY,   // == !=
    COMPARISON, // < > <= >=
    TERM,       // + -
    FACTOR,     // * /
    UNARY,      // ! -
    CALL,       // . ()
    PRIMARY,
}

type ParseFn = fn(&mut Parser) -> ();

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}
macro_rules! rule {
    (None, None, $precedence:ident) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::$precedence,
        }
    };
    ($prefix:path, None, $precedence:ident) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::$precedence,
        }
    };
    (None, $infix:path, $precedence:ident) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::$precedence,
        }
    };
    ($prefix:path, $infix:path, $precedence:ident) => {
        ParseRule {
            prefix: Some($prefix),
            infix: Some($infix),
            precedence: Precedence::$precedence,
        }
    };
}

/// Rules indexed by [TokenType]
const RULES: [ParseRule; 2] = [
    rule!(None, None, NONE),                        // UNINIT
    rule!(parsefun::unary, parsefun::binary, NONE), // MINUS
];

pub struct Parser<'src, 'c> {
    pub scanner: RefCell<Scanner<'src>>,
    pub chunk: &'c mut Chunk,
    pub current: Token,
    pub previous: Token,
    pub had_error: Cell<bool>,
    pub panic_mode: Cell<bool>,
}

impl<'src, 'c> Parser<'src, 'c> {
    pub fn new(scanner: RefCell<Scanner<'src>>, chunk: &'c mut Chunk) -> Self {
        Self {
            scanner,
            chunk,
            current: Token::null(),
            previous: Token::null(),
            had_error: Cell::new(false),
            panic_mode: Cell::new(false),
        }
    }

    pub fn advance(&mut self) {
        mem::swap(&mut self.previous, &mut self.current);

        loop {
            self.current = self.scanner.borrow_mut().scan_token();
            if self.current.token_type != ERROR {
                break;
            };

            let message = self.current.getstring();
            self.error_at_current(&message);
        }
    }

    pub fn consume(&mut self, tt: TokenType, message: &str) {
        if self.current.token_type == tt {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::ASSIGNMENT);
    }

    pub fn number(&mut self) {
        let value: f64 = self.previous.strref().parse().unwrap_or_else(|_| 0.0);
        self.emit_constant(Value::new(value));
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.consume(RIGHT_PAREN, "Expect ')' after expression.");
    }

    pub fn parse_precedence(&self, precedence: Precedence) {}

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(&value);
        self.emit_bytes((OpCode::OP_CONSTANT as u8, constant));
    }

    fn make_constant(&mut self, value: &Value) -> u8 {
        let constant = self.chunk.add_constant(value.0.get());
        if constant > (u8::MAX as usize) {
            self.error("Too many constants in one chunk.");
            return 0;
        }
        constant as u8
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OP_RETURN as u8);
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.line as i32;
        self.chunk.write(byte, line);
    }

    fn emit_bytes(&mut self, byte: (u8, u8)) {
        self.emit_byte(byte.0);
        self.emit_byte(byte.1);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current, message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.previous, message);
    }

    fn error_at(&self, token: &Token, message: &str) {
        if self.panic_mode.get() {
            return;
        }
        self.panic_mode.set(true);

        eprint!("[line {}] Error", token.line);
        match token.token_type {
            EOF => {
                eprint!(" at end");
            }
            ERROR => { //skip
            }
            _ => {
                eprint!(" at '{}'", token.strref());
            }
        }
        eprintln!(": {}", message);
        self.had_error.set(true);
    }
}

mod parsefun {
    use super::*;
    pub fn unary(scanneer: &mut Parser) {
        let operator_type = scanneer.previous.token_type;
        scanneer.parse_precedence(Precedence::UNARY);

        match operator_type {
            MINUS => {
                scanneer.emit_byte(OpCode::OP_NEGATE as u8);
            }
            _ => {
                return;
            }
        }
    }

    pub fn binary(scanneer: &mut Parser) {}
}

pub fn compile(source: String, chunk: &mut Chunk) -> InterpretResult {
    let scanner = Scanner::new(&source);
    let mut parser = Parser::new(RefCell::new(scanner), chunk);

    parser.advance();
    parser.expression();
    parser.consume(EOF, "Expect end of expression.");
    parser.end_compiler();
    if !parser.had_error.get() {
        Ok(())
    } else {
        Err(InterpretErrorCode::CompileError)
    }
}
