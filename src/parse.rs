use crate::prelude::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: impl IntoIterator<Item = Token>) -> Self {
        let tokens = tokens.into_iter().collect();
        Self { tokens, current: 0 }
    }

    fn equality(&mut self) -> Expr {
        todo!()
    }
}
