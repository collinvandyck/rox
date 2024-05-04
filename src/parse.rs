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

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_any([TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::binary(expr, op, right);
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        todo!()
    }

    fn current(&self) -> Token {
        todo!()
    }

    fn previous(&self) -> Token {
        todo!()
    }

    fn match_any(&mut self, typs: impl IntoIterator<Item = TokenType>) -> bool {
        todo!()
    }
}
