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
    pub fn assign(&self, name: impl AsRef<str>, val: impl Into<Literal>) -> Result<(), EnvError> {
        let mut inner = self.inner.borrow_mut();
        let lit = inner
            .vars
            .get_mut(name.as_ref())
            .ok_or_else(|| EnvError::UndefinedAssign {
                name: name.as_ref().to_string(),
            })?;
        *lit = val.into();
        Ok(())
    }
    pub fn define(&self, name: impl AsRef<str>, val: impl Into<Literal>) {
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

    pub fn child(&self) -> Self {
        Self {
            parent: Some(self.inner.clone()),
            ..Default::default()
        }
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
