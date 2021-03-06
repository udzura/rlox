use std::cell::RefCell;
use std::rc::Rc;

use crate::context::Context;
use crate::errors::*;

use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::*;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: Rc<RefCell<usize>>,

    context: Rc<RefCell<Context>>,
}

type ParseResult = Result<Expr, ParseError>;
type StmtResult = Result<Stmt, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>, context: Rc<RefCell<Context>>) -> Self {
        let current = Rc::new(RefCell::new(0));
        Self {
            tokens,
            current,
            context,
        }
    }

    pub fn parse(&self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }

        statements
    }

    fn declaration(&self) -> Stmt {
        match if self.matching(&[TokenType::CLASS]) {
            self.class_declaration()
        } else if self.matching(&[TokenType::FUN]) {
            self.function("function")
        } else if self.matching(&[TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        } {
            Ok(s) => s,
            Err(_) => {
                self.synchronize();
                Stmt::null()
            }
        }
    }

    fn function(&self, kind: &str) -> StmtResult {
        let name = self
            .consume(TokenType::IDENTIFIER, format!("Expect {} name.", kind))?
            .clone();
        self.consume(
            TokenType::LEFT_PAREN,
            format!("Expect '(' after {} name.", kind),
        )?;

        let mut parameters: Vec<Token> = vec![];
        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                if parameters.len() >= 255 {
                    return Err(self.error(self.peek(), "Can't have more than 255 arguments."));
                }

                parameters.push(
                    self.consume(TokenType::IDENTIFIER, "Expect parameter name.")?
                        .clone(),
                );
                if !self.matching(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after arguments.")?;
        self.consume(
            TokenType::LEFT_BRACE,
            format!("Expect {{ before {} body.", kind),
        )?;
        let body: Vec<Stmt> = self.block()?;

        return Ok(Stmt::fun(name, parameters, body));
    }

    fn class_declaration(&self) -> StmtResult {
        let name = self.consume(TokenType::IDENTIFIER, "Expect class name.")?;

        let mut superclass = None;
        if self.matching(&[TokenType::LESS]) {
            self.consume(TokenType::IDENTIFIER, "Expect superclass name.")?;
            superclass = Some(Expr::variable(self.previous().clone()));
        }

        self.consume(TokenType::LEFT_BRACE, "Expect '{' before class body.")?;

        let mut methods = vec![];
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }
        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after class body.")?;

        Ok(Stmt::class(name.clone(), superclass, methods))
    }

    fn var_declaration(&self) -> StmtResult {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;
        let initializer = if self.matching(&[TokenType::EQUAL]) {
            self.expression()?
        } else {
            Expr::literal(Literal::Nil)
        };
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::var(name.clone(), initializer))
    }

    fn statement(&self) -> StmtResult {
        if self.matching(&[TokenType::FOR]) {
            return self.for_statement();
        }
        if self.matching(&[TokenType::IF]) {
            return self.if_statement();
        }
        if self.matching(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        if self.matching(&[TokenType::RETURN]) {
            return self.return_statement();
        }
        if self.matching(&[TokenType::WHILE]) {
            return self.while_statement();
        }
        if self.matching(&[TokenType::LEFT_BRACE]) {
            return Ok(Stmt::block(self.block()?));
        }

        self.expression_statement()
    }

    fn block(&self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&self) -> StmtResult {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;

        Ok(Stmt::expression(expr))
    }

    fn if_statement(&self) -> StmtResult {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after if.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after if cond.")?;

        let then_stmt = self.statement()?;
        let else_stmt = if self.matching(&[TokenType::ELSE]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Stmt::if_stmt(condition, then_stmt, else_stmt))
    }

    fn while_statement(&self) -> StmtResult {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after while.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after while cond.")?;
        let body = self.statement()?;

        Ok(Stmt::while_stmt(condition, body))
    }

    fn for_statement(&self) -> StmtResult {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after for.")?;

        let initializer = if self.matching(&[TokenType::SEMICOLON]) {
            Stmt::null()
        } else if self.matching(&[TokenType::VAR]) {
            self.var_declaration()?
        } else {
            self.expression_statement()?
        };

        let condition = if self.check(TokenType::SEMICOLON) {
            Expr::literal(Literal::Bool(true))
        } else {
            self.expression()?
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after loop cond.")?;

        let increment = if self.check(TokenType::RIGHT_PAREN) {
            Expr::null()
        } else {
            self.expression()?
        };
        self.consume(TokenType::RIGHT_PAREN, "Expect '}' after loop incr.")?;

        let mut body = self.statement()?;

        match increment {
            Expr::Null => {}
            increment => body = Stmt::block(vec![body, Stmt::expression(increment)]),
        }

        body = Stmt::while_stmt(condition, body);

        match initializer {
            Stmt::Null => {}
            initializer => body = Stmt::block(vec![initializer, body]),
        }

        Ok(body)
    }

    fn print_statement(&self) -> StmtResult {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;

        Ok(Stmt::print(value))
    }

    fn return_statement(&self) -> StmtResult {
        let keyword = self.previous();
        let value: Expr = if self.check(TokenType::SEMICOLON) {
            Expr::literal(Literal::Nil)
        } else {
            self.expression()?
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after return value.")?;

        Ok(Stmt::return_stmt(keyword.clone(), value))
    }

    fn expression(&self) -> ParseResult {
        self.assignment()
    }

    fn assignment(&self) -> ParseResult {
        let expr = self.or()?;

        if self.matching(&[TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable_(expr) = expr {
                let name = expr.0.as_ref();
                return Ok(Expr::assign(name.clone(), value));
            } else if let Expr::Get_(get) = expr {
                let name = get.1.as_ref().clone();
                let obj = Box::into_inner(get.0);
                return Ok(Expr::set(obj, name, value));
            }

            return Err(self.error(equals, "Invalid assignment target."));
        }

        return Ok(expr);
    }

    fn or(&self) -> ParseResult {
        let mut expr = self.and()?;

        if self.matching(&[TokenType::OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::logical(expr, operator.clone(), right);
        }
        Ok(expr)
    }

    fn and(&self) -> ParseResult {
        let mut expr = self.equality()?;

        if self.matching(&[TokenType::AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::logical(expr, operator.clone(), right);
        }
        Ok(expr)
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

        self.call()
    }

    fn call(&self) -> ParseResult {
        let mut expr = self.primary()?;

        loop {
            if self.matching(&[TokenType::LEFT_PAREN]) {
                expr = self.finish_call(expr)?;
            } else if self.matching(&[TokenType::DOT]) {
                let name =
                    self.consume(TokenType::IDENTIFIER, "Expect property name after '.'.")?;
                expr = Expr::get(expr, name.clone());
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&self, callee: Expr) -> ParseResult {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    return Err(self.error(self.peek(), "Can't have more than 255 arguments."));
                }
                arguments.push(self.expression()?);
                if !self.matching(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RIGHT_PAREN, "Expect ')' after arguments.")?;

        Ok(Expr::call(callee, paren.clone(), arguments))
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

        if self.matching(&[TokenType::SUPER]) {
            let keyword = self.previous();
            self.consume(TokenType::DOT, "Expect '.' after 'super'.")?;
            let method = self.consume(TokenType::IDENTIFIER, "Expect superclass method name.")?;
            return Ok(Expr::super_(keyword.clone(), method.clone()));
        }

        if self.matching(&[TokenType::THIS]) {
            return Ok(Expr::this(self.previous().clone()));
        }

        if self.matching(&[TokenType::IDENTIFIER]) {
            return Ok(Expr::variable(self.previous().clone()));
        }

        Err(self.error(self.peek(), "Expect expression."))
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
            Err(self.error(token, message))
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

    fn error(&self, token: &Token, message: impl Into<String>) -> ParseError {
        self.context.borrow_mut().error_on(token, message);
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
