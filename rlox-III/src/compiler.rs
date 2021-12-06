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
#[repr(usize)]
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

impl Precedence {
    pub fn succ(self) -> Self {
        match self {
            Self::NONE => Self::ASSIGNMENT,
            Self::ASSIGNMENT => Self::OR,
            Self::OR => Self::AND,
            Self::AND => Self::EQUALITY,
            Self::EQUALITY => Self::COMPARISON,
            Self::COMPARISON => Self::TERM,
            Self::TERM => Self::FACTOR,
            Self::FACTOR => Self::UNARY,
            Self::UNARY => Self::CALL,
            Self::CALL => Self::PRIMARY,
            Self::PRIMARY => Self::PRIMARY,
        }
    }
}

type ParseFn = fn(&mut Parser) -> ();

pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}
macro_rules! rule {
    (None, None, $precedence:ident) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::$precedence,
        }
    };
    ($prefix:ident, None, $precedence:ident) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::$precedence,
        }
    };
    (None, $infix:ident, $precedence:ident) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::$precedence,
        }
    };
    ($prefix:ident, $infix:ident, $precedence:ident) => {
        ParseRule {
            prefix: Some(parsefun::$prefix),
            infix: Some(parsefun::$infix),
            precedence: Precedence::$precedence,
        }
    };
}

/// Rules indexed by [TokenType]
const RULES: [ParseRule; 41] = [
    rule!(None, None, NONE), // UNINIT,
    // follow text
    rule!(grouping, None, NONE), // LEFT_PAREN,
    rule!(None, None, NONE),     // RIGHT_PAREN,
    rule!(None, None, NONE),     // LEFT_BRACE,
    rule!(None, None, NONE),     // RIGHT_BRACE,
    rule!(None, None, NONE),     // COMMA,
    rule!(None, None, NONE),     // DOT,
    rule!(unary, binary, TERM),  // MINUS,
    rule!(None, binary, TERM),   // PLUS,
    rule!(None, None, NONE),     // SEMICOLON,
    rule!(None, binary, FACTOR), // SLASH,
    rule!(None, binary, FACTOR), // STAR,
    rule!(None, None, NONE),     // BANG,
    rule!(None, None, NONE),     // BANG_EQUAL,
    rule!(None, None, NONE),     // EQUAL,
    rule!(None, None, NONE),     // EQUAL_EQUAL,
    rule!(None, None, NONE),     // GREATER,
    rule!(None, None, NONE),     // GREATER_EQUAL,
    rule!(None, None, NONE),     // LESS,
    rule!(None, None, NONE),     // LESS_EQUAL,
    rule!(None, None, NONE),     // IDENTIFIER,
    rule!(None, None, NONE),     // STRING,
    rule!(number, None, NONE),   // NUMBER,
    rule!(None, None, NONE),     // AND,
    rule!(None, None, NONE),     // CLASS,
    rule!(None, None, NONE),     // ELSE,
    rule!(None, None, NONE),     // FALSE,
    rule!(None, None, NONE),     // FOR,
    rule!(None, None, NONE),     // FUN,
    rule!(None, None, NONE),     // IF,
    rule!(None, None, NONE),     // NIL,
    rule!(None, None, NONE),     // OR,
    rule!(None, None, NONE),     // PRINT,
    rule!(None, None, NONE),     // RETURN,
    rule!(None, None, NONE),     // SUPER,
    rule!(None, None, NONE),     // THIS,
    rule!(None, None, NONE),     // TRUE,
    rule!(None, None, NONE),     // VAR,
    rule!(None, None, NONE),     // WHILE,
    rule!(None, None, NONE),     // ERROR,
    rule!(None, None, NONE),     // EOF,
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

    pub fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = parsefun::get_rule(self.previous.token_type).prefix;
        match prefix_rule {
            None => {
                self.error("Expect expression.");
                return;
            }
            Some(prefix_rule) => prefix_rule(self),
        }

        let precedence = precedence as usize;
        while precedence <= parsefun::get_rule(self.current.token_type).precedence as usize {
            self.advance();
            let infix_rule = parsefun::get_rule(self.previous.token_type).infix;
            match infix_rule {
                None => {
                    self.error("Expect valid rule.");
                    return;
                }
                Some(infix_rule) => infix_rule(self),
            }
        }
    }

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
    use OpCode::*;
    pub fn get_rule(operator_type: TokenType) -> &'static ParseRule {
        RULES.get(operator_type as usize).unwrap()
    }

    pub fn number(scanneer: &mut Parser) {
        let value: f64 = scanneer.previous.strref().parse().unwrap_or_else(|_| 0.0);
        scanneer.emit_constant(Value::new(value));
    }

    pub fn grouping(scanneer: &mut Parser) {
        scanneer.expression();
        scanneer.consume(RIGHT_PAREN, "Expect ')' after expression.");
    }

    pub fn unary(scanneer: &mut Parser) {
        let operator_type = scanneer.previous.token_type;
        scanneer.parse_precedence(Precedence::UNARY);

        match operator_type {
            MINUS => {
                scanneer.emit_byte(OP_NEGATE as u8);
            }
            _ => {
                return;
            }
        }
    }

    pub fn binary(scanneer: &mut Parser) {
        let operator_type = scanneer.previous.token_type;
        let rule: &ParseRule = get_rule(operator_type);
        scanneer.parse_precedence(rule.precedence.succ());

        match operator_type {
            PLUS => scanneer.emit_byte(OP_ADD as u8),
            MINUS => scanneer.emit_byte(OP_SUBTRACT as u8),
            STAR => scanneer.emit_byte(OP_MULTIPLY as u8),
            SLASH => scanneer.emit_byte(OP_DIVIDE as u8),
            op => {
                unreachable!("Maybe a bug")
            }
        }
    }
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
