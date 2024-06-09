use crate::prelude::*;
use std::{collections::HashMap, fmt::Display};

#[derive(thiserror::Error, Debug)]
pub enum InstanceError {
    #[error("undefined property '{name}'")]
    UndefinedProperty { name: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    inner: Rc<RefCell<Inner>>,
}

#[derive(Clone, Debug, PartialEq)]
struct Inner {
    class: Class,
    fields: HashMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Fields {
    vals: Rc<RefCell<HashMap<String, Value>>>,
}

impl Instance {
    pub fn new(class: Class, fields: HashMap<String, Value>) -> Self {
        let inner = Inner { class, fields };
        let inner = Rc::new(RefCell::new(inner));
        Self { inner }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Result<Value, InstanceError> {
        let name = name.as_ref();
        self.inner
            .as_ref()
            .borrow()
            .fields
            .get(name)
            .ok_or_else(|| InstanceError::UndefinedProperty {
                name: name.to_string(),
            })
            .cloned()
    }

    pub fn set(&self, name: impl AsRef<str>, value: Value) -> Result<Value, InstanceError> {
        let name = name.as_ref();
        self.inner
            .as_ref()
            .borrow_mut()
            .fields
            .insert(name.to_string(), value);
        Ok(Value::Nil)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.inner.as_ref().borrow().class)
    }
}
