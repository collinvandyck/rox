use crate::prelude::*;

macro_rules! stmt {
    ($($l:tt)+) => {
        #[derive(Clone, Debug, PartialEq)]
        $($l)*
    };
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(ExprStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
    Return(ReturnStmt),
    Class(ClassStmt),
}

stmt! {
pub struct ExprStmt {
    pub expr: Expr,
}}

stmt! {
pub struct PrintStmt {
    pub expr: Expr,
}}

stmt! {
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}}

stmt! {
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}}

stmt! {pub struct IfStmt {
    pub condition: Expr,
    pub then_stmt: Box<Stmt>,
    pub else_stmt: Option<Box<Stmt>>,
}}

stmt! {pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}}

stmt! {pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}}

stmt! {pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Expr,
}}

stmt! {pub struct ClassStmt {
    pub name: Token,
    pub methods :Vec<Stmt>,
}}

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
            Stmt::Return(s) => visitor.visit_return_stmt(s),
            Stmt::Class(s) => visitor.visit_class_stmt(s),
        }
    }
}

pub trait StmtVisitor {
    type Output;

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> Self::Output;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Self::Output;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Self::Output;
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Self::Output;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Self::Output;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Output;
    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Self::Output;
    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Self::Output;
    fn visit_class_stmt(&mut self, stmt: &ClassStmt) -> Self::Output;
}
