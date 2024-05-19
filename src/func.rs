use crate::prelude::*;
use std::{fmt::Pointer, rc::Rc, sync::Arc};

#[derive(thiserror::Error, Debug)]
pub enum CallableError {
    #[error("{0}")]
    Generic(Box<dyn std::error::Error>),

    #[error("call: {0}")]
    Call(Box<interpret::Error>),
}

#[derive(Clone)]
pub enum Callable {
    Native(NativeCallable),
    LoxFunction(LoxFunction),
}

#[derive(Clone)]
pub struct LoxFunction {
    pub stmt: Box<FunctionStmt>,
    pub closure: tree::Env,
}

#[derive(Clone)]
pub struct NativeCallable {
    pub name: String,
    pub arity: usize,
    pub func: Rc<dyn Fn(&mut Interpreter, Vec<Value>) -> Result<Value, CallableError>>,
}

impl Callable {
    pub fn call(&self, int: &mut Interpreter, args: Vec<Value>) -> Result<Value, CallableError> {
        match self {
            Self::Native(NativeCallable { func, .. }) => func(int, args),
            Self::LoxFunction(LoxFunction { stmt, closure }) => {
                assert_eq!(stmt.params.len(), args.len());
                let mut env = closure.child();
                for (param, arg) in stmt.params.iter().zip(args.iter()) {
                    env.define(param.lexeme.as_ref(), arg.clone());
                }
                let env = int.swap_env(env);
                let res = int.execute_block(&stmt.body);
                int.restore_env(env);
                match res {
                    Ok(()) => Ok(Value::Nil),
                    Err(Error::Return(val)) => Ok(val),
                    Err(err) => Err(CallableError::Call(err.into())),
                }
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Self::Native(NativeCallable { arity, .. }) => *arity,
            Self::LoxFunction(func) => func.stmt.params.len(),
        }
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Callable::Native(n1), Callable::Native(n2)) => n1.name == n2.name,
            (Callable::LoxFunction(n1), Callable::LoxFunction(n2)) => n1.stmt == n2.stmt,
            _ => false,
        }
    }
}

impl std::fmt::Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(NativeCallable { name, arity, .. }) => f
                .debug_struct("Native")
                .field("name", name)
                .field("arity", arity)
                .finish(),
            Self::LoxFunction(func) => func.fmt(f),
        }
    }
}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(native) => write!(f, "<native fn {}>", native.name),
            Self::LoxFunction(func) => write!(f, "<fn {}>", func.stmt.name.lexeme),
        }
    }
}
