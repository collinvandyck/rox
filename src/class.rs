use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    name: String,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {}", self.name)
    }
}
