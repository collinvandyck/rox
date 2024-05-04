use crate::{prelude::*, Literal};

pub enum Expr {
    Binary(BinaryExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Group(GroupExpr),
}

impl Expr {
    pub fn binary(left: impl Into<Box<Expr>>, op: Token, right: impl Into<Box<Expr>>) -> Self {
        Self::Binary(BinaryExpr {
            left: left.into(),
            op,
            right: right.into(),
        })
    }
    pub fn literal(literal: impl Into<Literal>) -> Self {
        Self::Literal(LiteralExpr {
            lit: literal.into(),
        })
    }
    pub fn unary(op: Token, right: impl Into<Box<Expr>>) -> Self {
        Self::Unary(UnaryExpr {
            op,
            right: right.into(),
        })
    }
    pub fn group(expr: impl Into<Box<Expr>>) -> Self {
        Self::Group(GroupExpr { expr: expr.into() })
    }
}

pub struct BinaryExpr {
    left: Box<Expr>,
    op: Token,
    right: Box<Expr>,
}

pub struct LiteralExpr {
    lit: Literal,
}

pub struct UnaryExpr {
    op: Token,
    right: Box<Expr>,
}

pub struct GroupExpr {
    expr: Box<Expr>,
}

impl Expr {
    pub fn accept<V, O>(&self, visitor: &mut V) -> O
    where
        V: Visitor<Output = O>,
    {
        match self {
            Expr::Binary(b) => visitor.visit_binary(b),
            Expr::Literal(l) => visitor.visit_literal(l),
            Expr::Unary(u) => visitor.visit_unary(u),
            Expr::Group(g) => visitor.visit_group(g),
        }
    }
}

pub trait Visitor {
    type Output;
    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output;
    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output;
    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output;
    fn visit_group(&mut self, expr: &GroupExpr) -> Self::Output;
}

#[derive(Default)]
pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(mut self, expr: &Expr) -> String {
        expr.accept(&mut self)
    }
}

impl Visitor for AstPrinter {
    type Output = String;
    fn visit_group(&mut self, expr: &GroupExpr) -> Self::Output {
        format!("( group {} )", expr.expr.accept(self))
    }
    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
        expr.lit.to_string()
    }
    fn visit_binary(&mut self, expr: &BinaryExpr) -> Self::Output {
        format!(
            "( {} {} {})",
            expr.op.lexeme,
            expr.left.accept(self),
            expr.right.accept(self)
        )
    }
    fn visit_unary(&mut self, expr: &UnaryExpr) -> Self::Output {
        format!("( {} {} )", expr.op.lexeme, expr.right.accept(self))
    }
}
