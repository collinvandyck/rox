use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Function(Function),
    Class(Class),
    Instance(Instance),
    Nil,
    Undefined,
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ValueError {
    #[error("value was not a number")]
    NotANumber,
}

impl Value {
    pub fn as_callable(&self) -> Option<&dyn Callable> {
        match self {
            Value::Function(f) => Some(f),
            Value::Class(c) => Some(c),
            _ => None,
        }
    }

    pub fn to_lox(&self) -> String {
        match self {
            Self::Number(v) => v.to_string(),
            Self::String(s) => s.clone(),
            Self::Bool(b) => b.to_string(),
            Self::Nil => "nil".to_string(),
            Self::Undefined => "undefined".to_string(),
            Self::Function(f) => f.to_string(),
            Self::Class(c) => c.to_string(),
            Self::Instance(i) => i.to_string(),
        }
    }
    pub fn truthy(&self) -> bool {
        match self {
            Self::Class(_)
            | Self::Instance(_)
            | Self::Number(_)
            | Self::String(_)
            | Self::Function(_) => true,
            Self::Bool(b) => *b,
            Self::Nil | Self::Undefined => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, r#""{s}""#),
            Self::Bool(v) => write!(f, "{v}"),
            Self::Nil => write!(f, "nil"),
            Self::Undefined => write!(f, "undefined"),
            Self::Function(func) => func.fmt(f),
            Self::Class(c) => c.fmt(f),
            Self::Instance(i) => i.fmt(f),
        }
    }
}
