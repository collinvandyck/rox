use crate::{prelude::*, Literal};

trait Expr {
    fn accept<V, O>(&self, visitor: &mut V) -> O
    where
        V: ExprVisitor<Output = O>;
}

trait ExprVisitor {
    type Output;
    fn visit_binary(&mut self, expr: &BinaryExpr<impl Expr, impl Expr>) -> Self::Output {
        unimplemented!()
    }
    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
        unimplemented!()
    }
    fn visit_unary(&mut self, expr: &UnaryExpr<impl Expr>) -> Self::Output {
        unimplemented!()
    }
    fn visit_grouping(&mut self, expr: &GroupExpr<impl Expr>) -> Self::Output {
        unimplemented!()
    }
}

#[derive(Default)]
struct AstPrinter {}

impl AstPrinter {
    pub fn print(mut self, expr: impl Expr) -> String {
        expr.accept(&mut self)
    }
}

impl ExprVisitor for AstPrinter {
    type Output = String;

    fn visit_grouping(&mut self, expr: &GroupExpr<impl Expr>) -> Self::Output {
        format!("( {} )", expr.accept(self))
    }
    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
        expr.lit.to_string()
    }
    fn visit_binary(&mut self, expr: &BinaryExpr<impl Expr, impl Expr>) -> Self::Output {
        format!(
            "( {} {} {})",
            expr.op.lexeme,
            expr.left.accept(self),
            expr.right.accept(self)
        )
    }
    fn visit_unary(&mut self, expr: &UnaryExpr<impl Expr>) -> Self::Output {
        format!("( {} {} )", expr.op.lexeme, expr.right.accept(self))
    }
}

#[derive(Default)]
struct BoolVisitor;
impl ExprVisitor for BoolVisitor {
    type Output = bool;

    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output {
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
    let mut visitor = BoolVisitor::default();
    let val = expr.accept(&mut visitor);
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
    fn accept<V, O>(&self, visitor: &mut V) -> O
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
    fn accept<V, O>(&self, visitor: &mut V) -> O
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
    fn accept<V, O>(&self, visitor: &mut V) -> O
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
    fn accept<V, O>(&self, visitor: &mut V) -> O
    where
        V: ExprVisitor<Output = O>,
    {
        visitor.visit_grouping(&self)
    }
}
