use std::ops::Neg;

use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("expected numbers for op: {op}")]
    NumbersRequired { op: Token },

    #[error("expected two numbers or two strings for op: {op}")]
    TwoNumbersOrStringsRequired { op: Token },

    #[error("invalid op: {op} for binary expr")]
    InvalidBinaryOp { op: Token },

    #[error("divide by zero detected at line {line}")]
    DivideByZero { line: usize },
}

#[derive(Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn interpret(expr: &Expr) -> Result<Literal, Error> {
        Self.eval_expr(expr)
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Literal, Error>;

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output {
        use TokenType::*;
        let left = self.eval_expr(&expr.left)?;
        let right = self.eval_expr(&expr.right)?;
        let op = &expr.op;
        Ok(match op.typ {
            Minus | Slash | Star | Greater | GreaterEqual | Less | LessEqual => {
                let (Literal::Number(left), Literal::Number(right)) = (left, right) else {
                    return Err(Error::NumbersRequired { op: op.clone() });
                };
                match op.typ {
                    Minus => (left - right).into(),
                    Slash => {
                        if right == 0.0 {
                            return Err(Error::DivideByZero { line: op.line });
                        }
                        (left / right).into()
                    }
                    Star => (left * right).into(),
                    Greater => (left > right).into(),
                    GreaterEqual => (left >= right).into(),
                    Less => (left < right).into(),
                    LessEqual => (left < right).into(),
                    _ => unreachable!(),
                }
            }
            Plus => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => (left + right).into(),
                (Literal::String(left), Literal::String(right)) => format!("{left}{right}").into(),
                _ => {
                    return Err(Error::NumbersRequired {
                        op: expr.op.clone(),
                    })
                }
            },
            BangEqual => (left != right).into(),
            EqualEqual => (left == right).into(),
            _ => {
                return Err(Error::InvalidBinaryOp {
                    op: expr.op.clone(),
                })
            }
        })
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
        Ok(expr.value.clone())
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output {
        let right = self.eval_expr(&expr.right)?;
        Ok(match expr.op.typ {
            TokenType::Minus => {
                let Literal::Number(right) = &right else {
                    return Err(Error::NumbersRequired {
                        op: expr.op.clone(),
                    });
                };
                (-right).into()
            }
            TokenType::Bang => (!right.truthy()).into(),
            _ => unreachable!(),
        })
    }

    fn visit_group(&mut self, expr: &GroupExpr) -> Self::Output {
        self.eval_expr(&expr.expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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
            let tokens = Scanner::new(prog).scan_tokens().unwrap();
            let expr = Parser::new(tokens).parse().unwrap();
            let lit = Interpreter::interpret(&expr).unwrap();
            assert_eq!(lit, ex, "expected {prog} to evaluate to {ex}");
        }
    }
}
