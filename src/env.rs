use crate::prelude::*;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

#[derive(thiserror::Error, Debug, strum_macros::EnumIs, derive_more::From)]
pub enum EnvError {
    #[error("undefined variable in assign '{}'", name)]
    UndefinedAssign { name: String },

    #[error("undefined variable '{}'", token.lexeme)]
    NotFound { token: Token },

    #[error("no parent env to restore to")]
    NoParentEnv,
}

impl EnvError {
    fn undefined_assign(name: impl AsRef<str>) -> Self {
        Self::UndefinedAssign {
            name: name.as_ref().to_string(),
        }
    }
    fn not_found(token: &Token) -> Self {
        Self::NotFound {
            token: token.clone(),
        }
    }
}

#[derive(Clone, Default)]
pub struct Env {
    inner: Rc<RefCell<EnvInner>>,
}

impl std::fmt::Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.as_ref().borrow().fmt(f)
    }
}

impl Env {
    pub fn assign(&self, name: impl AsRef<str>, val: impl Into<Value>) -> Result<(), EnvError> {
        self.inner.as_ref().borrow_mut().assign(name, val)
    }
    pub fn define(&self, name: impl AsRef<str>, val: impl Into<Value>) {
        self.inner.as_ref().borrow_mut().define(name, val);
    }
    pub fn get(&self, token: &Token) -> Result<Value, EnvError> {
        self.inner.as_ref().borrow().get(token)
    }
    pub fn push(&mut self) {
        let child = self.child();
        *self = child;
    }
    pub fn pop(&mut self) -> Result<(), EnvError> {
        let env = self
            .inner
            .as_ref()
            .borrow_mut()
            .parent
            .take()
            .ok_or(EnvError::NoParentEnv)?;
        *self = env;
        Ok(())
    }
    pub fn child(&self) -> Self {
        let inner = EnvInner {
            parent: Some(self.clone()),
            ..Default::default()
        };
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }
}

#[derive(Default, Debug)]
pub struct EnvInner {
    parent: Option<Env>,
    vars: HashMap<String, Value>,
}

impl EnvInner {
    fn define(&mut self, name: impl AsRef<str>, val: impl Into<Value>) {
        self.vars.insert(name.as_ref().to_string(), val.into());
    }
    fn assign(&mut self, name: impl AsRef<str>, val: impl Into<Value>) -> Result<(), EnvError> {
        if let Some(v) = self.vars.get_mut(name.as_ref()) {
            *v = val.into();
            return Ok(());
        }
        if let Some(parent) = &self.parent {
            return parent.assign(name, val);
        }
        Err(EnvError::undefined_assign(name))
    }
    fn get(&self, token: &Token) -> Result<Value, EnvError> {
        let f: Option<&Value> = self.vars.get(token.lexeme.as_ref());
        if let Some(f) = f {
            return Ok(f.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.get(token);
        }
        Err(EnvError::not_found(token))
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

    #[test]
    fn test_nested_assign() {
        let env = Env::default();
        env.define("foo", "bar");
        env.assign("foo", "bar2");

        let child = env.child();
        child.define("foo", "bar");
        assert_eq!(child.get(&id("foo")).unwrap(), "bar".into());

        let child = env.child();
        assert_eq!(child.get(&id("foo")).unwrap(), "bar2".into());
        child.assign("foo", "bar3");
        assert_eq!(child.get(&id("foo")).unwrap(), "bar3".into());

        assert_eq!(env.get(&id("foo")).unwrap(), "bar3".into());
    }

    #[test]
    fn test_push_pop() {
        let mut env = Env::default();
        env.define("foo", "bar");
        env.assign("foo", "bar2");

        env.push();
        env.define("foo", "baz");
        assert_eq!(env.get(&id("foo")).unwrap(), "baz".into());

        env.pop();
        assert_eq!(env.get(&id("foo")).unwrap(), "bar2".into());
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
