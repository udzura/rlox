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
pub struct If(pub ExprP, pub StmtP, pub Option<StmtP>);
#[derive(Debug, Clone)]
pub struct Print(pub ExprP);
#[derive(Debug, Clone)]
pub struct Var(pub TokenP, pub ExprP);
#[derive(Debug, Clone)]
pub struct While(pub ExprP, pub StmtP);

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Stmt {
    Block_(Block),
    Expression_(Expression),
    If_(If),
    Print_(Print),
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
    pub fn accept<T>(&self, visitor: &T) -> <T as StmtVisitor>::R
    where
        T: StmtVisitor,
    {
        use Stmt::*;
        match self {
            Block_(stmt) => visitor.visit_block(stmt),
            Expression_(stmt) => visitor.visit_expression(stmt),
            If_(stmt) => visitor.visit_if(stmt),
            Print_(stmt) => visitor.visit_print(stmt),
            Var_(stmt) => visitor.visit_var(stmt),
            While_(stmt) => visitor.visit_while(stmt),
            Null => visitor.visit_null(),
            _ => panic!("[BUG] invalid type of expr {:?}.", self),
        }
    }
}
