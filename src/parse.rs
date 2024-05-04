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

    fn check(&self, typ: TokenType) -> bool {
        self.tokens.get(self.current).map(|t| t.typ) == Some(typ)
    }

    fn match_any(&mut self, types: impl IntoIterator<Item = TokenType>) -> bool {
        for typ in types {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn previous(&self) -> Token {
        self.token_at(self.current - 1)
    }

    fn current(&self) -> Token {
        self.token_at(self.current)
    }

    fn token_at(&self, pos: usize) -> Token {
        self.tokens[pos].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
