use crate::{prelude::*, Literal};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Group(GroupExpr),
    Var(VarExpr),
    Assign(AssignExpr),
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
            value: literal.into(),
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

#[derive(Debug)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Literal,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct GroupExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct VarExpr {
    pub name: Token,
}

impl Expr {
    pub fn accept<Out>(&self, visitor: &mut impl ExprVisitor<Output = Out>) -> Out {
        match self {
            Expr::Binary(e) => visitor.visit_binary_expr(e),
            Expr::Literal(e) => visitor.visit_literal_expr(e),
            Expr::Unary(e) => visitor.visit_unary_expr(e),
            Expr::Group(e) => visitor.visit_group_epxr(e),
            Expr::Var(e) => visitor.visit_var_expr(e),
            Expr::Assign(e) => visitor.visit_assign_expr(e),
        }
    }
}

pub trait ExprVisitor {
    type Output;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Self::Output;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Self::Output;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Self::Output;
    fn visit_group_epxr(&mut self, expr: &GroupExpr) -> Self::Output;
    fn visit_var_expr(&mut self, expr: &VarExpr) -> Self::Output;
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Self::Output;
}
