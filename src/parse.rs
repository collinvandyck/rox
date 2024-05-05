use crate::prelude::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("parsing failed:\n{}", errs.iter().map(|l| l.to_string()).join("\n"))]
    Failed { errs: Vec<LineError> },
}

#[derive(thiserror::Error, Debug)]
pub enum LineError {
    #[error("line {line}: expected {expected} but was instead {actual}")]
    Expected {
        expected: TokenType,
        actual: TokenType,
        line: usize,
    },
    #[error("line {line}: expected expression")]
    ExpectedExpr { line: usize },
}

impl Parser {
    pub fn new(tokens: impl IntoIterator<Item = Token>) -> Self {
        let tokens = tokens.into_iter().collect();
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let mut errs = vec![];
        loop {
            if self.at_end() {
                break;
            }
            match self.expr() {
                Ok(expr) if errs.is_empty() => {
                    return Ok(expr);
                }
                Ok(_expr) => {} // ignore
                Err(err) => {
                    errs.push(err);
                    self.synchronize();
                }
            }
        }
        Err(ParseError::Failed { errs })
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.at_end() {
            if self.previous().typ == TokenType::Semicolon {
                return;
            }
            if matches!(
                self.peek().typ,
                TokenType::Class
                    | TokenType::For
                    | TokenType::Fun
                    | TokenType::If
                    | TokenType::Print
                    | TokenType::Return
                    | TokenType::Var
                    | TokenType::While
            ) {
                return;
            }
            self.advance();
        }
    }

    fn stmt(&mut self) -> Result<Stmt, LineError> {
        if self.match_any([TokenType::Print]) {
            return self.print_stmt();
        }
        self.expr_stmt()
    }

    fn print_stmt(&mut self) -> Result<Stmt, LineError> {
        todo!()
    }

    fn expr_stmt(&mut self) -> Result<Stmt, LineError> {
        todo!()
    }

    // expression -> equality ;
    fn expr(&mut self) -> Result<Expr, LineError> {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.comparison()?;
        while self.match_any([TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expr::binary(expr, op, right);
        }
        Ok(expr)
    }

    // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.term()?;
        while self.match_any([
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::binary(expr, op, right);
        }
        Ok(expr)
    }

    // term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.factor()?;
        while self.match_any([TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::binary(expr, op, right);
        }
        Ok(expr)
    }

    // fractor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.unary()?;
        while self.match_any([TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::binary(expr, op, right);
        }
        Ok(expr)
    }

    // unary → ( "!" | "-" ) unary
    //         | primary ;
    fn unary(&mut self) -> Result<Expr, LineError> {
        if self.match_any([TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::unary(op, right));
        }
        self.primary()
    }

    // primary → NUMBER | STRING | "true" | "false" | "nil"
    //           | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, LineError> {
        if self.match_any([TokenType::False]) {
            return Ok(Expr::literal(Literal::Bool(false)));
        }
        if self.match_any([TokenType::True]) {
            return Ok(Expr::literal(Literal::Bool(true)));
        }
        if self.match_any([TokenType::Nil]) {
            return Ok(Expr::literal(Literal::Nil));
        }
        if self.match_any([TokenType::Number, TokenType::String]) {
            let prev = self.previous();
            return Ok(Expr::literal(prev.literal.unwrap()));
        }
        if self.match_any([TokenType::LeftParen]) {
            let expr = self.expr()?;
            self.consume(TokenType::RightParen)?;
            return Ok(Expr::group(expr));
        }
        Err(LineError::ExpectedExpr {
            line: self.peek().line,
        })
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

    fn consume(&mut self, typ: TokenType) -> Result<(), LineError> {
        if self.match_any([typ]) {
            return Ok(());
        }
        Err(LineError::Expected {
            expected: typ,
            actual: self.peek().typ,
            line: self.peek().line,
        })
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
