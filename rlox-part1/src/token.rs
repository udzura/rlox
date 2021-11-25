use std::{
    fmt,
    hash::{Hash, Hasher},
};

use crate::value::Value;

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

impl TokenType {
    pub fn reserved_or_ident(s: &str) -> TokenType {
        use TokenType::*;
        match s {
            "and" => AND,
            "class" => CLASS,
            "else" => ELSE,
            "false" => FALSE,
            "for" => FOR,
            "fun" => FUN,
            "if" => IF,
            "nil" => NIL,
            "or" => OR,
            "print" => PRINT,
            "return" => RETURN,
            "super" => SUPER,
            "this" => THIS,
            "true" => TRUE,
            "var" => VAR,
            "while" => WHILE,

            _ => IDENTIFIER,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Num(n) => write!(f, "<Num {:?}>", n),
            Literal::Str(s) => write!(f, "<Str {:?}>", s),
            Literal::Bool(b) => write!(f, "<Bool {:?}>", b),
            Literal::Nil => write!(f, "<None>"),
        }
    }
}

impl Literal {
    pub fn value(&self) -> Value {
        match self {
            Literal::Num(n) => Value::Number(*n),
            Literal::Str(s) => Value::LoxString(s.to_owned()),
            Literal::Bool(b) => Value::Boolean(*b),
            Literal::Nil => Value::Nil,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    index: usize,
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: i64,
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&self.index, &self.token_type, &self.lexeme, &self.line).hash(state);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        (&self.index, &self.token_type, &self.lexeme, &self.line)
            == (&other.index, &other.token_type, &other.lexeme, &other.line)
    }
}
impl Eq for Token {}

impl Token {
    pub fn new(
        index: usize,
        token_type: TokenType,
        lexeme: impl Into<String>,
        literal: Literal,
        line: i64,
    ) -> Self {
        let lexeme = lexeme.into();
        Self {
            index,
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn this() -> Self {
        Self::new(0, TokenType::THIS, "this", Literal::Nil, 0)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}){:?} {} {}",
            self.index, self.token_type, self.lexeme, self.literal
        )
    }
}
