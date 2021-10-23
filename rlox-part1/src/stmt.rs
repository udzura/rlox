use crate::expr::*;
use crate::visitor::StmtVisitor;

#[derive(Debug, Clone)]
pub struct Expression(pub ExprP);
#[derive(Debug, Clone)]
pub struct Print(pub ExprP);

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression_(Expression),
    Print_(Print),

    Dummy,
}

impl Stmt {
    pub fn expression(expr: Expr) -> Self {
        Self::Expression_(Expression(Box::new(expr)))
    }

    pub fn print(expr: Expr) -> Self {
        Self::Print_(Print(Box::new(expr)))
    }
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &T) -> <T as StmtVisitor>::R
    where
        T: StmtVisitor,
    {
        use Stmt::*;
        match self {
            Expression_(stmt) => visitor.visit_expression(stmt),
            Print_(stmt) => visitor.visit_print(stmt),
            _ => panic!("[BUG] invalid type of expr."),
        }
    }
}
