use std::cell::RefCell;
use std::rc::Rc;

use crate::callable::Callable;
use crate::environment::Environment;
use crate::errors::RuntimeBreak;
use crate::expr::*;
use crate::function::Function;
use crate::stmt::*;
use crate::token::*;
use crate::value::Value;
use crate::visitor::*;

#[derive(Debug)]
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        let environment = globals.clone();

        fn fun_clock(_interpreter: &Interpreter, _arguments: &[Value]) -> Value {
            use std::time::*;
            let millis = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Value::Number(millis as f64)
        }

        let function = Function::new_native("clock", 0u8, fun_clock);
        globals
            .borrow_mut()
            .define("clock", Value::LoxFunction(function));

        Self {
            globals,
            environment,
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeBreak> {
        for statement in statements.iter() {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), RuntimeBreak> {
        let mut replacement = Rc::new(RefCell::new(environment));
        std::mem::swap(&mut self.environment, &mut replacement);

        let res = 'trying: loop {
            for statement in statements.iter() {
                if let Err(e) = self.execute(statement) {
                    break 'trying Err(e);
                }
            }
            break Ok(());
        };

        std::mem::swap(&mut self.environment, &mut replacement);

        // let environment = self.environment.take();
        // // if let Some(enclosing) = environment.take_enclosing() {
        // //     let original = RefCell::new(enclosing);
        // //     self.environment.swap(&original);
        // // } else {
        // //dbg!("before replace back");
        // //dbg!(&replacement);
        // self.environment.swap(&replacement);
        // //dbg!("after replace back");
        // //dbg!(&replacement);
        // //panic!("BUG: missing enclosure");
        // // }
        res
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeBreak> {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeBreak> {
        expr.accept(self)
    }

    fn is_truthy(value: &Value) -> bool {
        use Value::*;
        match value {
            Nil => false,
            Boolean(b) => b.to_owned(),

            _ => true,
        }
    }

    fn check_number_operand(
        &mut self,
        oprator: &Token,
        operand: &Value,
    ) -> Result<f64, RuntimeBreak> {
        match operand {
            Value::Number(n) => return Ok(*n),
            _ => Err(RuntimeBreak::raise(
                oprator.clone(),
                "Operand must be a number.",
            )),
        }
    }

    fn check_number_operands(
        &mut self,
        oprator: &Token,
        left: &Value,
        right: &Value,
    ) -> Result<(f64, f64), RuntimeBreak> {
        use Value::*;
        if let (Number(l), Number(r)) = (left, right) {
            return Ok((*l, *r));
        } else {
            return Err(RuntimeBreak::raise(
                oprator.clone(),
                "Operand must be a number.",
            ));
        };
    }
}

impl StmtVisitor for Interpreter {
    type R = Result<(), RuntimeBreak>;

    fn visit_block(&mut self, stmt: &Block) -> Self::R {
        let environment = self.environment.clone();
        let environment = Environment::new(Some(environment));
        self.execute_block(&stmt.0, environment)
    }

    fn visit_expression(&mut self, stmt: &crate::stmt::Expression) -> Self::R {
        self.evaluate(stmt.0.as_ref())?;
        Ok(())
    }

    fn visit_fun(&mut self, stmt: &Rc<Fun>) -> Self::R {
        let name = &stmt.0.as_ref().lexeme;
        let function = Function::new_lox(stmt.clone(), Some(self.environment.clone()));
        let value = Value::LoxFunction(function);
        self.environment.borrow_mut().define(name, value);

        Ok(())
    }

    fn visit_if(&mut self, stmt: &If) -> Self::R {
        if Self::is_truthy(&(self.evaluate(stmt.0.as_ref())?)) {
            self.execute(stmt.1.as_ref())
        } else if let Some(boxed) = stmt.2.as_ref() {
            let else_stmt = boxed.as_ref();
            self.execute(else_stmt)
        } else {
            Ok(())
        }
    }

    fn visit_while(&mut self, stmt: &While) -> Self::R {
        let cond = stmt.0.as_ref();
        while Self::is_truthy(&(self.evaluate(cond)?)) {
            self.execute(stmt.1.as_ref())?;
        }
        Ok(())
    }

    fn visit_print(&mut self, stmt: &crate::stmt::Print) -> Self::R {
        let value: Value = self.evaluate(stmt.0.as_ref())?;
        println!("{}", value);
        Ok(())
    }

    fn visit_return(&mut self, stmt: &Return) -> Self::R {
        let value = self.evaluate(stmt.1.as_ref())?;

        Err(RuntimeBreak::ret(value))
    }

    fn visit_var(&mut self, stmt: &Var) -> Self::R {
        let initializer = stmt.1.as_ref();
        let value = self.evaluate(initializer)?;

        let mut environment = self.environment.borrow_mut();
        environment.define(&stmt.0.as_ref().lexeme, value);
        Ok(())
    }

    fn visit_null(&mut self) -> Self::R {
        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    type R = Result<Value, RuntimeBreak>;

    fn visit_assign(&mut self, expr: &Assign) -> Self::R {
        let value = self.evaluate(expr.1.as_ref())?;
        let mut envronmnt = self.environment.borrow_mut();
        envronmnt.assign(expr.0.as_ref(), value.clone())?;
        Ok(value)
    }

    fn visit_binary(&mut self, expr: &Binary) -> Self::R {
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
                    return Err(RuntimeBreak::raise(operator.clone(), "Devided by 0"));
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
                    return Err(RuntimeBreak::raise(
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

    fn visit_call(&mut self, expr: &Call) -> Self::R {
        let callee = self.evaluate(expr.0.as_ref())?;
        let mut arguments: Vec<Value> = vec![];
        for v in expr.2.iter() {
            arguments.push(self.evaluate(v)?);
        }

        if let Value::LoxFunction(function) = callee {
            if arguments.len() != function.arity() as usize {
                return Err(RuntimeBreak::raise(
                    expr.1.as_ref().to_owned(),
                    &format!(
                        "Expected {arity} arguments but got {len}.",
                        arity = function.arity(),
                        len = arguments.len()
                    ),
                ));
            }

            function.call(self, &arguments)
        } else {
            Err(RuntimeBreak::raise(
                expr.1.as_ref().to_owned(),
                "Can only call functions and classes.",
            ))
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::R {
        self.evaluate(expr.0.as_ref())
    }

    fn visit_literal(&mut self, expr: &Lit) -> Self::R {
        Ok(expr.0.as_ref().into())
    }

    fn visit_logical(&mut self, expr: &Logical) -> Self::R {
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

    fn visit_unary(&mut self, expr: &Unary) -> Self::R {
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

    fn visit_variable(&mut self, expr: &Variable) -> Self::R {
        let environment = self.environment.borrow();
        let v = environment.get(expr.0.as_ref())?;
        Ok(v.clone())
    }

    fn visit_null(&mut self) -> Self::R {
        Ok(Value::Nil)
    }
}
