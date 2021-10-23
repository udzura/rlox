use super::expr::*;
use super::stmt::*;

pub trait ExprVisitor {
    type R;

    fn visit_binary(&self, expr: &Binary) -> Self::R;
    fn visit_grouping(&self, expr: &Grouping) -> Self::R;
    fn visit_literal(&self, expr: &Lit) -> Self::R;
    fn visit_unary(&self, expr: &Unary) -> Self::R;
}

pub trait StmtVisitor {
    type R;

    fn visit_expression(&self, stmt: &Expression) -> Self::R;
    fn visit_print(&self, stmt: &Print) -> Self::R;
    fn visit_var(&self, stmt: &Var) -> Self::R;
}
