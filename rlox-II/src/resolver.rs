use crate::context::Context;
use crate::expr::*;
use crate::interpreter::Interpreter;
use crate::stmt::*;
use crate::token::Literal;
use crate::token::Token;
use crate::visitor::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ClassType {
    None,
    Class,
    SubClass,
}

#[derive(Debug)]
pub struct Resolver<'a> {
    pub interpreter: &'a mut Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,

    context: Rc<RefCell<Context>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter, context: Rc<RefCell<Context>>) -> Self {
        let scopes = Vec::new();
        Self {
            interpreter,
            scopes,
            current_function: FunctionType::None,
            current_class: ClassType::None,
            context,
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        let mut had_error = false;
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                had_error = true
            }
            scope.insert(name.lexeme.clone(), false);
        }

        if had_error {
            self.error(name, "Already a variable with this name in this scope.");
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        } else {
            return;
        }
    }

    fn resolve_local(&mut self, name: &Token) {
        let len = self.scopes.len();
        if len == 0 {
            return;
        }

        for i in (0..=(len - 1)).rev() {
            if self.scopes.get(i).unwrap().contains_key(&name.lexeme) {
                self.interpreter.resolve(name.clone(), len - 1 - i);
                return;
            }
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) {
        for stmt in statements.iter() {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_function(&mut self, function: &Rc<Fun>, funtype: FunctionType) {
        let enclosing_function = self.current_function;
        self.current_function = funtype;

        self.begin_scope();
        for param in function.1.iter() {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.2);
        self.end_scope();
        self.current_function = enclosing_function;
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self);
    }

    fn error(&mut self, token: &Token, message: impl Into<String>) {
        self.context.borrow_mut().error_on(token, message);
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    type R = ();

    fn visit_block(&mut self, stmt: &Block) -> Self::R {
        self.begin_scope();
        self.resolve(stmt.0.as_ref());
        self.end_scope();
    }

    fn visit_class(&mut self, stmt: &Class) -> Self::R {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;

        self.declare(stmt.0.as_ref());
        self.define(stmt.0.as_ref());

        if let Some(superclass) = stmt.1.as_ref() {
            self.current_class = ClassType::SubClass;
            if let Expr::Variable_(var) = superclass.as_ref() {
                if stmt.0.as_ref().lexeme == var.0.as_ref().lexeme {
                    self.error(var.0.as_ref(), "A class can't inherit from itself.");
                }
            }

            self.resolve_expr(superclass.as_ref());
        }

        if stmt.1.is_some() {
            self.begin_scope();
            self.scopes
                .last_mut()
                .unwrap()
                .insert(String::from_str("super").unwrap(), true);
        }

        self.begin_scope();
        self.scopes
            .last_mut()
            .unwrap()
            .insert(String::from_str("this").unwrap(), true);

        for method in stmt.2.iter() {
            match method {
                Stmt::Fun_(fun) => {
                    let declaration = if &fun.0.as_ref().lexeme == "init" {
                        FunctionType::Initializer
                    } else {
                        FunctionType::Method
                    };

                    self.resolve_function(fun, declaration);
                }
                _ => {
                    self.error(
                        stmt.0.as_ref(),
                        "[BUG] Included a stmt that is ot a function.",
                    );
                }
            }
        }

        self.end_scope();
        if stmt.1.is_some() {
            self.end_scope();
        }
        self.current_class = enclosing_class;
    }

    fn visit_expression(&mut self, stmt: &Expression) -> Self::R {
        self.resolve_expr(stmt.0.as_ref())
    }

    fn visit_fun(&mut self, stmt: &Rc<Fun>) -> Self::R {
        self.declare(stmt.0.as_ref());
        self.define(stmt.0.as_ref());

        self.resolve_function(stmt, FunctionType::Function);
    }

    fn visit_if(&mut self, stmt: &If) -> Self::R {
        self.resolve_expr(stmt.0.as_ref());
        self.resolve_stmt(stmt.1.as_ref());
        if let Some(other) = stmt.2.as_ref() {
            self.resolve_stmt(other.as_ref());
        }
    }

    fn visit_print(&mut self, stmt: &Print) -> Self::R {
        self.resolve_expr(stmt.0.as_ref())
    }

    fn visit_return(&mut self, stmt: &Return) -> Self::R {
        if self.current_function == FunctionType::None {
            self.error(stmt.0.as_ref(), "Can't return from top-level code.");
        }

        let value = stmt.1.as_ref();
        if match value {
            Expr::Literal_(Lit(b)) => {
                if let Literal::Nil = b.as_ref() {
                    false
                } else {
                    true
                }
            }
            _ => self.current_function == FunctionType::Initializer,
        } {
            self.error(stmt.0.as_ref(), "Can't return a value from an initializer.");
        }
        self.resolve_expr(value);
    }

    fn visit_var(&mut self, stmt: &Var) -> Self::R {
        let name = stmt.0.as_ref();
        self.declare(name);
        match stmt.1.as_ref() {
            Expr::Null => {}
            initializer => {
                self.resolve_expr(initializer);
            }
        }

        self.define(name);
    }

    fn visit_while(&mut self, stmt: &While) -> Self::R {
        self.resolve_expr(stmt.0.as_ref());
        self.resolve_stmt(stmt.1.as_ref());
    }

    fn visit_null(&mut self) -> Self::R {
        ()
    }
}

impl<'a> ExprVisitor for Resolver<'a> {
    type R = ();

    fn visit_assign(&mut self, expr: &Assign) -> Self::R {
        //let expr_ = Expr::Assign_(expr.clone());
        self.resolve_expr(expr.1.as_ref());
        self.resolve_local(expr.0.as_ref());
    }

    fn visit_binary(&mut self, expr: &Binary) -> Self::R {
        self.resolve_expr(expr.0.as_ref());
        self.resolve_expr(expr.2.as_ref());
    }

    fn visit_call(&mut self, expr: &Call) -> Self::R {
        self.resolve_expr(expr.0.as_ref());
        for argument in expr.2.iter() {
            self.resolve_expr(argument);
        }
    }

    fn visit_get(&mut self, expr: &Get) -> Self::R {
        self.resolve_expr(expr.0.as_ref());
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::R {
        self.resolve_expr(expr.0.as_ref());
    }

    fn visit_literal(&mut self, _expr: &Lit) -> Self::R {
        ()
    }

    fn visit_logical(&mut self, expr: &Logical) -> Self::R {
        self.resolve_expr(expr.0.as_ref());
        self.resolve_expr(expr.2.as_ref());
    }

    fn visit_set(&mut self, expr: &Set) -> Self::R {
        self.resolve_expr(expr.2.as_ref());
        self.resolve_expr(expr.0.as_ref());
    }

    fn visit_super(&mut self, expr: &Super) -> Self::R {
        match self.current_class {
            ClassType::None => self.error(expr.0.as_ref(), "Can't use 'super' outside of a class."),
            ClassType::Class => self.error(
                expr.0.as_ref(),
                "Can't use 'super' in a class with no superclass.",
            ),
            _ => {}
        }

        self.resolve_local(expr.0.as_ref());
    }

    fn visit_this(&mut self, expr: &This) -> Self::R {
        match self.current_class {
            ClassType::None => self.error(expr.0.as_ref(), "Can't use 'this' outside of a class."),
            _ => self.resolve_local(expr.0.as_ref()),
        }
    }

    fn visit_unary(&mut self, expr: &Unary) -> Self::R {
        self.resolve_expr(expr.1.as_ref());
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
            self.error(
                expr.0.as_ref(),
                "Can't read local variable in its own initializer.",
            );
        }

        self.resolve_local(expr.0.as_ref());
    }

    fn visit_null(&mut self) -> Self::R {
        ()
    }
}
