use std::hash::{Hash, Hasher};

use super::token::{Literal, Token};
use super::visitor::ExprVisitor;

pub type ExprP = Box<Expr>;
pub type ExprV = Vec<Expr>;
pub type TokenP = Box<Token>;
pub type LiteralP = Box<Literal>;

#[derive(Debug, Clone)]
pub struct Assign(pub TokenP, pub ExprP);
#[derive(Debug, Clone)]
pub struct Binary(pub ExprP, pub TokenP, pub ExprP);
#[derive(Debug, Clone)]
pub struct Call(pub ExprP, pub TokenP, pub ExprV);
#[derive(Debug, Clone)]
pub struct Get(pub ExprP, pub TokenP);
#[derive(Debug, Clone)]
pub struct Grouping(pub ExprP);
#[derive(Debug, Clone)]
pub struct Lit(pub LiteralP);
#[derive(Debug, Clone)]
pub struct Logical(pub ExprP, pub TokenP, pub ExprP);
#[derive(Debug, Clone)]
pub struct Set(pub ExprP, pub TokenP, pub ExprP);
#[derive(Debug, Clone)]
pub struct Super(pub TokenP, pub TokenP);
#[derive(Debug, Clone)]
pub struct This(pub TokenP);
#[derive(Debug, Clone)]
pub struct Unary(pub TokenP, pub ExprP);
#[derive(Debug, Clone)]
pub struct Variable(pub TokenP);

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Expr {
    Assign_(Assign),
    Binary_(Binary),
    Call_(Call),
    Get_(Get),
    Grouping_(Grouping),
    Literal_(Lit),
    Logical_(Logical),
    Set_(Set),
    Super_(Super),
    This_(This),
    Unary_(Unary),
    Variable_(Variable),

    Null,
    Dummy,
}

impl Expr {
    pub fn assign(name: Token, value: Self) -> Self {
        Self::Assign_(Assign(Box::new(name), Box::new(value)))
    }

    pub fn binary(left: Self, operator: Token, right: Self) -> Self {
        Self::Binary_(Binary(Box::new(left), Box::new(operator), Box::new(right)))
    }

    pub fn call(callee: Self, paren: Token, arguments: Vec<Self>) -> Self {
        Self::Call_(Call(Box::new(callee), Box::new(paren), arguments))
    }

    pub fn get(object: Self, name: Token) -> Self {
        Self::Get_(Get(Box::new(object), Box::new(name)))
    }

    pub fn grouping(expr: Self) -> Self {
        Self::Grouping_(Grouping(Box::new(expr)))
    }

    pub fn literal(literal: Literal) -> Self {
        Self::Literal_(Lit(Box::new(literal)))
    }

    pub fn logical(left: Self, operator: Token, right: Self) -> Self {
        Self::Logical_(Logical(Box::new(left), Box::new(operator), Box::new(right)))
    }

    pub fn set(object: Self, name: Token, value: Self) -> Self {
        Self::Set_(Set(Box::new(object), Box::new(name), Box::new(value)))
    }

    pub fn super_(keyword: Token, method: Token) -> Self {
        Self::Super_(Super(Box::new(keyword), Box::new(method)))
    }

    pub fn this(keyword: Token) -> Self {
        Self::This_(This(Box::new(keyword)))
    }

    pub fn unary(operator: Token, right: Self) -> Self {
        Self::Unary_(Unary(Box::new(operator), Box::new(right)))
    }

    pub fn variable(name: Token) -> Self {
        Self::Variable_(Variable(Box::new(name)))
    }

    pub fn null() -> Self {
        Self::Null
    }
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut T) -> <T as ExprVisitor>::R
    where
        T: ExprVisitor,
    {
        use Expr::*;
        match self {
            Assign_(expr) => visitor.visit_assign(expr),
            Binary_(expr) => visitor.visit_binary(expr),
            Call_(expr) => visitor.visit_call(expr),
            Get_(expr) => visitor.visit_get(expr),
            Grouping_(expr) => visitor.visit_grouping(expr),
            Literal_(expr) => visitor.visit_literal(expr),
            Logical_(expr) => visitor.visit_logical(expr),
            Set_(expr) => visitor.visit_set(expr),
            Super_(expr) => visitor.visit_super(expr),
            This_(expr) => visitor.visit_this(expr),
            Unary_(expr) => visitor.visit_unary(expr),
            Variable_(expr) => visitor.visit_variable(expr),
            Null => visitor.visit_null(),
            _ => panic!("[BUG] invalid type of expr."),
        }
    }
}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Expr::*;
        match self {
            Assign_(expr) => expr.0.hash(state),
            Variable_(expr) => expr.0.hash(state),
            _ => format!("{:?}", self).hash(state),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        use Expr::*;
        match (self, other) {
            (Assign_(expr), Assign_(oth)) => expr.0 == oth.0,
            (Variable_(expr), Variable_(oth)) => expr.0 == oth.0,
            _ => false,
        }
    }
}
impl Eq for Expr {}
