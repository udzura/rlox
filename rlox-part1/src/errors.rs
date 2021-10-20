#[derive(Debug)]
pub struct ParseError {
    line: i64,
    occurred: String,
    message: String,
}

impl ParseError {
    pub fn raise(line: i64, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
            occurred: "".to_string(),
        }
    }
}

use std::fmt;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[line: {}] Error{}: {}",
            self.line, self.occurred, self.message
        )
    }
}

impl std::error::Error for ParseError {}
