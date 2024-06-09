use crate::prelude::*;

use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    fmt::Display,
};

/// The *runtime* representation of a lox class
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    inner: Rc<RefCell<ClassInner>>,
}

#[derive(Clone, Debug, PartialEq)]
struct ClassInner {
    name: String,
    methods: Methods,
}

type Methods = HashMap<String, LoxFunction>;

impl Class {
    pub fn new(name: impl AsRef<str>, methods: Methods) -> Self {
        let inner = ClassInner {
            name: name.as_ref().to_string(),
            methods,
        };
        let inner = Rc::new(RefCell::new(inner));
        Self { inner }
    }
}

/// A Class is callable in the sense that the class itself is also a constructor
impl Callable for Class {
    fn call(&self, int: &mut Interpreter, args: Vec<Value>) -> Result<Value, CallableError> {
        Ok(Value::from(Instance::new(self.clone(), HashMap::default())))
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.inner.as_ref().borrow().name;
        write!(f, "{name}")
    }
}
