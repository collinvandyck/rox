use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum EnvError {
    #[error("undefined variable '{}'", token.lexeme)]
    Undefined { token: Token },

    #[error("undefined variable in assign '{}'", name)]
    UndefinedAssign { name: String },
}

#[derive(Clone)]
pub struct Env {
    parent: Option<Rc<RefCell<EnvInner>>>,
    inner: Rc<RefCell<EnvInner>>,
}

impl Env {
    pub fn define(&mut self, name: impl AsRef<str>, val: Literal) {
        self.inner
            .borrow_mut()
            .vars
            .insert(name.as_ref().to_string(), val);
    }

    pub fn get(&self, token: &Token) -> Result<Literal, EnvError> {
        todo!()
    }
}

pub struct EnvInner {
    vars: HashMap<String, Literal>,
}
