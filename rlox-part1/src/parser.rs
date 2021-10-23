use std::cell::RefCell;
use std::rc::Rc;

use crate::errors::*;

use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::*;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: Rc<RefCell<usize>>,
}

type ParseResult = Result<Expr, ParseError>;
type StmtResult = Result<Stmt, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let current = Rc::new(RefCell::new(0));
        Self { tokens, current }
    }

    pub fn parse(&self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&self) -> StmtResult {
        if self.matching(&[TokenType::PRINT]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn expression_statement(&self) -> StmtResult {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;

        Ok(Stmt::expression(expr))
    }

    fn print_statement(&self) -> StmtResult {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;

        Ok(Stmt::print(value))
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
