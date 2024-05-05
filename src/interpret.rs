use crate::prelude::*;

pub struct Interpreter {}

impl ExprVisitor for Interpreter {
    type Output = Literal;

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output {
        todo!()
    }
    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
        expr.value.clone()
    }
    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output {
        todo!()
    }
    fn visit_group(&mut self, expr: &GroupExpr) -> Self::Output {
        todo!()
    }
}
