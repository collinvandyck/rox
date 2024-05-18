use crate::prelude::*;
use std::io;
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
    Env(#[from] EnvError),

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
}

pub struct Interpreter {
    globals: Env, // we have to keep a cloned copy around so that we can make fn calls
    env: Env,
    writer: Box<dyn io::Write>,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut globals = Env::default();
        globals.define(
            "clock",
            Value::Function(Callable::Native(NativeCallable {
                name: "clock".to_string(),
                arity: 0,
                func: Rc::new(|interpreter: &mut Interpreter, args: Vec<Value>| {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map_err(|err| CallableError::Generic(err.into()))?;
                    Ok(Value::Number(now.as_secs_f64()))
                }),
            })),
        );
        let env = globals.clone().child();
        Self {
            globals,
            env,
            writer: Box::new(stdout()),
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
    pub fn with_writer(mut self, w: Box<dyn io::Write>) -> Self {
        self.writer = w;
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

    pub fn new_env(&self) -> Env {
        self.env.clone().child()
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
    fn writer(&mut self) -> &mut dyn io::Write {
        self.writer.as_mut()
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
        writeln!(self.writer(), "{}", literal.to_lox()).map_err(Error::Print)?;
        Ok(())
    }
    fn visit_var_stmt(&mut self, expr: &VarStmt) -> Self::Output {
        let value: Value = expr
            .initializer
            .as_ref()
            .map(|i| self.evaluate(i))
            .transpose()?
            .unwrap_or(Value::Undefined);
        self.env.define(expr.name.lexeme.as_ref(), value);
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
            stmt.name.lexeme.as_ref(),
            Value::Function(Callable::LoxFunction(LoxFunction {
                stmt: stmt.clone().into(),
            })),
        );
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
                    LessEqual => (left < right).into(),
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
        Ok(func.call(self, args)?)
    }
}
