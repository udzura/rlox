#[derive(Debug)]
pub struct Scanner<'src> {
    source: &'src str,
    start: usize,
    current: usize,
    line: i64,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance();
        if is_digit(c) {
            return self.number();
        }
        if is_alpha(c) {
            return self.identifier();
        }

        match c {
            '(' => {
                return self.make_token(TokenType::LEFT_PAREN);
            }
            ')' => {
                return self.make_token(TokenType::RIGHT_PAREN);
            }
            '{' => {
                return self.make_token(TokenType::LEFT_BRACE);
            }
            '}' => {
                return self.make_token(TokenType::RIGHT_BRACE);
            }
            ';' => {
                return self.make_token(TokenType::SEMICOLON);
            }
            ',' => {
                return self.make_token(TokenType::COMMA);
            }
            '.' => {
                return self.make_token(TokenType::DOT);
            }
            '-' => {
                return self.make_token(TokenType::MINUS);
            }
            '+' => {
                return self.make_token(TokenType::PLUS);
            }
            '/' => {
                return self.make_token(TokenType::SLASH);
            }
            '*' => {
                return self.make_token(TokenType::STAR);
            }

            '!' => {
                let tt = if self.matches('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                return self.make_token(tt);
            }
            '=' => {
                let tt = if self.matches('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                return self.make_token(tt);
            }
            '<' => {
                let tt = if self.matches('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                return self.make_token(tt);
            }
            '>' => {
                let tt = if self.matches('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                return self.make_token(tt);
            }
            '"' => {
                return self.string();
            }

            _ => {}
        }

        self.error("Unexpected character.")
    }

    fn is_at_end(&mut self) -> bool {
        self.current == self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars()[self.current - 1]
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        self.chars()[self.current]
    }

    fn peeknext(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.chars()[self.current + 1]
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                return;
            }
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peeknext() == '/' {
                        // A comment goes until the end of the line.
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    };
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn check_keyword(
        &mut self,
        start: usize,
        length: usize,
        rest: &'static str,
        tt: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length
            && &self.source[(self.start + start)..(self.start + start + length)] == rest
        {
            tt
        } else {
            TokenType::IDENTIFIER
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error("Unterminated string.");
        }
        self.advance();

        self.make_token(TokenType::STRING)
    }

    fn number(&mut self) -> Token {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peeknext()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::NUMBER)
    }

    fn identifier(&mut self) -> Token {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let tt = self.identifier_type();
        self.make_token(tt)
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.chars()[self.start] {
            'a' => {
                return self.check_keyword(1, 2, "nd", TokenType::AND);
            }
            'c' => {
                return self.check_keyword(1, 4, "lass", TokenType::CLASS);
            }
            'e' => {
                return self.check_keyword(1, 3, "lse", TokenType::ELSE);
            }
            'f' => {
                if self.current - self.start > 1 {
                    match self.chars()[self.start + 1] {
                        'a' => {
                            return self.check_keyword(2, 3, "lse", TokenType::FALSE);
                        }
                        'o' => {
                            return self.check_keyword(1, 4, "r", TokenType::FOR);
                        }
                        'u' => {
                            return self.check_keyword(1, 3, "n", TokenType::FUN);
                        }
                        _ => {}
                    }
                }
            }
            'i' => {
                return self.check_keyword(1, 1, "f", TokenType::IF);
            }
            'n' => {
                return self.check_keyword(1, 2, "il", TokenType::NIL);
            }
            'o' => {
                return self.check_keyword(1, 1, "r", TokenType::OR);
            }
            'p' => {
                return self.check_keyword(1, 4, "rint", TokenType::PRINT);
            }
            'r' => {
                return self.check_keyword(1, 5, "eturn", TokenType::RETURN);
            }
            's' => {
                return self.check_keyword(1, 4, "uper", TokenType::SUPER);
            }
            't' => {
                if self.current - self.start > 1 {
                    match self.chars()[self.start + 1] {
                        'h' => {
                            return self.check_keyword(2, 3, "is", TokenType::THIS);
                        }
                        'r' => {
                            return self.check_keyword(1, 4, "ue", TokenType::TRUE);
                        }
                        _ => {}
                    }
                }
            }
            'v' => {
                return self.check_keyword(1, 2, "ar", TokenType::VAR);
            }
            'w' => {
                return self.check_keyword(1, 4, "hile", TokenType::WHILE);
            }
            _ => {}
        }

        TokenType::IDENTIFIER
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let string = self.source[self.start..self.current].to_owned();
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            string,
        }
    }

    pub fn error(&mut self, message: &'static str) -> Token {
        Token {
            token_type: TokenType::ERROR,
            start: 0,
            length: message.len(),
            line: self.line,
            string: message.to_string(),
        }
    }

    fn chars(&self) -> Vec<char> {
        self.source.chars().collect::<Vec<char>>()
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub string: String,
    pub line: i64,
}

impl Token {
    pub fn null() -> Self {
        Self {
            token_type: TokenType::UNINIT,
            start: 0,
            length: 0,
            string: "".to_string(),
            line: 0,
        }
    }

    pub fn strref(&self) -> &str {
        &self.string
    }

    pub fn getstring(&self) -> String {
        self.string.clone()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// #[expr(usize)]
pub enum TokenType {
    UNINIT,

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
    FOR,
    FUN,
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

    ERROR,
    EOF,
}
