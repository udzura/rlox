use crate::chunk::Chunk;
use crate::scanner::*;
use crate::vm::*;

use crate::scanner::TokenType::*;

use std::cell::RefCell;
use std::mem;

struct Parser<'src> {
    scanner: RefCell<Scanner<'src>>,
    current: Token,
    previous: Token,
    had_error: bool,
}

impl<'src> Parser<'src> {
    pub fn new(scanner: RefCell<Scanner<'src>>) -> Self {
        Self {
            scanner,
            current: Token::null(),
            previous: Token::null(),
            had_error: false,
        }
    }

    pub fn advance(&mut self) {
        mem::swap(&mut self.previous, &mut self.current);

        loop {
            self.current = self.scanner.borrow_mut().scan_token();
            if self.current.token_type != ERROR {
                break;
            };

            let message = self.current.getstr();
            self.error_at_current(message);
        }
    }

    fn error_at_current(&mut self, message: String) {
        self.had_error = true;
        self.error_at(&self.current, message);
    }

    fn error(&mut self, message: String) {
        self.had_error = true;
        self.error_at(&self.previous, message);
    }

    fn error_at(&self, token: &Token, message: String) {
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
    }
}

pub fn compile(source: String, chunk: &Chunk) -> InterpretResult {
    let scanner = Scanner::new(&source);
    let mut parser = Parser::new(RefCell::new(scanner));

    parser.advance();
    //parser.expression();
    //parser.consume(EOF, "Expect end of expression.");
    if !parser.had_error {
        Ok(())
    } else {
        Err(InterpretErrorCode::CompileError)
    }
}
