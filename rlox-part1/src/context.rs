use crate::token::{Token, TokenType};

#[derive(Debug, Default)]
pub struct Context {
    pub had_error: bool,
}

impl Context {
    pub fn error(&mut self, line: i64, message: impl Into<String>) {
        self.report(line, "", message);
    }

    pub fn error_on(&mut self, token: &Token, message: impl Into<String>) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, format!(" at '{}'", token.lexeme), message);
        }
    }

    pub fn report(&mut self, line: i64, occurred: impl Into<String>, message: impl Into<String>) {
        eprintln!(
            "[line: {}] Error{}: {}",
            line,
            occurred.into(),
            message.into()
        );
        self.had_error = true;
    }
}
