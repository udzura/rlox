use super::token::{Literal, Token};

type ExprP = Box<Expr>;
type TokenP = Box<Token>;
type LteralP = Box<Literal>;

pub enum Expr {
    Binary(ExprP, TokenP, ExprP),
    Grouping(ExprP),
    Lit(LteralP),
    Unary(TokenP, ExprP),
}
