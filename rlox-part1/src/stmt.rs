use std::rc::Rc;

use crate::expr::*;
use crate::token::*;
use crate::visitor::StmtVisitor;

type StmtP = Box<Stmt>;
type Statements = Vec<Stmt>;

#[derive(Debug, Clone)]
pub struct Block(pub Statements);
#[derive(Debug, Clone)]
pub struct Expression(pub ExprP);
#[derive(Debug, Clone)]
pub struct Fun(pub TokenP, pub Vec<Token>, pub Statements);
#[derive(Debug, Clone)]
pub struct If(pub ExprP, pub StmtP, pub Option<StmtP>);
#[derive(Debug, Clone)]
pub struct Print(pub ExprP);
#[derive(Debug, Clone)]
pub struct Return(pub TokenP, pub ExprP);
#[derive(Debug, Clone)]
pub struct Var(pub TokenP, pub ExprP);
#[derive(Debug, Clone)]
pub struct While(pub ExprP, pub StmtP);

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Stmt {
    Block_(Block),
    Expression_(Expression),
    Fun_(Rc<Fun>),
    If_(If),
    Print_(Print),
    Return_(Return),
    Var_(Var),
    While_(While),

    Null,
    Dummy,
}

impl Stmt {
    pub fn block(statements: Vec<Self>) -> Self {
        Self::Block_(Block(statements))
    }

    pub fn expression(expr: Expr) -> Self {
        Self::Expression_(Expression(Box::new(expr)))
    }

    pub fn fun(name: Token, params: Vec<Token>, body: Statements) -> Self {
        Self::Fun_(Rc::new(Fun(Box::new(name), params, body)))
    }

    pub fn if_stmt(condition: Expr, then_branch: Self, else_branch: Option<Self>) -> Self {
        Self::If_(If(
            Box::new(condition),
            Box::new(then_branch),
            else_branch.map(|e| Box::new(e)),
        ))
    }

    pub fn print(expr: Expr) -> Self {
        Self::Print_(Print(Box::new(expr)))
    }

    pub fn return_stmt(keyword: Token, value: Expr) -> Self {
        Self::Return_(Return(Box::new(keyword), Box::new(value)))
    }

    pub fn var(name: Token, initializer: Expr) -> Self {
        Self::Var_(Var(Box::new(name), Box::new(initializer)))
    }

    pub fn while_stmt(condition: Expr, body: Self) -> Self {
        Self::While_(While(Box::new(condition), Box::new(body)))
    }

    pub fn null() -> Self {
        Self::Null
    }
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut T) -> <T as StmtVisitor>::R
    where
        T: StmtVisitor,
    {
        use Stmt::*;
        match self {
            Block_(stmt) => visitor.visit_block(stmt),
            Expression_(stmt) => visitor.visit_expression(stmt),
            Fun_(stmt) => visitor.visit_fun(stmt),
            If_(stmt) => visitor.visit_if(stmt),
            Print_(stmt) => visitor.visit_print(stmt),
            Return_(stmt) => visitor.visit_return(stmt),
            Var_(stmt) => visitor.visit_var(stmt),
            While_(stmt) => visitor.visit_while(stmt),
            Null => visitor.visit_null(),
            _ => panic!("[BUG] invalid type of expr {:?}.", self),
        }
    }
}
