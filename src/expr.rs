use crate::prelude::*;

trait Expr {
    fn visit<V, O>(&self, visitor: V) -> O
    where
        V: Visitor<Output = O>;
}

trait Visitor {
    type Output;
    fn visit_binary(&self, expr: &Binary<impl Expr, impl Expr>) -> Self::Output {
        unimplemented!()
    }
    fn visit_literal(&self, expr: &Literal) -> Self::Output {
        unimplemented!()
    }
    fn visit_unary(&self, expr: &Unary<impl Expr>) -> Self::Output {
        unimplemented!()
    }
    fn visit_grouping(&self, expr: &Grouping<impl Expr>) -> Self::Output {
        unimplemented!()
    }
}

struct BoolVisitor {}
impl Visitor for BoolVisitor {
    type Output = bool;

    fn visit_binary(&self, expr: &Binary<impl Expr, impl Expr>) -> Self::Output {
        false
    }
    fn visit_literal(&self, expr: &Literal) -> Self::Output {
        false
    }
    fn visit_unary(&self, expr: &Unary<impl Expr>) -> Self::Output {
        false
    }
    fn visit_grouping(&self, expr: &Grouping<impl Expr>) -> Self::Output {
        false
    }
}

struct Binary<L, R> {
    left: L,
    op: Token,
    right: R,
}

impl<L, R> Binary<L, R>
where
    L: Expr,
    R: Expr,
{
    fn new(left: L, op: Token, right: R) -> Self {
        Self { left, op, right }
    }
}

impl<L, R> Expr for Binary<L, R>
where
    L: Expr,
    R: Expr,
{
    fn visit<V, O>(&self, visitor: V) -> O
    where
        V: Visitor<Output = O>,
    {
        visitor.visit_binary(&self)
    }
}

struct Literal {
    lit: crate::Literal,
}

impl Literal {
    fn new(lit: crate::Literal) -> Self {
        Self { lit }
    }
}

impl Expr for Literal {
    fn visit<V, O>(&self, visitor: V) -> O
    where
        V: Visitor<Output = O>,
    {
        visitor.visit_literal(&self)
    }
}

struct Unary<R> {
    op: Token,
    right: R,
}

impl<R> Unary<R>
where
    R: Expr,
{
    fn new(op: Token, right: R) -> Self {
        Self { op, right }
    }
}

impl<R> Expr for Unary<R>
where
    R: Expr,
{
    fn visit<V, O>(&self, visitor: V) -> O
    where
        V: Visitor<Output = O>,
    {
        visitor.visit_unary(&self)
    }
}

struct Grouping<E> {
    expr: E,
}

impl<E> Grouping<E>
where
    E: Expr,
{
    fn new(expr: E) -> Self {
        Grouping { expr }
    }
}

impl<E> Expr for Grouping<E>
where
    E: Expr,
{
    fn visit<V, O>(&self, visitor: V) -> O
    where
        V: Visitor<Output = O>,
    {
        visitor.visit_grouping(&self)
    }
}
