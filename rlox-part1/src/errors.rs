use super::token::{Token, TokenType};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ScanError {
    line: i64,
    occurred: String,
    message: String,
}

impl ScanError {
    pub fn raise(line: i64, occurred: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            line,
            occurred: occurred.into(),
            message: message.into(),
        }
    }

    pub fn report(token: &Token, message: impl Into<String>) {
        let err = if token.token_type == TokenType::EOF {
            Self::raise(token.line, " at end", message)
        } else {
            Self::raise(token.line, format!(" at '{}'", token.lexeme), message)
        };
        println!("{}", &err);
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[line: {}] Error{}: {}",
            self.line, self.occurred, self.message
        )
    }
}

impl Error for ScanError {}

#[derive(Debug)]
pub struct ParseError {}

impl ParseError {
    pub fn raise() -> Self {
        Self {}
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError.")
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn raise(token: Token, message: impl Into<String>) -> Self {
        let message = message.into();
        Self { token, message }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RuntimeError: {}\n[line {}]",
            self.message, self.token.line
        )
    }
}

impl Error for RuntimeError {}
