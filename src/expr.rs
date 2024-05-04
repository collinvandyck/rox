use crate::prelude::*;

trait Expr {}

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
}

struct Literal {
    lit: crate::Literal,
}

impl Literal {
    fn new(lit: crate::Literal) -> Self {
        Self { lit }
    }
}

impl Expr for Literal {}

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

impl<R> Expr for Unary<R> where R: Expr {}

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

impl<E> Expr for Grouping<E> where E: Expr {}
