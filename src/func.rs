use crate::prelude::*;
use std::{rc::Rc, sync::Arc};

#[derive(thiserror::Error, Debug)]
pub enum CallableError {
    #[error("{0}")]
    Generic(Box<dyn std::error::Error>),
}

#[derive(Clone)]
pub enum Callable {
    Native {
        name: String,
        arity: usize,
        func: Rc<CallableFn>,
    },
}

type CallableFn = dyn Fn(&mut Interpreter, Vec<Value>) -> Result<Value, CallableError>;

impl Callable {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, CallableError> {
        match self {
            Self::Native { arity, func, .. } => func.as_ref()(interpreter, args),
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Self::Native { arity, .. } => *arity,
        }
    }
}
impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Callable::Native { name: n1, .. }, Callable::Native { name: n2, .. }) => n1 == n2,
        }
    }
}

impl std::fmt::Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native { name, arity, .. } => f
                .debug_struct("Native")
                .field("name", name)
                .field("arity", arity)
                .finish(),
        }
    }
}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native { name, .. } => write!(f, "<native fn {name}>"),
        }
    }
}
