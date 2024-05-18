use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(ExprStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrintStmt {
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_stmt: Box<Stmt>,
    pub else_stmt: Option<Box<Stmt>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl Stmt {
    pub fn accept<Out>(&self, visitor: &mut impl StmtVisitor<Output = Out>) -> Out {
        match self {
            Stmt::Expr(s) => visitor.visit_expr_stmt(s),
            Stmt::Print(s) => visitor.visit_print_stmt(s),
            Stmt::Var(s) => visitor.visit_var_stmt(s),
            Stmt::Block(s) => visitor.visit_block_stmt(s),
            Stmt::If(s) => visitor.visit_if_stmt(s),
            Stmt::While(s) => visitor.visit_while_stmt(s),
            Stmt::Function(s) => visitor.visit_function_stmt(s),
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
    fn visit_function_stmt(&mut self, expr: &FunctionStmt) -> Self::Output;
}
