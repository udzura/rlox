use super::errors::ParseError;

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    // This is "Object", so set dummy type and fix in the future
    pub literal: String,
    pub line: i64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: i64) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

use std::fmt;
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

pub struct Scanner<'source> {
    pub source: &'source str,
    pub tokens: Vec<Token>,
    start: i64,
    current: i64,
    line: i64,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        let tokens = Vec::new();
        Self {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<usize, ParseError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            "".to_string(),
            self.line,
        ));
        Ok(self.tokens.len())
    }

    fn scan_token(&mut self) -> Result<(), ParseError> {
        use TokenType::*;
        let c = self.advance();
        match c {
            '(' => {
                self.add_token(LEFT_PAREN, None);
            }
            ')' => {
                self.add_token(RIGHT_PAREN, None);
            }
            '{' => {
                self.add_token(LEFT_BRACE, None);
            }
            '}' => {
                self.add_token(RIGHT_BRACE, None);
            }
            ',' => {
                self.add_token(COMMA, None);
            }
            '.' => {
                self.add_token(DOT, None);
            }
            '-' => {
                self.add_token(MINUS, None);
            }
            '+' => {
                self.add_token(PLUS, None);
            }
            ';' => {
                self.add_token(SEMICOLON, None);
            }
            '*' => {
                self.add_token(STAR, None);
            }
            _ => {
                return Err(ParseError::raise(self.line, "Unexpected character."));
            }
        };
        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let start = self.start as usize;
        let current = self.current as usize;
        let text = &self.source[start..current];
        self.tokens.push(Token::new(
            token_type,
            text.to_owned(),
            "".to_string(),
            self.line,
        ))
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.source.len() as i64)
    }
}
