use crate::{prelude::*, Literal};

pub enum Expr {
    Binary(Box<EnumBinary>),
    Literal(Box<EnumLiteral>),
    Unary(Box<EnumUnary>),
    Group(Box<EnumGroup>),
}

impl Expr {
    pub fn binary(left: Expr, op: Token, right: Expr) -> Self {
        Self::Binary(EnumBinary { left, op, right }.into())
    }
    pub fn literal(literal: Literal) -> Self {
        Self::Literal(EnumLiteral { lit: literal }.into())
    }
    pub fn unary(op: Token, right: Expr) -> Self {
        Self::Unary(EnumUnary { op, right }.into())
    }
    pub fn group(expr: Expr) -> Self {
        Self::Group(EnumGroup { expr }.into())
    }
}

pub struct EnumBinary {
    left: Expr,
    op: Token,
    right: Expr,
}

pub struct EnumLiteral {
    lit: Literal,
}

pub struct EnumUnary {
    op: Token,
    right: Expr,
}

pub struct EnumGroup {
    expr: Expr,
}

impl Expr {
    pub fn accept<V, O>(&self, visitor: &mut V) -> O
    where
        V: Visitor<Output = O>,
    {
        match self {
            Expr::Binary(b) => visitor.visit_binary(&b),
            Expr::Literal(l) => visitor.visit_literal(&l),
            Expr::Unary(u) => visitor.visit_unary(&u),
            Expr::Group(g) => visitor.visit_group(&g),
        }
    }
}

pub trait Visitor {
    type Output;
    fn visit_binary(&mut self, expr: &EnumBinary) -> Self::Output;
    fn visit_literal(&mut self, expr: &EnumLiteral) -> Self::Output;
    fn visit_unary(&mut self, expr: &EnumUnary) -> Self::Output;
    fn visit_group(&mut self, expr: &EnumGroup) -> Self::Output;
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
    fn visit_group(&mut self, expr: &EnumGroup) -> Self::Output {
        format!("( group {} )", expr.expr.accept(self))
    }
    fn visit_literal(&mut self, expr: &EnumLiteral) -> Self::Output {
        expr.lit.to_string()
    }
    fn visit_binary(&mut self, expr: &EnumBinary) -> Self::Output {
        format!(
            "( {} {} {})",
            expr.op.lexeme,
            expr.left.accept(self),
            expr.right.accept(self)
        )
    }
    fn visit_unary(&mut self, expr: &EnumUnary) -> Self::Output {
        format!("( {} {} )", expr.op.lexeme, expr.right.accept(self))
    }
}
