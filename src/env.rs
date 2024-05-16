use crate::prelude::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(thiserror::Error, Debug, strum_macros::EnumIs)]
pub enum EnvError {
    #[error("undefined variable in assign '{}'", name)]
    UndefinedAssign { name: String },

    #[error("undefined variable '{}'", token.lexeme)]
    NotFound { token: Token },
}

#[derive(Clone, Default)]
pub struct Env {
    parent: Option<Rc<RefCell<EnvInner>>>,
    inner: Rc<RefCell<EnvInner>>,
}

impl Env {
    pub fn define(&mut self, name: impl AsRef<str>, val: impl Into<Literal>) {
        let val = val.into();
        self.inner
            .borrow_mut()
            .vars
            .insert(name.as_ref().to_string(), val);
    }

    pub fn get(&self, token: &Token) -> Result<Literal, EnvError> {
        self.inner.borrow().get(token).or_else(|err| {
            if let EnvError::NotFound { .. } = err {
                self.parent.as_ref().ok_or(err)?.borrow().get(token)
            } else {
                Err(err)
            }
        })
    }
}

#[derive(Default)]
pub struct EnvInner {
    vars: HashMap<String, Literal>,
}

impl EnvInner {
    fn get(&self, token: &Token) -> Result<Literal, EnvError> {
        let key = &token.lexeme;
        self.vars
            .get(key.as_ref())
            .cloned()
            .ok_or_else(|| EnvError::NotFound {
                token: token.clone(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let mut env = Env::default();
        env.define("foo", "bar");
    }
}
