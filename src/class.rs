use crate::prelude::*;
use std::fmt::Display;

/// The *runtime* representation of a lox class
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    class: Class,
}

impl Class {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }
}

impl Instance {
    pub fn get(&self, name: &Token) -> Value {
        todo!()
    }
}

/// A Class is callable in the sense that the class itself is also a constructor
impl Callable for Class {
    fn call(&self, int: &mut Interpreter, args: Vec<Value>) -> Result<Value, CallableError> {
        Ok(Value::from(Instance {
            class: self.clone(),
        }))
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
