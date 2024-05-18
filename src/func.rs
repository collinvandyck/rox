use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum CallableError {}

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}

impl Callable {
    pub fn call(&self, interpreter: &mut Interpreter) -> Result<Value, CallableError> {
        todo!()
    }

    pub fn arity(&self) -> usize {
        todo!()
    }
}
