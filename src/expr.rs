use crate::{prelude::*, Literal};

trait Expr {
    fn visit<V, O>(&self, visitor: &mut V) -> O
    where
        V: ExprVisitor<Output = O>;
}

trait ExprVisitor {
    type Output;
    fn visit_binary(&self, expr: &BinaryExpr<impl Expr, impl Expr>) -> Self::Output {
        unimplemented!()
    }
    fn visit_literal(&self, expr: &LiteralExpr) -> Self::Output {
        unimplemented!()
    }
    fn visit_unary(&self, expr: &UnaryExpr<impl Expr>) -> Self::Output {
        unimplemented!()
    }
    fn visit_grouping(&self, expr: &GroupExpr<impl Expr>) -> Self::Output {
        unimplemented!()
    }
}

struct BoolVisitor {}
impl ExprVisitor for BoolVisitor {
    type Output = bool;

    fn visit_literal(&self, expr: &LiteralExpr) -> Self::Output {
        if let Literal::Bool(v) = expr.lit {
            v
        } else {
            false
        }
    }
}

#[test]
fn test_bool_visitor() {
    let expr = LiteralExpr {
        lit: Literal::Bool(true),
    };
    let mut visitor = BoolVisitor {};
    let val = expr.visit(&mut visitor);
    assert_eq!(val, true);
}

struct BinaryExpr<L, R> {
    left: L,
    op: Token,
    right: R,
}

impl<L, R> BinaryExpr<L, R>
where
    L: Expr,
    R: Expr,
{
    fn new(left: L, op: Token, right: R) -> Self {
        Self { left, op, right }
    }
}

impl<L, R> Expr for BinaryExpr<L, R>
where
    L: Expr,
    R: Expr,
{
    fn visit<V, O>(&self, visitor: &mut V) -> O
    where
        V: ExprVisitor<Output = O>,
    {
        visitor.visit_binary(&self)
    }
}

struct LiteralExpr {
    lit: Literal,
}

impl LiteralExpr {
    fn new(lit: Literal) -> Self {
        Self { lit }
    }
}

impl Expr for LiteralExpr {
    fn visit<V, O>(&self, visitor: &mut V) -> O
    where
        V: ExprVisitor<Output = O>,
    {
        visitor.visit_literal(&self)
    }
}

struct UnaryExpr<R> {
    op: Token,
    right: R,
}

impl<R> UnaryExpr<R>
where
    R: Expr,
{
    fn new(op: Token, right: R) -> Self {
        Self { op, right }
    }
}

impl<R> Expr for UnaryExpr<R>
where
    R: Expr,
{
    fn visit<V, O>(&self, visitor: &mut V) -> O
    where
        V: ExprVisitor<Output = O>,
    {
        visitor.visit_unary(&self)
    }
}

struct GroupExpr<E> {
    expr: E,
}

impl<E> GroupExpr<E>
where
    E: Expr,
{
    fn new(expr: E) -> Self {
        GroupExpr { expr }
    }
}

impl<E> Expr for GroupExpr<E>
where
    E: Expr,
{
    fn visit<V, O>(&self, visitor: &mut V) -> O
    where
        V: ExprVisitor<Output = O>,
    {
        visitor.visit_grouping(&self)
    }
}
