use crate::prelude::*;

#[derive(Debug)]
pub enum Stmt {
    Expr(ExprStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
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

#[derive(Debug)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

impl Stmt {
    pub fn accept<V, O>(&self, visitor: &mut V) -> O
    where
        V: StmtVisitor<Output = O>,
    {
        match self {
            Stmt::Expr(s) => visitor.visit_expr_stmt(s),
            Stmt::Print(s) => visitor.visit_print_stmt(s),
            Stmt::Var(s) => visitor.visit_var_stmt(s),
            Stmt::Block(s) => visitor.visit_block_stmt(s),
        }
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_expr_stmt(&mut self, expr: &ExprStmt) -> Self::Output;
    fn visit_print_stmt(&mut self, expr: &PrintStmt) -> Self::Output;
    fn visit_var_stmt(&mut self, expr: &VarStmt) -> Self::Output;
    fn visit_block_stmt(&mut self, expr: &BlockStmt) -> Self::Output;
}
