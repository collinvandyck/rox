use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

type Result<T> = std::result::Result<T, Error>;

pub struct Scanner {
    source: Vec<char>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
        }
    }

    pub fn scan_tokens() -> Result<Vec<()>> {
        Ok(vec![])
    }
}
