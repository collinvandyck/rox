use crate::prelude::*;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    fmt::Display,
};

/// The *runtime* representation of a lox class
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    name: String,
    methods: Methods,
}

type Methods = HashMap<String, LoxFunction>;

#[derive(thiserror::Error, Debug)]
pub enum InstanceError {
    #[error("undefined property '{name}'")]
    UndefinedProperty { name: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    class: Class,
    fields: Fields,
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Fields {
    vals: Rc<RefCell<HashMap<String, Value>>>,
}

impl Class {
    pub fn new(name: impl AsRef<str>, methods: Methods) -> Self {
        Self {
            name: name.as_ref().to_string(),
            methods,
        }
    }
}

impl Instance {
    pub fn get(&self, name: impl AsRef<str>) -> Result<Value, InstanceError> {
        let name = name.as_ref();
        self.fields
            .vals
            .as_ref()
            .borrow()
            .get(name)
            .ok_or_else(|| InstanceError::UndefinedProperty {
                name: name.to_string(),
            })
            .cloned()
    }

    pub fn set(&self, name: impl AsRef<str>, value: Value) -> Result<Value, InstanceError> {
        let name = name.as_ref();
        self.fields
            .vals
            .as_ref()
            .borrow_mut()
            .insert(name.to_string(), value);
        Ok(Value::Nil)
    }
}

/// A Class is callable in the sense that the class itself is also a constructor
impl Callable for Class {
    fn call(&self, int: &mut Interpreter, args: Vec<Value>) -> Result<Value, CallableError> {
        Ok(Value::from(Instance {
            class: self.clone(),
            fields: Fields::default(),
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
