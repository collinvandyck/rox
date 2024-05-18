use crate::prelude::*;
use std::{fmt::Pointer, rc::Rc, sync::Arc};

#[derive(thiserror::Error, Debug)]
pub enum CallableError {
    #[error("{0}")]
    Generic(Box<dyn std::error::Error>),
}

#[derive(Clone)]
pub enum Callable {
    Native(NativeCallable),
    LoxFunction(LoxFunction),
}

trait CallableTrait {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>)
        -> Result<Value, CallableError>;
    fn arity(&self) -> usize;
}

#[derive(Clone)]
pub struct LoxFunction {
    pub stmt: Box<FunctionStmt>,
}

#[derive(Clone)]
pub struct NativeCallable {
    pub name: String,
    pub arity: usize,
    pub func: Rc<dyn Fn(&mut Interpreter, Vec<Value>) -> Result<Value, CallableError>>,
}

impl CallableTrait for NativeCallable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, CallableError> {
        self.func.as_ref()(interpreter, args)
    }
    fn arity(&self) -> usize {
        self.arity
    }
}

impl Callable {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, CallableError> {
        match self {
            Self::Native(native) => native.call(interpreter, args),
            Self::LoxFunction(func) => {
                //
                todo!()
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Self::Native(func) => func.arity(),
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
            Self::LoxFunction(func) => write!(f, "<lox fn {}>", func.stmt.name),
        }
    }
}
