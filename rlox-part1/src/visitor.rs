use super::expr::*;

pub trait Visitor {
    type R;

    fn visit_binary(&self, expr: &Binary) -> Self::R;
    fn visit_grouping(&self, expr: &Grouping) -> Self::R;
    fn visit_literal(&self, expr: &Lit) -> Self::R;
    fn visit_unary(&self, expr: &Unary) -> Self::R;
}
