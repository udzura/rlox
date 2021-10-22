use std::cell::RefCell;
use std::rc::Rc;

use super::errors::*;

use super::expr::Expr;
use super::token::*;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: Rc<RefCell<usize>>,
}

type ParseResult = Result<Expr, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let current = Rc::new(RefCell::new(0));
        Self { tokens, current }
    }

    pub fn parse(&self) -> Option<Expr> {
        self.expression().ok()
    }

    fn expression(&self) -> ParseResult {
        self.equality()
    }

    fn equality(&self) -> ParseResult {
        let mut expr = self.comparison()?;

        while self.matching(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::binary(expr, operator.clone(), right);
        }

        Ok(expr)
    }

    fn comparison(&self) -> ParseResult {
        let mut expr = self.term()?;

        while self.matching(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::binary(expr, operator.clone(), right);
        }

        Ok(expr)
    }

    fn term(&self) -> ParseResult {
        let mut expr = self.factor()?;

        while self.matching(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator.clone(), right);
        }

        Ok(expr)
    }

    fn factor(&self) -> ParseResult {
        let mut expr = self.unary()?;

        while self.matching(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::binary(expr, operator.clone(), right);
        }

        Ok(expr)
    }

    fn unary(&self) -> ParseResult {
        if self.matching(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::unary(operator.clone(), right));
        }

        return self.primary();
    }

    fn primary(&self) -> ParseResult {
        if self.matching(&[TokenType::FALSE]) {
            return Ok(Expr::literal(Literal::Bool(false)));
        }
        if self.matching(&[TokenType::TRUE]) {
            return Ok(Expr::literal(Literal::Bool(true)));
        }
        if self.matching(&[TokenType::NIL]) {
            return Ok(Expr::literal(Literal::Nil));
        }

        if self.matching(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::literal(self.previous().literal.clone()));
        }

        if self.matching(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Expr::grouping(expr));
        }

        Err(self.report_error(self.peek(), "Expect expression."))
    }

    // fn a_parser(&self) -> ParseResult {
    //     Ok(Expr::Dummy)
    // }

    fn matching(&self, types: &[TokenType]) -> bool {
        for tt in types.iter() {
            if self.check(*tt) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, tt: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == tt
    }

    fn advance(&self) -> &Token {
        if !self.is_at_end() {
            let curr = self.curr();
            let mut currp = self.current.borrow_mut();
            *currp = curr + 1;
        }
        self.previous()
    }

    fn consume(&self, tt: TokenType, message: impl Into<String>) -> Result<&Token, ParseError> {
        if self.check(tt) {
            Ok(self.advance())
        } else {
            let token = self.peek();
            Err(self.report_error(token, message))
        }
    }

    fn synchronize(&self) {
        use TokenType::*;
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == SEMICOLON {
                return;
            }

            match self.peek().token_type {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => {
                    return;
                }

                _ => {}
            }

            self.advance();
        }
    }

    fn report_error(&self, token: &Token, message: impl Into<String>) -> ParseError {
        ScanError::report(token, message);
        ParseError::raise()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.curr()).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.curr() - 1).unwrap()
    }

    fn curr(&self) -> usize {
        *self.current.borrow()
    }
}
