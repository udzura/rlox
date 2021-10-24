use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::expr::*;
use crate::stmt::*;
use crate::token::*;
use crate::value::Value;
use crate::visitor::*;

#[derive(Debug)]
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new(None))),
        }
    }

    pub fn interpret(&self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for statement in statements.iter() {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute_block(&self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        let environment = self.environment.take();
        let environment = Environment::new(Some(Rc::new(RefCell::new(environment))));
        let replacement = RefCell::new(environment);
        self.environment.swap(&replacement);

        let res = 'trying: loop {
            for statement in statements.iter() {
                if let Err(e) = self.execute(statement) {
                    break 'trying Err(e);
                }
            }
            break Ok(());
        };

        let environment = self.environment.take();
        if let Some(enclosing) = environment.take_enclosing() {
            let replacement = RefCell::new(enclosing);
            self.environment.swap(&replacement);
        } else {
            panic!("BUG: missing enclosure");
        }
        res
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }

    fn is_truthy(value: &Value) -> bool {
        use Value::*;
        match value {
            Nil => false,
            Boolean(b) => b.to_owned(),

            Number(_) | LoxString(_) => true,
        }
    }

    fn check_number_operand(&self, oprator: &Token, operand: &Value) -> Result<f64, RuntimeError> {
        match operand {
            Value::Number(n) => return Ok(*n),
            _ => Err(RuntimeError::raise(
                oprator.clone(),
                "Operand must be a number.",
            )),
        }
    }

    fn check_number_operands(
        &self,
        oprator: &Token,
        left: &Value,
        right: &Value,
    ) -> Result<(f64, f64), RuntimeError> {
        use Value::*;
        if let (Number(l), Number(r)) = (left, right) {
            return Ok((*l, *r));
        } else {
            return Err(RuntimeError::raise(
                oprator.clone(),
                "Operand must be a number.",
            ));
        };
    }
}

impl StmtVisitor for Interpreter {
    type R = Result<(), RuntimeError>;

    fn visit_block(&self, stmt: &Block) -> Self::R {
        self.execute_block(&stmt.0)
    }

    fn visit_expression(&self, stmt: &crate::stmt::Expression) -> Self::R {
        self.evaluate(stmt.0.as_ref())?;
        Ok(())
    }

    fn visit_if(&self, stmt: &If) -> Self::R {
        if Self::is_truthy(&(self.evaluate(stmt.0.as_ref())?)) {
            self.execute(stmt.1.as_ref())
        } else if let Some(boxed) = stmt.2.as_ref() {
            let else_stmt = boxed.as_ref();
            self.execute(else_stmt)
        } else {
            Ok(())
        }
    }

    fn visit_while(&self, stmt: &While) -> Self::R {
        let cond = stmt.0.as_ref();
        while Self::is_truthy(&(self.evaluate(cond)?)) {
            self.execute(stmt.1.as_ref())?;
        }
        Ok(())
    }

    fn visit_print(&self, stmt: &crate::stmt::Print) -> Self::R {
        let value: Value = self.evaluate(stmt.0.as_ref())?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var(&self, stmt: &Var) -> Self::R {
        let initializer = stmt.1.as_ref();
        let value = self.evaluate(initializer)?;

        let mut environment = self.environment.borrow_mut();
        environment.define(&stmt.0.as_ref().lexeme, value);
        Ok(())
    }

    fn visit_null(&self) -> Self::R {
        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    type R = Result<Value, RuntimeError>;

    fn visit_assign(&self, expr: &Assign) -> Self::R {
        let value = self.evaluate(expr.1.as_ref())?;
        let mut envronmnt = self.environment.borrow_mut();
        envronmnt.assign(expr.0.as_ref(), value.clone())?;
        Ok(value)
    }

    fn visit_binary(&self, expr: &Binary) -> Self::R {
        use TokenType::*;

        let left = self.evaluate(expr.0.as_ref())?;
        let right = self.evaluate(expr.2.as_ref())?;
        let operator = expr.1.as_ref();
        match operator.token_type {
            GREATER => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                return Ok(Value::Boolean(left > right));
            }
            GREATER_EQUAL => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                return Ok(Value::Boolean(left >= right));
            }
            LESS => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                return Ok(Value::Boolean(left < right));
            }
            LESS_EQUAL => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                return Ok(Value::Boolean(left <= right));
            }
            BANG_EQUAL => {
                return Ok(Value::Boolean(left != right));
            }
            EQUAL_EQUAL => {
                return Ok(Value::Boolean(left == right));
            }
            MINUS => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                return Ok(Value::Number(left - right));
            }
            SLASH => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                if right == 0f64 {
                    return Err(RuntimeError::raise(operator.clone(), "Devided by 0"));
                }
                return Ok(Value::Number(left / right));
            }
            STAR => {
                let (left, right) = self.check_number_operands(operator, &left, &right)?;
                return Ok(Value::Number(left * right));
            }
            PLUS => {
                use Value::*;
                if let (Number(l), Number(r)) = (&left, &right) {
                    return Ok(Number(*l + *r));
                } else if let (LoxString(l), LoxString(r)) = (&left, &right) {
                    return Ok(LoxString(format!("{}{}", l, r)));
                } else {
                    return Err(RuntimeError::raise(
                        operator.clone(),
                        "Operands must be numbers or strings.",
                    ));
                };
            }
            _ => {
                unreachable!("[BUG] Maybe a parser bug");
            }
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> Self::R {
        self.evaluate(expr.0.as_ref())
    }

    fn visit_literal(&self, expr: &Lit) -> Self::R {
        Ok(expr.0.as_ref().into())
    }

    fn visit_logical(&self, expr: &Logical) -> Self::R {
        let left = self.evaluate(expr.0.as_ref())?;

        if expr.1.as_ref().token_type == TokenType::OR {
            if Self::is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !Self::is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(expr.2.as_ref())
    }

    fn visit_unary(&self, expr: &Unary) -> Self::R {
        use TokenType::*;

        let right = self.evaluate(expr.1.as_ref())?;
        match expr.0.as_ref().token_type {
            BANG => {
                let b = Self::is_truthy(&right);
                return Ok(Value::Boolean(!b));
            }
            MINUS => {
                let right = self.check_number_operand(expr.0.as_ref(), &right)?;
                return Ok(Value::Number(-right));
            }
            _ => {}
        }

        // Should be unreachable...
        // Value::Nil
        unreachable!("[BUG] Maybe the parser has bug");
    }

    fn visit_variable(&self, expr: &Variable) -> Self::R {
        let environment = self.environment.borrow();
        let v = environment.get(expr.0.as_ref())?;
        Ok(v.clone())
    }

    fn visit_null(&self) -> Self::R {
        Ok(Value::Nil)
    }
}
