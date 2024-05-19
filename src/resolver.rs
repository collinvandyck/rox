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

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

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

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_string(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_string(), true);
        }
    }
}

impl ExprVisitor for Resolver {
    type Output = Result<(), ResolverError>;

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Self::Output {
        Ok(())
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Self::Output {
        Ok(())
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Self::Output {
        Ok(())
    }

    fn visit_group_expr(&mut self, expr: &GroupExpr) -> Self::Output {
        Ok(())
    }

    fn visit_var_expr(&mut self, expr: &VarExpr) -> Self::Output {
        let valid = self
            .scopes
            .last()
            .and_then(|m| m.get(expr.name.lexeme.as_ref()))
            .copied()
            == Some(false);
        Ok(())
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Self::Output {
        Ok(())
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Self::Output {
        Ok(())
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> Self::Output {
        Ok(())
    }
}

impl StmtVisitor for Resolver {
    type Output = Result<(), ResolverError>;

    fn visit_expr_stmt(&mut self, stmt: &ExprStmt) -> Self::Output {
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Self::Output {
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Self::Output {
        self.declare(&stmt.name);
        if let Some(initializer) = &stmt.initializer {
            self.resolve_expr(initializer)?;
        }
        self.define(&stmt.name);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Self::Output {
        self.begin_scope();
        self.resolve_stmts(&stmt.statements);
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Self::Output {
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Output {
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Self::Output {
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Self::Output {
        Ok(())
    }
}
