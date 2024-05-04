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

    // expression -> equality ;
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

    // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_any([
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::binary(expr, op, right);
        }
        expr
    }

    // term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_any([TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::binary(expr, op, right);
        }
        expr
    }

    // fractor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_any([TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::binary(expr, op, right);
        }
        expr
    }

    // unary → ( "!" | "-" ) unary
    //         | primary ;
    fn unary(&mut self) -> Expr {
        if self.match_any([TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            return Expr::unary(op, right);
        }
        self.primary()
    }

    // primary → NUMBER | STRING | "true" | "false" | "nil"
    //           | "(" expression ")" ;
    fn primary(&mut self) -> Expr {
        todo!()
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

    fn check(&self, typ: TokenType) -> bool {
        self.tokens.get(self.current).map(|t| t.typ) == Some(typ)
    }

    fn previous(&self) -> Token {
        self.token_at(self.current - 1)
    }

    fn peek(&self) -> Token {
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
        self.tokens.get(self.current).map(|t| t.typ) == Some(TokenType::Eof)
    }
}
