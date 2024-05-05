use std::ops::Neg;

use crate::prelude::*;

pub struct Interpreter {}

impl ExprVisitor for Interpreter {
    type Output = Value;

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output {
        use TokenType::*;
        let left = self.eval_expr(&expr.left);
        let right = self.eval_expr(&expr.right);
        match expr.op.typ {
            Minus => (left.num() - right.num()).into(),
            Slash => (left.num() / right.num()).into(),
            Star => (left.num() * right.num()).into(),
            Plus => match (left.lit, right.lit) {
                (Literal::Number(left), Literal::Number(right)) => (left + right).into(),
                (Literal::String(left), Literal::String(right)) => format!("{left}{right}").into(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
        expr.value.clone().into()
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output {
        let right = self.eval_expr(&expr.right);
        match expr.op.typ {
            TokenType::Minus => (-right.num()).into(),
            TokenType::Bang => (!right.truthy()).into(),
            v => panic!("invalid op type: {v}"),
        }
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

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value {
            lit: Literal::Bool(value),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value {
            lit: Literal::String(value),
        }
    }
}

impl Value {
    fn num(&self) -> f64 {
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
}
