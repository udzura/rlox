use crate::errors::RuntimeBreak;
use crate::expr::*;
use crate::interpreter::Interpreter;
use crate::stmt::*;
use crate::token::Token;
use crate::visitor::*;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Resolver<'a> {
    pub interpreter: &'a mut Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        let scopes = Vec::new();
        Self {
            interpreter,
            scopes,
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        } else {
            return;
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        } else {
            return;
        }
    }

    fn resolve_local(&mut self, name: &Token) -> Result<(), RuntimeBreak> {
        let len = self.scopes.len();
        for i in (len - 1)..=0 {
            if self.scopes.get(i).unwrap().contains_key(&name.lexeme) {
                self.interpreter.resolve(name.clone(), len - 1 - i)?;
                return Ok(());
            }
        }
        Ok(())
    }

    fn resolve(&mut self, statements: &[Stmt]) -> Result<(), RuntimeBreak> {
        for stmt in statements.iter() {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_function(&mut self, function: &Rc<Fun>) -> Result<(), RuntimeBreak> {
        self.begin_scope();
        for param in function.1.iter() {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.2)?;
        self.end_scope();
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeBreak> {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), RuntimeBreak> {
        expr.accept(self)
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    type R = Result<(), RuntimeBreak>;

    fn visit_block(&mut self, stmt: &Block) -> Self::R {
        self.begin_scope();
        self.resolve(stmt.0.as_ref())?;
        self.end_scope();
        Ok(())
    }

    fn visit_class(&mut self, stmt: &Class) -> Self::R {
        Ok(())
    }

    fn visit_expression(&mut self, stmt: &Expression) -> Self::R {
        self.resolve_expr(stmt.0.as_ref())
    }

    fn visit_fun(&mut self, stmt: &Rc<Fun>) -> Self::R {
        self.declare(stmt.0.as_ref());
        self.declare(stmt.0.as_ref());

        self.resolve_function(stmt)
    }

    fn visit_if(&mut self, stmt: &If) -> Self::R {
        self.resolve_expr(stmt.0.as_ref())?;
        self.resolve_stmt(stmt.1.as_ref())?;
        if let Some(other) = stmt.2.as_ref() {
            self.resolve_stmt(other.as_ref())?;
        }
        Ok(())
    }

    fn visit_print(&mut self, stmt: &Print) -> Self::R {
        self.resolve_expr(stmt.0.as_ref())
    }

    fn visit_return(&mut self, stmt: &Return) -> Self::R {
        self.resolve_expr(stmt.1.as_ref())
    }

    fn visit_var(&mut self, stmt: &Var) -> Self::R {
        let name = stmt.0.as_ref();
        self.declare(name);
        match stmt.1.as_ref() {
            Expr::Null => {}
            initializer => {
                self.resolve_expr(initializer)?;
            }
        }

        self.define(name);
        Ok(())
    }

    fn visit_while(&mut self, stmt: &While) -> Self::R {
        self.resolve_expr(stmt.0.as_ref())?;
        self.resolve_stmt(stmt.1.as_ref())?;
        Ok(())
    }

    fn visit_null(&mut self) -> Self::R {
        Ok(())
    }
}

impl<'a> ExprVisitor for Resolver<'a> {
    type R = Result<(), RuntimeBreak>;

    fn visit_assign(&mut self, expr: &Assign) -> Self::R {
        let expr_ = Expr::Assign_(expr.clone());
        self.resolve_expr(expr.1.as_ref())?;
        self.resolve_local(expr.0.as_ref())
    }

    fn visit_binary(&mut self, expr: &Binary) -> Self::R {
        self.resolve_expr(expr.0.as_ref())?;
        self.resolve_expr(expr.2.as_ref())?;
        Ok(())
    }

    fn visit_call(&mut self, expr: &Call) -> Self::R {
        self.resolve_expr(expr.0.as_ref())?;
        for argument in expr.2.iter() {
            self.resolve_expr(argument)?;
        }
        Ok(())
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::R {
        self.resolve_expr(expr.0.as_ref())
    }

    fn visit_literal(&mut self, expr: &Lit) -> Self::R {
        Ok(())
    }

    fn visit_logical(&mut self, expr: &Logical) -> Self::R {
        self.resolve_expr(expr.0.as_ref())?;
        self.resolve_expr(expr.2.as_ref())?;
        Ok(())
    }

    fn visit_set(&mut self, expr: &Set) -> Self::R {
        Ok(())
    }

    fn visit_super(&mut self, expr: &Super) -> Self::R {
        Ok(())
    }

    fn visit_this(&mut self, expr: &This) -> Self::R {
        Ok(())
    }

    fn visit_unary(&mut self, expr: &Unary) -> Self::R {
        self.resolve_expr(expr.1.as_ref())?;
        Ok(())
    }

    fn visit_variable(&mut self, expr: &Variable) -> Self::R {
        if !self.scopes.is_empty()
            && !self
                .scopes
                .last()
                .unwrap()
                .get(&expr.0.as_ref().lexeme)
                .unwrap_or(&true)
        {
            return Err(RuntimeBreak::raise(
                expr.0.as_ref().clone(),
                "Can't read local variable in its own initializer.",
            ));
        }

        self.resolve_local(expr.0.as_ref())?;

        Ok(())
    }

    fn visit_null(&mut self) -> Self::R {
        Ok(())
    }
}
