use crate::prelude::*;

#[derive(Debug)]
pub enum Stmt {
    Expr(ExprStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
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

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_stmt: Box<Stmt>,
    pub else_stmt: Option<Box<Stmt>>,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug)]
pub struct ForStmt {}

impl Stmt {
    pub fn accept<Out>(&self, visitor: &mut impl StmtVisitor<Output = Out>) -> Out {
        match self {
            Stmt::Expr(s) => visitor.visit_expr_stmt(s),
            Stmt::Print(s) => visitor.visit_print_stmt(s),
            Stmt::Var(s) => visitor.visit_var_stmt(s),
            Stmt::Block(s) => visitor.visit_block_stmt(s),
            Stmt::If(s) => visitor.visit_if_stmt(s),
            Stmt::While(s) => visitor.visit_while_stmt(s),
            Stmt::For(s) => visitor.visit_for_stmt(s),
        }
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_expr_stmt(&mut self, expr: &ExprStmt) -> Self::Output;
    fn visit_print_stmt(&mut self, expr: &PrintStmt) -> Self::Output;
    fn visit_var_stmt(&mut self, expr: &VarStmt) -> Self::Output;
    fn visit_block_stmt(&mut self, expr: &BlockStmt) -> Self::Output;
    fn visit_if_stmt(&mut self, expr: &IfStmt) -> Self::Output;
    fn visit_while_stmt(&mut self, expr: &WhileStmt) -> Self::Output;
    fn visit_for_stmt(&mut self, expr: &ForStmt) -> Self::Output;
}
