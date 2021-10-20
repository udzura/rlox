#[derive(Debug)]
struct ParseError {
    line: i64,
    occurred: String,
    message: String,
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
