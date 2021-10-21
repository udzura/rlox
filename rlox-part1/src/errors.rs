use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ScanError {
    line: i64,
    occurred: String,
    message: String,
}

impl ScanError {
    pub fn raise(line: i64, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
            occurred: "".to_string(),
        }
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
