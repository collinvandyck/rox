use crate::prelude::*;
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

#[derive(thiserror::Error, Debug, strum_macros::EnumIs)]
pub enum EnvError {
    #[error("undefined variable in assign '{}'", name)]
    UndefinedAssign { name: String },

    #[error("undefined variable '{}'", token.lexeme)]
    NotFound { token: Token },
}

#[derive(Clone, Default)]
pub struct Env {
    inner: Rc<RefCell<EnvInner>>,
}

impl Env {
    pub fn assign(&self, name: impl AsRef<str>, val: impl Into<Literal>) -> Result<(), EnvError> {
        self.inner.borrow_mut().assign(name, val)
    }
    pub fn define(&self, name: impl AsRef<str>, val: impl Into<Literal>) {
        let val = val.into();
        self.inner
            .borrow_mut()
            .vars
            .insert(name.as_ref().to_string(), val);
    }

    pub fn get(&self, token: &Token) -> Result<Literal, EnvError> {
        self.inner.as_ref().borrow().get(token)
    }

    pub fn child(&self) -> Self {
        let inner = EnvInner {
            parent: Some(Rc::new(RefCell::new(self.clone()))),
            ..Default::default()
        };
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }
}

#[derive(Default)]
pub struct EnvInner {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Literal>,
}

impl EnvInner {
    fn assign(&mut self, name: impl AsRef<str>, val: impl Into<Literal>) -> Result<(), EnvError> {
        if let Some(v) = self.vars.get_mut(name.as_ref()) {
            *v = val.into();
            Ok(())
        } else {
            Err(EnvError::UndefinedAssign {
                name: name.as_ref().into(),
            })
        }
    }
    fn get(&self, token: &Token) -> Result<Literal, EnvError> {
        let f: Option<&Literal> = self.vars.get(token.lexeme.as_ref());
        if let Some(f) = f {
            return Ok(f.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.as_ref().borrow().get(token);
        }
        Err(EnvError::NotFound {
            token: token.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let env = Env::default();
        env.define("foo", "bar");
        env.define("fzz", "bzz");
        assert_eq!(env.get(&id("foo")).unwrap(), "bar".into());
        assert_eq!(env.get(&id("fzz")).unwrap(), "bzz".into());
        assert!(env.get(&id("fuz")).unwrap_err().is_not_found());
        assert!(env
            .assign("foo-dne", "baz")
            .unwrap_err()
            .is_undefined_assign());

        let child = env.child();
        assert_eq!(child.get(&id("foo")).unwrap(), "bar".into());
        assert!(child.get(&id("fuz")).unwrap_err().is_not_found());
        child.define("foo", "baz");
        assert_eq!(child.get(&id("foo")).unwrap(), "baz".into());
        assert_eq!(child.get(&id("fzz")).unwrap(), "bzz".into());

        assert_eq!(env.get(&id("foo")).unwrap(), "bar".into());
        assert!(env.get(&id("fuz")).unwrap_err().is_not_found());
        assert_eq!(env.get(&id("fzz")).unwrap(), "bzz".into());

        env.assign("fzz", "dzz").unwrap();
        assert_eq!(env.get(&id("fzz")).unwrap(), "dzz".into());
    }

    fn id(name: &str) -> Token {
        Token {
            typ: TokenType::Identifier,
            lexeme: name.into(),
            literal: Some(name.into()),
            line: 1,
        }
    }
}
