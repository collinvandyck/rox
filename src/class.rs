use crate::prelude::*;
use std::{collections::HashMap, fmt::Display};

/// The *runtime* representation of a lox class
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum InstanceError {
    #[error("undefined property '{name}'")]
    UndefinedProperty { name: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    class: Class,
    fields: HashMap<String, Value>,
}

impl Class {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }
}

impl Instance {
    pub fn get(&self, name: impl AsRef<str>) -> Result<Value, InstanceError> {
        let name = name.as_ref();
        self.fields
            .get(name)
            .ok_or_else(|| InstanceError::UndefinedProperty {
                name: name.to_string(),
            })
            .cloned()
    }
}

/// A Class is callable in the sense that the class itself is also a constructor
impl Callable for Class {
    fn call(&self, int: &mut Interpreter, args: Vec<Value>) -> Result<Value, CallableError> {
        Ok(Value::from(Instance {
            class: self.clone(),
            fields: HashMap::default(),
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
