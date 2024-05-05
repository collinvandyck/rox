use crate::prelude::*;

pub struct Interpreter {}

pub enum Value {}

impl ExprVisitor for Interpreter {
    type Output = Value;

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output {
        todo!()
    }
    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
        todo!()
    }
    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output {
        todo!()
    }
    fn visit_group(&mut self, expr: &GroupExpr) -> Self::Output {
        todo!()
    }
}
