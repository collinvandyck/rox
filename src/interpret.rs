use std::{collections::HashMap, ops::Neg};

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
pub struct Interpreter {
    vars: HashMap<Token, Literal>,
}

impl Interpreter {
    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), Error> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), Error> {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Literal, Error> {
        expr.accept(self)
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<(), Error>;
    fn visit_expr(&mut self, expr: &ExprStmt) -> Self::Output {
        self.evaluate(&expr.expr)?;
        Ok(())
    }
    fn visit_print(&mut self, expr: &PrintStmt) -> Self::Output {
        let literal = self.evaluate(&expr.expr)?;
        println!("{}", literal.to_lox());
        Ok(())
    }
    fn visit_var(&mut self, expr: &VarStmt) -> Self::Output {
        todo!()
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Literal, Error>;

    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output {
        use TokenType::*;
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
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

    fn visit_var(&mut self, expr: &VarExpr) -> Self::Output {
        todo!()
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
        Ok(expr.value.clone())
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output {
        let right = self.evaluate(&expr.right)?;
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
        self.evaluate(&expr.expr)
    }
}
