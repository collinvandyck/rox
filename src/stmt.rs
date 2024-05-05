use crate::prelude::*;

#[derive(Debug)]
pub enum Stmt {
    Expr(ExprStmt),
    Print(PrintStmt),
}

#[derive(Debug)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expr: Expr,
}

impl Stmt {
    pub fn accept<V, O>(&self, visitor: &mut V) -> O
    where
        V: StmtVisitor<Output = O>,
    {
        match self {
            Stmt::Expr(s) => visitor.visit_expr_stmt(s),
            Stmt::Print(s) => visitor.visit_print_stmt(s),
        }
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_expr_stmt(&mut self, expr: &ExprStmt) -> Self::Output;
    fn visit_print_stmt(&mut self, expr: &PrintStmt) -> Self::Output;
}
