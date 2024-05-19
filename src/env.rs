use crate::prelude::*;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::VecDeque,
};

#[derive(thiserror::Error, Debug, strum_macros::EnumIs)]
pub enum EnvError {
    #[error("undefined variable in assign '{}'", name)]
    UndefinedAssign { name: String },

    #[error("undefined variable '{}'", token.lexeme)]
    NotFound { token: Token },

    #[error("a binding '{}' already exists in this scope", name)]
    AlreadyDefined { name: String },

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

type Result<T> = std::result::Result<T, EnvError>;

#[derive(Clone, Debug, Default)]
pub struct Env {
    // the offset into the inner record collection that instructs get/assign operations how far
    // into the vec of records that may be considered. because Env can be cloned, the cursor value
    // that will also be copied is how we snapshot a particular scoped set of env values.
    cursor: usize,
    inner: Rc<RefCell<Inner>>,
}

impl Env {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cursor(&self, cursor: usize) -> Self {
        let mut clone = self.clone();
        clone.cursor = cursor;
        clone
    }

    pub fn push(&mut self) {
        let env = self.child();
        *self = env;
    }

    pub fn child(&self) -> Self {
        let parent = self.clone();
        let inner = Inner::new(Some(parent));
        Env {
            inner: Rc::new(RefCell::new(inner)),
            cursor: 0,
        }
    }

    pub fn pop(&mut self) -> Result<()> {
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

    pub fn define(&mut self, name: impl AsRef<str>, val: impl Into<Value>) -> Result<()> {
        self.inner
            .as_ref()
            .borrow_mut()
            .define(name.as_ref(), val)?;
        self.cursor += 1;
        Ok(())
    }

    pub fn assign(&self, name: impl AsRef<str>, val: impl Into<Value>) -> Result<()> {
        self.inner
            .as_ref()
            .borrow_mut()
            .assign(name, val, self.cursor)
    }

    pub fn get(&self, token: &Token) -> Result<Value> {
        self.inner.as_ref().borrow().get(token, self.cursor)
    }
}

#[derive(Clone, Debug, Default)]
struct Inner {
    parent: Option<Env>,
    records: Vec<Record>,
}

#[derive(Clone, Debug)]
struct Record {
    name: String,
    val: Value,
}

impl Inner {
    fn new(parent: Option<Env>) -> Self {
        Self {
            parent,
            records: vec![],
        }
    }
    fn define(&mut self, name: impl AsRef<str>, val: impl Into<Value>) -> Result<()> {
        if self.parent.is_some() {
            if self.records.iter().any(|r| &r.name == name.as_ref()) {
                return Err(EnvError::AlreadyDefined {
                    name: name.as_ref().to_string(),
                });
            }
        }
        let name = name.as_ref().to_string();
        let val = val.into();
        let record = Record { name, val };
        self.records.push(record);
        Ok(())
    }

    fn cursor(&self, cursor: usize) -> usize {
        if self.parent.is_some() {
            cursor
        } else {
            self.records.len()
        }
    }

    fn assign(
        &mut self,
        name: impl AsRef<str>,
        val: impl Into<Value>,
        cursor: usize,
    ) -> Result<()> {
        let cursor = self.cursor(cursor);
        if let Some(found) = self
            .records
            .iter_mut()
            .take(cursor)
            .rev()
            .find(|r| &r.name == name.as_ref())
        {
            let name = name.as_ref().to_string();
            let val = val.into();
            let record = Record { name, val };
            *found = record;
            return Ok(());
        }
        if let Some(parent) = &self.parent {
            return parent.assign(name, val);
        }
        Err(EnvError::undefined_assign(name))
    }

    fn get(&self, token: &Token, cursor: usize) -> Result<Value> {
        let cursor = self.cursor(cursor);
        if let Some(found) = self
            .records
            .iter()
            .take(cursor)
            .rev()
            .find(|r| &r.name == token.lexeme.as_ref())
        {
            return Ok(found.val.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.get(token);
        }
        Err(EnvError::not_found(token))
    }
}

#[cfg(test)]
mod tests {
    use super::Env;
    use super::*;

    #[test]
    fn test_env() {
        let mut env = Env::default();
        env.define("foo", "bar").unwrap();
        env.define("fzz", "bzz").unwrap();
        assert_eq!(env.get(&id("foo")).unwrap(), "bar".into());
        assert_eq!(env.get(&id("fzz")).unwrap(), "bzz".into());
        assert!(env.get(&id("fuz")).unwrap_err().is_not_found());
        assert!(env
            .assign("foo-dne", "baz")
            .unwrap_err()
            .is_undefined_assign());

        env.push();
        assert_eq!(env.get(&id("foo")).unwrap(), "bar".into());
        assert!(env.get(&id("fuz")).unwrap_err().is_not_found());
        env.define("foo", "baz").unwrap();
        assert_eq!(env.get(&id("foo")).unwrap(), "baz".into());
        assert_eq!(env.get(&id("fzz")).unwrap(), "bzz".into());

        env.pop();
        assert_eq!(env.get(&id("foo")).unwrap(), "bar".into());
        assert!(env.get(&id("fuz")).unwrap_err().is_not_found());
        assert_eq!(env.get(&id("fzz")).unwrap(), "bzz".into());

        env.assign("fzz", "dzz").unwrap();
        assert_eq!(env.get(&id("fzz")).unwrap(), "dzz".into());
    }

    #[test]
    fn test_nested_assign() {
        let mut env = Env::default();
        env.define("foo", "bar").unwrap();
        env.assign("foo", "bar2").unwrap();

        env.push();
        env.define("foo", "bar").unwrap();
        assert_eq!(env.get(&id("foo")).unwrap(), "bar".into());
        env.pop();

        env.push();
        assert_eq!(env.get(&id("foo")).unwrap(), "bar2".into());
        env.assign("foo", "bar3");
        assert_eq!(env.get(&id("foo")).unwrap(), "bar3".into());
        env.pop();

        assert_eq!(env.get(&id("foo")).unwrap(), "bar3".into());
    }

    #[test]
    fn test_push_pop() {
        let mut env = Env::default();
        env.define("foo", "bar").unwrap();
        env.assign("foo", "bar2").unwrap();

        env.push();
        env.define("foo", "baz").unwrap();
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
