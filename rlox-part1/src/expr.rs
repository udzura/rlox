use super::token::{Literal, Token};
use super::visitor::ExprVisitor;

pub type ExprP = Box<Expr>;
pub type TokenP = Box<Token>;
pub type LiteralP = Box<Literal>;

#[derive(Debug, Clone)]
pub struct Binary(pub ExprP, pub TokenP, pub ExprP);
#[derive(Debug, Clone)]
pub struct Grouping(pub ExprP);
#[derive(Debug, Clone)]
pub struct Lit(pub LiteralP);
#[derive(Debug, Clone)]
pub struct Unary(pub TokenP, pub ExprP);

#[derive(Debug, Clone)]
pub enum Expr {
    Binary_(Binary),
    Grouping_(Grouping),
    Literal_(Lit),
    Unary_(Unary),

    Dummy,
}

impl Expr {
    pub fn binary(left: Self, operator: Token, right: Self) -> Self {
        Self::Binary_(Binary(Box::new(left), Box::new(operator), Box::new(right)))
    }

    pub fn grouping(expr: Self) -> Self {
        Self::Grouping_(Grouping(Box::new(expr)))
    }

    pub fn literal(literal: Literal) -> Self {
        Self::Literal_(Lit(Box::new(literal)))
    }

    pub fn unary(operator: Token, right: Self) -> Self {
        Self::Unary_(Unary(Box::new(operator), Box::new(right)))
    }
}

impl Expr {
    pub fn accept<T>(&self, visitor: &T) -> <T as ExprVisitor>::R
    where
        T: ExprVisitor,
    {
        use Expr::*;
        match self {
            Binary_(expr) => visitor.visit_binary(expr),
            Grouping_(expr) => visitor.visit_grouping(expr),
            Literal_(expr) => visitor.visit_literal(expr),
            Unary_(expr) => visitor.visit_unary(expr),
            _ => panic!("[BUG] invalid type of expr."),
        }
    }
}
