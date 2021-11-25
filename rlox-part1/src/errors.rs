use crate::value::Value;

use super::token::Token;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ScanError {}

impl ScanError {
    pub fn raise() -> Self {
        Self {}
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanError.",)
    }
}

impl Error for ScanError {}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct SemanticError {}

impl SemanticError {
    pub fn raise() -> Self {
        Self {}
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SemanticError.")
    }
}

impl Error for SemanticError {}

#[derive(Debug)]
pub enum RuntimeBreak {
    RuntimeError { token: Token, message: String },
    Return { value: Value },
}

impl RuntimeBreak {
    pub fn raise(token: Token, message: impl Into<String>) -> Self {
        let message = message.into();
        Self::RuntimeError { token, message }
    }

    pub fn ret(value: Value) -> Self {
        Self::Return { value }
    }
}

impl fmt::Display for RuntimeBreak {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeBreak::RuntimeError { message, token } => {
                write!(f, "RuntimeError: {}\n[line {}]", message, token.line)
            }
            RuntimeBreak::Return { value } => write!(f, "Return: {}", value),
        }
    }
}

impl Error for RuntimeBreak {}
