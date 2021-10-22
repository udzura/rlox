use super::value::Value;
use super::visitor::Visitor;
use crate::errors::RuntimeError;
use crate::expr::*;
use crate::token::*;

#[derive(Debug)]
struct Interpreter {}

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }

    pub fn is_truthy(value: Value) -> bool {
        use Value::*;
        match value {
            Nil => false,
            Boolean(b) => b,

            Number(_) | LoxString(_) | Object(_) => true,
        }
    }

    pub fn check_number_operand(
        &self,
        oprator: &Token,
        operand: &Value,
    ) -> Result<f64, RuntimeError> {
        match operand {
            Value::Number(n) => return Ok(*n),
            _ => Err(RuntimeError::raise(
                oprator.clone(),
                "Operand must be a number.",
            )),
        }
    }

    pub fn check_number_operands(
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

impl Visitor for Interpreter {
    type R = Result<Value, RuntimeError>;

    fn visit_binary(&self, expr: &Binary) -> Self::R {
        let left = self.evaluate(expr.0.as_ref())?;
        let right = self.evaluate(expr.2.as_ref())?;
        match expr.0.as_ref().token_type {}
    }

    fn visit_grouping(&self, expr: &Grouping) -> Self::R {
        self.evaluate(expr.0.as_ref())
    }

    fn visit_literal(&self, expr: &Lit) -> Self::R {
        Ok(expr.0.as_ref().into())
    }

    fn visit_unary(&self, expr: &Unary) -> Self::R {
        let right = self.evaluate(expr.1.as_ref())?;
        match expr.0.as_ref().token_type {
            TokenType::BANG => {
                let b = Self::is_truthy(right);
                return Ok(Value::Boolean(b));
            }
            TokenType::MINUS => {
                let right = self.check_number_operand(expr.0.as_ref(), &right)?;
                return Ok(Value::Number(-right));
            }
            _ => {}
        }

        // Should be unreachable...
        // Value::Nil
        panic!("[BUG] Sould not be reachable");
    }
}
