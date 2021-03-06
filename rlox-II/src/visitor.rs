use std::rc::Rc;

use super::expr::*;
use super::stmt::*;

pub trait ExprVisitor {
    type R;

    fn visit_assign(&mut self, expr: &Assign) -> Self::R;
    fn visit_binary(&mut self, expr: &Binary) -> Self::R;
    fn visit_call(&mut self, expr: &Call) -> Self::R;
    fn visit_get(&mut self, expr: &Get) -> Self::R;
    fn visit_grouping(&mut self, expr: &Grouping) -> Self::R;
    fn visit_literal(&mut self, expr: &Lit) -> Self::R;
    fn visit_logical(&mut self, expr: &Logical) -> Self::R;
    fn visit_set(&mut self, expr: &Set) -> Self::R;
    fn visit_super(&mut self, expr: &Super) -> Self::R;
    fn visit_this(&mut self, expr: &This) -> Self::R;
    fn visit_unary(&mut self, expr: &Unary) -> Self::R;
    fn visit_variable(&mut self, expr: &Variable) -> Self::R;

    fn visit_null(&mut self) -> Self::R;
}

pub trait StmtVisitor {
    type R;

    fn visit_block(&mut self, stmt: &Block) -> Self::R;
    fn visit_class(&mut self, stmt: &Class) -> Self::R;
    fn visit_expression(&mut self, stmt: &Expression) -> Self::R;
    fn visit_fun(&mut self, stmt: &Rc<Fun>) -> Self::R;
    fn visit_if(&mut self, stmt: &If) -> Self::R;
    fn visit_print(&mut self, stmt: &Print) -> Self::R;
    fn visit_return(&mut self, stmt: &Return) -> Self::R;
    fn visit_var(&mut self, stmt: &Var) -> Self::R;
    fn visit_while(&mut self, stmt: &While) -> Self::R;

    fn visit_null(&mut self) -> Self::R;
}
