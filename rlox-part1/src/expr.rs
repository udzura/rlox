use super::token::{Literal, Token};
use super::visitor::Visitor;

type ExprP = Box<Expr>;
type TokenP = Box<Token>;
type LteralP = Box<Literal>;

pub struct Binary(ExprP, TokenP, ExprP);
pub struct Grouping(ExprP);
pub struct Lit(LteralP);
pub struct Unary(TokenP, ExprP);

pub enum Expr {
    Binary_(Binary),
    Grouping_(Grouping),
    Literal_(Lit),
    Unary_(Unary),
}

impl Expr {
    pub fn accept<T>(&self, visitor: T) -> <T as Visitor>::R
    where
        T: Visitor,
    {
        use Expr::*;
        match self {
            Binary_(expr) => visitor.visit_binary(expr),
            Grouping_(expr) => visitor.visit_grouping(expr),
            Literal_(expr) => visitor.visit_literal(expr),
            Unary_(expr) => visitor.visit_unary(expr),
            // _ => panic!("[BUG] invalid type of expr."),
        }
    }
}
