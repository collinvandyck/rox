use crate::prelude::*;

#[derive(Debug)]
pub enum Stmt {
    Expr(ExprStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

#[derive(Debug)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl Stmt {
    pub fn accept<V, O>(&self, visitor: &mut V) -> O
    where
        V: StmtVisitor<Output = O>,
    {
        match self {
            Stmt::Expr(s) => visitor.visit_expr(s),
            Stmt::Print(s) => visitor.visit_print(s),
            Stmt::Var(s) => visitor.visit_var(s),
        }
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_expr(&mut self, expr: &ExprStmt) -> Self::Output;
    fn visit_print(&mut self, expr: &PrintStmt) -> Self::Output;
    fn visit_var(&mut self, expr: &VarStmt) -> Self::Output;
}
