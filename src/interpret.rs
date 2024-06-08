use crate::env::Env;
use crate::prelude::*;
use std::io::{self, stderr};
use std::time::Instant;
use std::{cell::RefCell, collections::HashMap, io::stdout, ops::Neg, rc::Rc};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("expected numbers for op: {op}")]
    NumbersRequired { op: Token },

    #[error("expected two numbers or two strings for op: {op}")]
    TwoNumbersOrStringsRequired { op: Token },

    #[error("invalid op: {op} for binary expr")]
    InvalidBinaryOp { op: Token },

    #[error("divide by zero detected at line {line}")]
    DivideByZero { line: usize },

    #[error(transparent)]
    Env(#[from] env::EnvError),

    #[error("cannot evaluate undefined var {}", token.lexeme)]
    UndfinedVar { token: Token },

    #[error("line {}: can only call functions and classes", token.line)]
    NotAFunction { token: Token },

    #[error("line {}: expected {expected} args but got {actual}", token.line)]
    FunctionArity {
        token: Token,
        expected: usize,
        actual: usize,
    },

    #[error(transparent)]
    CallableError(#[from] CallableError),

    #[error("could not print: {0}")]
    Print(#[source] io::Error),

    #[error("not an actual error! used to unwind the call stack.")]
    Return(Value),

    #[error("can't return from top-level code.")]
    TopLevelReturn,
}

pub struct Interpreter {
    env: Env,
    stdout: Box<dyn io::Write>,
    stderr: Box<dyn io::Write>,
    fn_depth: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut env = Env::default();
        env.define(
            "clock",
            Value::Function(Callable::Native(NativeFunction {
                name: "clock".to_string(),
                arity: 0,
                func: Rc::new(|interpreter: &mut Interpreter, args: Vec<Value>| {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map_err(|err| CallableError::Generic(err.into()))?;
                    Ok(Value::Number(now.as_secs_f64()))
                }),
            })),
        )
        .unwrap();
        Self {
            env,
            stdout: Box::new(stdout()),
            stderr: Box::new(stderr()),
            fn_depth: 0,
        }
    }
}

impl Interpreter {
    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), Error> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, Error> {
        expr.accept(self)
    }

    pub fn with_stdout(mut self, w: Box<dyn io::Write>) -> Self {
        self.stdout = w;
        self
    }

    pub fn with_stderr(mut self, w: Box<dyn io::Write>) -> Self {
        self.stderr = w;
        self
    }

    pub fn execute_block(&mut self, stmts: &[Stmt]) -> Result<(), Error> {
        self.env.push();
        let res = (|| {
            for stmt in stmts {
                self.execute(stmt)?;
            }
            Ok(())
        })();
        self.env.pop()?;
        res
    }

    pub fn swap_env(&mut self, env: Env) -> Env {
        std::mem::replace(&mut self.env, env)
    }

    // replaces the env with the supplied one
    pub fn restore_env(&mut self, env: Env) {
        self.env = env;
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), Error> {
        stmt.accept(self)
    }

    fn stderr(&mut self) -> &mut dyn io::Write {
        self.stderr.as_mut()
    }

    fn stdout(&mut self) -> &mut dyn io::Write {
        self.stdout.as_mut()
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<(), Error>;
    fn visit_expr_stmt(&mut self, expr: &ExprStmt) -> Self::Output {
        self.evaluate(&expr.expr)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &PrintStmt) -> Self::Output {
        let literal = self.evaluate(&expr.expr)?;
        writeln!(self.stdout(), "{}", literal.to_lox()).map_err(Error::Print)?;
        Ok(())
    }

    fn visit_var_stmt(&mut self, expr: &VarStmt) -> Self::Output {
        let value: Value = expr
            .initializer
            .as_ref()
            .map(|i| self.evaluate(i))
            .transpose()?
            .unwrap_or(Value::Undefined);
        self.env.define(&expr.name, value)?;
        Ok(())
    }
    fn visit_block_stmt(&mut self, expr: &BlockStmt) -> Self::Output {
        self.execute_block(&expr.statements)
    }

    fn visit_if_stmt(&mut self, expr: &IfStmt) -> Self::Output {
        if self.evaluate(&expr.condition)?.truthy() {
            self.execute(&expr.then_stmt)?;
        } else if let Some(else_stmt) = &expr.else_stmt {
            self.execute(else_stmt)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, expr: &WhileStmt) -> Self::Output {
        while (self.evaluate(&expr.condition)?.truthy()) {
            self.execute(&expr.body)?;
        }
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Self::Output {
        self.env.define(
            &stmt.name,
            Value::Function(Callable::LoxFunction(LoxFunction {
                stmt: stmt.clone().into(),
                closure: self.env.clone(),
            })),
        )?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Self::Output {
        if self.fn_depth == 0 {
            return Err(Error::TopLevelReturn);
        }
        let value = self.evaluate(&stmt.value)?;
        Err(Error::Return(value))
    }

    fn visit_class_stmt(&mut self, stmt: &ClassStmt) -> Self::Output {
        self.env.define(&stmt.name, Value::Nil)?;
        let class = Class::new(&stmt.name);
        self.env.assign(&stmt.name, class)?;
        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Value, Error>;

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Self::Output {
        let val: Value = self.evaluate(&expr.value)?;
        self.env.assign(expr.name.lexeme.as_ref(), val.clone())?;
        Ok(val)
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Self::Output {
        use TokenType::*;
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let op = &expr.op;
        Ok(match op.typ {
            Minus | Slash | Star | Greater | GreaterEqual | Less | LessEqual => {
                let (Value::Number(left), Value::Number(right)) = (left, right) else {
                    return Err(Error::NumbersRequired { op: op.clone() });
                };
                match op.typ {
                    Minus => (left - right).into(),
                    Slash => {
                        if right == 0.0 {
                            return Err(Error::DivideByZero { line: op.line });
                        }
                        (left / right).into()
                    }
                    Star => (left * right).into(),
                    Greater => (left > right).into(),
                    GreaterEqual => (left >= right).into(),
                    Less => (left < right).into(),
                    LessEqual => (left <= right).into(),
                    _ => unreachable!(),
                }
            }
            Plus => match (left, right) {
                (Value::Number(left), Value::Number(right)) => (left + right).into(),
                (Value::String(left), Value::String(right)) => format!("{left}{right}").into(),
                _ => {
                    return Err(Error::NumbersRequired {
                        op: expr.op.clone(),
                    })
                }
            },
            BangEqual => (left != right).into(),
            EqualEqual => (left == right).into(),
            _ => {
                return Err(Error::InvalidBinaryOp {
                    op: expr.op.clone(),
                })
            }
        })
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Self::Output {
        let left = self.evaluate(&expr.left)?;
        let truthy = left.truthy();
        if let TokenType::Or = expr.op.typ {
            if truthy {
                return Ok(left);
            }
        } else if !truthy {
            return Ok(left);
        }
        self.evaluate(&expr.right)
    }

    fn visit_var_expr(&mut self, expr: &VarExpr) -> Self::Output {
        let val = self.env.get(&expr.name)?;
        if let Value::Undefined = val {
            return Err(Error::UndfinedVar {
                token: expr.name.clone(),
            });
        }
        Ok(self.env.get(&expr.name)?)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Self::Output {
        Ok(expr.value.clone())
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Self::Output {
        let right = self.evaluate(&expr.right)?;
        Ok(match expr.op.typ {
            TokenType::Minus => {
                let Value::Number(right) = &right else {
                    return Err(Error::NumbersRequired {
                        op: expr.op.clone(),
                    });
                };
                (-right).into()
            }
            TokenType::Bang => (!right.truthy()).into(),
            _ => unreachable!(),
        })
    }

    fn visit_group_expr(&mut self, expr: &GroupExpr) -> Self::Output {
        self.evaluate(&expr.expr)
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> Self::Output {
        let callee = self.evaluate(&expr.callee)?;
        let Value::Function(func) = callee else {
            return Err(Error::NotAFunction {
                token: expr.paren.clone(),
            });
        };
        let args = expr
            .args
            .iter()
            .map(|arg| self.evaluate(arg))
            .collect::<Result<Vec<_>, _>>()?;
        let arity = func.arity();
        if args.len() != arity {
            return Err(Error::FunctionArity {
                token: expr.paren.clone(),
                expected: arity,
                actual: args.len(),
            });
        }
        self.fn_depth += 1;
        let fn_res = func.call(self, args);
        self.fn_depth -= 1;
        Ok(fn_res?)
    }
}
