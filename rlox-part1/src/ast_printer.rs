use super::expr::*;
use super::visitor::ExprVisitor;

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut buf = String::new();
        buf.push_str("(");
        buf.push_str(name);
        for expr in exprs.iter() {
            buf.push_str(" ");
            let value = (*expr).accept(self);
            buf.push_str(&value);
        }
        buf.push_str(")");

        buf
    }
}

impl ExprVisitor for AstPrinter {
    type R = String;

    fn visit_assign(&self, _expr: &Assign) -> Self::R {
        todo!()
    }

    fn visit_binary(&self, expr: &Binary) -> Self::R {
        let name = &expr.1.lexeme;
        let exprs = vec![expr.0.as_ref(), expr.2.as_ref()];
        self.parenthesize(name, &exprs)
    }

    fn visit_grouping(&self, expr: &Grouping) -> Self::R {
        let exprs = vec![expr.0.as_ref()];
        self.parenthesize("group", &exprs)
    }

    fn visit_literal(&self, expr: &Lit) -> Self::R {
        use super::token::Literal;
        match expr.0.as_ref() {
            Literal::Nil => "nil".to_string(),
            lit => format!("{:?}", lit.value()),
        }
    }

    fn visit_unary(&self, expr: &Unary) -> Self::R {
        let name = &expr.0.lexeme;
        let right = vec![expr.1.as_ref()];
        self.parenthesize(name, &right)
    }

    fn visit_variable(&self, _expr: &Variable) -> Self::R {
        "".to_string()
    }

    fn visit_logical(&self, _expr: &Logical) -> Self::R {
        todo!()
    }

    fn visit_null(&self) -> Self::R {
        todo!()
    }
}
