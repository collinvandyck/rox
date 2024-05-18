use crate::prelude::*;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum ResolverError {}

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::default(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::default());
    }

    fn end_scope(&mut self) {}

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ResolverError> {
        expr.accept(self)?;
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), ResolverError> {
        stmt.accept(self)?;
        Ok(())
    }

    fn resolve_stmts(&mut self, statements: &[Stmt]) -> Result<(), ResolverError> {
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }
}

impl ExprVisitor for Resolver {
    type Output = Result<(), ResolverError>;

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Self::Output {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Self::Output {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Self::Output {
        todo!()
    }

    fn visit_group_expr(&mut self, expr: &GroupExpr) -> Self::Output {
        todo!()
    }

    fn visit_var_expr(&mut self, expr: &VarExpr) -> Self::Output {
        todo!()
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Self::Output {
        todo!()
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Self::Output {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> Self::Output {
        todo!()
    }
}

impl StmtVisitor for Resolver {
    type Output = Result<(), ResolverError>;

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> Self::Output {
        todo!()
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Self::Output {
        todo!()
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Self::Output {
        todo!()
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Self::Output {
        self.begin_scope();
        self.resolve_stmts(&stmt.statements);
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Self::Output {
        todo!()
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Output {
        todo!()
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Self::Output {
        todo!()
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Self::Output {
        todo!()
    }
}
