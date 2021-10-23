use super::expr::*;
use super::stmt::*;

pub trait ExprVisitor {
    type R;

    fn visit_assign(&self, expr: &Assign) -> Self::R;
    fn visit_binary(&self, expr: &Binary) -> Self::R;
    fn visit_grouping(&self, expr: &Grouping) -> Self::R;
    fn visit_literal(&self, expr: &Lit) -> Self::R;
    fn visit_unary(&self, expr: &Unary) -> Self::R;
    fn visit_variable(&self, expr: &Variable) -> Self::R;
}

pub trait StmtVisitor {
    type R;

    fn visit_block(&self, stmt: &Block) -> Self::R;
    fn visit_expression(&self, stmt: &Expression) -> Self::R;
    fn visit_print(&self, stmt: &Print) -> Self::R;
    fn visit_var(&self, stmt: &Var) -> Self::R;

    fn visit_null(&self) -> Self::R;
}
