use crate::prelude::*;

#[derive(Debug)]
pub enum Stmt {
    Expr(ExprStmt),
}

#[derive(Debug)]
pub struct ExprStmt {
    pub expr: Expr,
}
