use std::ops::Neg;

use crate::prelude::*;

pub struct Interpreter {}

impl ExprVisitor for Interpreter {
    type Output = Value;

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output {
        use TokenType::*;
        let left = self.eval_expr(&expr.left).number();
        let right = self.eval_expr(&expr.right).number();
        match expr.op.typ {
            Minus => (left - right).into(),
            Slash => (left / right).into(),
            Star => (left * right).into(),
            _ => unreachable!(),
        }
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
        expr.value.clone().into()
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output {
        let right = self.eval_expr(&expr.right);
        match expr.op.typ {
            TokenType::Minus => {
                return right.map_number(Neg::neg);
            }
            TokenType::Bang => {
                return right.negate();
            }
            v => panic!("invalid op type: {v}"),
        }
        unreachable!()
    }

    fn visit_group(&mut self, expr: &GroupExpr) -> Self::Output {
        self.eval_expr(&expr.expr)
    }
}

#[derive(derive_more::From, derive_more::Display)]
pub struct Value {
    lit: Literal,
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value {
            lit: Literal::Number(value),
        }
    }
}

impl Value {
    fn negate(&self) -> Self {
        Self::from(Literal::Bool(!self.truthy()))
    }

    fn number(&self) -> f64 {
        if let Literal::Number(v) = self.lit {
            v
        } else {
            panic!("not a number")
        }
    }

    fn truthy(&self) -> bool {
        match self.lit {
            Literal::Number(_) | Literal::String(_) => true,
            Literal::Bool(b) => b,
            Literal::Nil => false,
        }
    }

    fn map_number(&self, f: impl Fn(f64) -> f64) -> Self {
        match self.lit {
            Literal::Number(val) => Self::from(Literal::Number(f(val))),
            _ => panic!("invalid value {self} for map_number"),
        }
    }
}
