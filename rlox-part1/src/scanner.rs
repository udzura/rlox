use super::errors::ScanError;
use super::token::*;

pub struct Scanner<'source> {
    pub source: &'source str,
    pub tokens: Vec<Token>,
    start: i64,
    current: i64,
    line: i64,
    current_index: usize,
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
            current_index: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<usize, ScanError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.current_index += 1;
        self.tokens.push(Token::new(
            self.current_index,
            TokenType::EOF,
            "".to_string(),
            Literal::Nil,
            self.line,
        ));
        Ok(self.tokens.len())
    }

    fn scan_token(&mut self) -> Result<(), ScanError> {
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
            '!' => {
                let tok = if self.test('=') { BANG_EQUAL } else { BANG };
                self.add_token(tok, None);
            }
            '=' => {
                let tok = if self.test('=') { EQUAL_EQUAL } else { EQUAL };
                self.add_token(tok, None);
            }
            '<' => {
                let tok = if self.test('=') { LESS_EQUAL } else { LESS };
                self.add_token(tok, None);
            }
            '>' => {
                let tok = if self.test('=') {
                    GREATER_EQUAL
                } else {
                    GREATER
                };
                self.add_token(tok, None);
            }
            '/' => {
                if self.test('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH, None);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace.
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }

            c => {
                if Self::is_digit(c) {
                    self.number()?;
                } else if Self::is_alpha(c) {
                    self.identifier()?;
                } else {
                    return Err(ScanError::raise(self.line, "", "Unexpected character."));
                }
            }
        };
        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        c
    }

    fn test(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source.chars().nth(self.current as usize).unwrap();
        if c != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current as usize).unwrap()
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= (self.source.len() as i64) {
            '\0'
        } else {
            self.source.chars().nth(self.current as usize + 1).unwrap()
        }
    }

    fn string(&mut self) -> Result<(), ScanError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScanError::raise(self.line, "", "Unterminated string."));
        }

        // The closing ".
        self.advance();

        let start = self.start as usize + 1;
        let end = self.current as usize - 1;
        let value = &self.source[start..end];
        self.add_token(TokenType::STRING, Some(Literal::Str(value.to_owned())));
        Ok(())
    }

    fn number(&mut self) -> Result<(), ScanError> {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let start = self.start as usize;
        let end = self.current as usize;
        let literal: f64 = (&self.source[start..end])
            .parse()
            .map_err(|_| ScanError::raise(self.line, "", "[BUG] invalid numeric format"))?;

        self.add_token(TokenType::NUMBER, Some(Literal::Num(literal)));
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), ScanError> {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let start = self.start as usize;
        let end = self.current as usize;
        let text = &self.source[start..end];
        let token_type = TokenType::reserved_or_ident(text);

        self.add_token(token_type, None);
        Ok(())
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.source.len() as i64)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let start = self.start as usize;
        let current = self.current as usize;
        let text = &self.source[start..current];
        let literal = match literal {
            None => Literal::Nil,
            Some(l) => l,
        };
        self.current_index += 1;
        self.tokens.push(Token::new(
            self.current_index,
            token_type,
            text.to_owned(),
            literal,
            self.line,
        ))
    }
}
