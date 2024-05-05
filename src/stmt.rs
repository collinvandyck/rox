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

pub trait StmtVisitor {
    type Output;

    fn visit_print(&mut self, expr: &PrintStmt) -> Self::Output;
    fn visit_expr(&mut self, expr: &ExprStmt) -> Self::Output;
}
