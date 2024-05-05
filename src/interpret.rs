use std::ops::Neg;

use crate::prelude::*;

#[derive(Default)]
pub struct Interpreter;

impl ExprVisitor for Interpreter {
    type Output = Literal;

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output {
        use TokenType::*;
        let left = self.eval_expr(&expr.left);
        let right = self.eval_expr(&expr.right);
        match expr.op.typ {
            Minus => (left.num() - right.num()).into(),
            Slash => (left.num() / right.num()).into(),
            Star => (left.num() * right.num()).into(),
            Plus => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => (left + right).into(),
                (Literal::String(left), Literal::String(right)) => format!("{left}{right}").into(),
                _ => unreachable!(),
            },
            Greater => (left.num() > right.num()).into(),
            GreaterEqual => (left.num() >= right.num()).into(),
            Less => (left.num() < right.num()).into(),
            LessEqual => (left.num() <= right.num()).into(),
            BangEqual => (left != right).into(),
            EqualEqual => (left == right).into(),
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
            _ => unreachable!(),
        }
    }

    fn visit_group(&mut self, expr: &GroupExpr) -> Self::Output {
        self.eval_expr(&expr.expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse::Parser, ExprVisitor, Literal, Scanner};

    use super::Interpreter;

    #[test]
    fn test_interpret_expr() {
        for (prog, ex) in [
            ("3", Literal::Number(3.0)),
            ("3 + 4", Literal::Number(7.0)),
            ("3 * 4", Literal::Number(12.0)),
            ("4 / 2", Literal::Number(2.0)),
            ("(1 + 3) * 5", Literal::Number(20.0)),
            (r#""Coll" + "in""#, Literal::String(String::from("Collin"))),
            (r#" ! "Col""#, Literal::Bool(false)),
            (r#" !! "Col""#, Literal::Bool(true)),
        ] {
            //
            let tokens = Scanner::new(&prog).scan_tokens().unwrap();
            let expr = Parser::new(tokens).parse().unwrap();
            let lit = Interpreter::default().eval_expr(&expr);
            assert_eq!(lit, ex, "expected {prog} to evaluate to {ex}");
        }
    }
}
