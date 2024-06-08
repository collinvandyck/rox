use std::fmt::Display;

/// The *runtime* representation of a lox class
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
