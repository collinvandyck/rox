use std::io::{self, stderr};

use crate::prelude::*;
type TT = TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errs: Vec<LineError>,
    stderr: Box<dyn io::Write>,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("parsing failed")]
    Failed,

    #[error("single expr required")]
    SingleEpxr,

    #[error(transparent)]
    LineError(#[from] LineError),
}

#[derive(thiserror::Error, Debug)]
pub enum LineError {
    #[error("line {line}: expected {expected} but was instead {actual}")]
    Expected {
        expected: TT,
        actual: TT,
        line: usize,
    },
    #[error("line {line}: expected expression")]
    ExpectedExpr { line: usize },

    #[error("line {}: too many args (max: 255)", token.line)]
    TooManyArgs { token: Token },

    #[error("line {}: too many params (max: 255)", token.line)]
    TooManyParams { token: Token },

    #[error("({kind}): {err}")]
    FunctionKind {
        kind: FunctionKind,
        #[source]
        err: Box<LineError>,
    },

    #[error("{context}: {err}")]
    WithContext {
        context: String,
        #[source]
        err: Box<LineError>,
    },
}

trait LineResultExt<T> {
    fn context(self, ctx: impl AsRef<str>) -> Result<T, LineError>;
    fn for_fn_kind(self, fk: FunctionKind) -> Result<T, LineError>;
}

impl<T> LineResultExt<T> for Result<T, LineError> {
    /// extends a LineError result with additional context info
    fn context(self, ctx: impl AsRef<str>) -> Result<T, LineError> {
        self.map_err(|err| err.context(ctx.as_ref().to_string()))
    }
    /// adds additional context about the kind of fn
    fn for_fn_kind(self, kind: FunctionKind) -> Result<T, LineError> {
        self.map_err(|err| LineError::FunctionKind {
            kind,
            err: err.into(),
        })
    }
}

impl LineError {
    fn fn_kind(kind: FunctionKind, err: Self) -> Self {
        Self::FunctionKind {
            kind,
            err: err.into(),
        }
    }
    fn context(self, s: impl AsRef<str>) -> Self {
        Self::WithContext {
            context: s.as_ref().to_string(),
            err: Box::new(self),
        }
    }
}

#[derive(Debug, Clone, Copy, strum_macros::Display)]
pub enum FunctionKind {
    Function,
    Method,
}

impl Parser {
    pub fn new(tokens: impl IntoIterator<Item = Token>) -> Self {
        let tokens = tokens.into_iter().collect();
        Self {
            tokens,
            current: 0,
            errs: vec![],
            stderr: Box::new(stderr()),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = vec![];
        let mut failed = false;
        loop {
            if self.at_end() {
                break;
            }
            match self.decl() {
                Ok(stmt) => {
                    if !failed {
                        stmts.push(stmt);
                    }
                }

                Err(err) => {
                    tracing::error!("parse: {err}");
                    failed = true;
                    self.synchronize();
                }
            }
        }
        if failed || !self.errs.is_empty() {
            Err(ParseError::Failed)
        } else {
            Ok(stmts)
        }
    }

    // returns an error if the tokens have more than a single expr
    pub fn single_expr(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expr()?;
        if self.at_end() {
            Ok(expr)
        } else {
            Err(ParseError::SingleEpxr)
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.at_end() {
            if self.previous().typ == TT::Semicolon {
                return;
            }
            if matches!(
                self.peek().typ,
                TT::Class
                    | TT::For
                    | TT::Fun
                    | TT::If
                    | TT::Print
                    | TT::Return
                    | TT::Var
                    | TT::While
            ) {
                return;
            }
            self.advance();
        }
    }

    fn decl(&mut self) -> Result<Stmt, LineError> {
        if self.match_any(TT::Fun) {
            return self.function(FunctionKind::Function);
        }
        if self.match_any(TT::Var) {
            return self.var_decl();
        }
        if self.match_any(TT::Class) {
            return self.class_decl();
        }
        self.stmt()
    }

    fn class_decl(&mut self) -> Result<Stmt, LineError> {
        let ident = self.consume(TT::Identifier)?;
        self.consume(TT::LeftBrace)
            .context("expect '{' before class body")?;
        let mut methods = vec![];
        while !self.check(TT::RightBrace) && !self.at_end() {
            methods.push(self.function(FunctionKind::Method)?);
        }
        self.consume(TokenType::RightBrace)
            .context("expect '}' after class body")?;
        Ok(Stmt::Class(ClassStmt {
            name: ident,
            methods,
        }))
    }

    fn function(&mut self, kind: FunctionKind) -> Result<Stmt, LineError> {
        let name = self.consume(TT::Identifier).for_fn_kind(kind)?;
        self.consume(TT::LeftParen).for_fn_kind(kind)?;
        let mut params = vec![];
        if !self.check(TT::RightParen) {
            loop {
                params.push(self.consume(TT::Identifier)?);
                if !self.match_any(TT::Comma) {
                    break;
                }
            }
        }
        if params.len() >= 255 {
            self.smol_error(LineError::TooManyParams { token: self.peek() });
        }
        self.consume(TT::RightParen).for_fn_kind(kind)?;
        self.consume(TT::LeftBrace)?;
        let body = self.block()?;
        Ok(Stmt::Function(FunctionStmt { name, params, body }))
    }

    fn var_decl(&mut self) -> Result<Stmt, LineError> {
        let name = self.consume(TT::Identifier)?;
        let initializer = self.match_any(TT::Equal).then(|| self.expr()).transpose()?;
        self.consume(TT::Semicolon)?;
        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    fn stmt(&mut self) -> Result<Stmt, LineError> {
        if self.match_any(TT::If) {
            return self.if_stmt();
        }
        if self.match_any(TT::Print) {
            return self.print_stmt();
        }
        if self.match_any(TT::Return) {
            return self.return_stmt();
        }
        if self.match_any(TT::While) {
            return self.while_stmt();
        }
        if self.match_any(TT::For) {
            return self.for_stmt();
        }
        if self.match_any(TT::LeftBrace) {
            return self.block_stmt();
        }
        self.expr_stmt()
    }

    fn return_stmt(&mut self) -> Result<Stmt, LineError> {
        let keyword = self.previous();
        let value = if self.check(TT::Semicolon) {
            Expr::Literal(LiteralExpr { value: Value::Nil })
        } else {
            self.expr()?
        };
        self.consume(TT::Semicolon)?;
        Ok(Stmt::Return(ReturnStmt { keyword, value }))
    }

    fn for_stmt(&mut self) -> Result<Stmt, LineError> {
        self.consume(TT::LeftParen)?;
        let init: Option<Stmt> = if self.match_any(TT::Semicolon) {
            None
        } else if self.match_any(TT::Var) {
            Some(self.var_decl()?)
        } else {
            Some(self.expr_stmt()?)
        };
        let condition: Expr = if !self.check(TT::Semicolon) {
            self.expr()?
        } else {
            Expr::literal(true)
        };
        self.consume(TT::Semicolon)?;
        let incr = if !self.check(TT::RightParen) {
            Some(self.expr()?)
        } else {
            None
        };
        self.consume(TT::RightParen)?;
        let mut body = self.stmt()?;
        if let Some(incr) = incr {
            body = Stmt::Block(BlockStmt {
                statements: vec![body, Stmt::Expr(ExprStmt { expr: incr })],
            });
        }
        body = Stmt::While(WhileStmt {
            condition,
            body: Box::new(body),
        });
        if let Some(init) = init {
            body = Stmt::Block(BlockStmt {
                statements: vec![init, body],
            });
        }
        Ok(body)
    }

    fn while_stmt(&mut self) -> Result<Stmt, LineError> {
        self.consume(TT::LeftParen)?;
        let condition = self.expr()?;
        self.consume(TT::RightParen)?;
        let body = self.stmt()?;
        Ok(Stmt::While(WhileStmt {
            condition,
            body: Box::new(body),
        }))
    }

    fn if_stmt(&mut self) -> Result<Stmt, LineError> {
        self.consume(TT::LeftParen)?;
        let condition = self.expr()?;
        self.consume(TT::RightParen)?;
        let then_stmt = Box::new(self.stmt()?);
        let mut else_stmt = None;
        if self.match_any(TT::Else) {
            else_stmt.replace(Box::new(self.stmt()?));
        }
        Ok(Stmt::If(IfStmt {
            condition,
            then_stmt,
            else_stmt,
        }))
    }

    fn print_stmt(&mut self) -> Result<Stmt, LineError> {
        tracing::debug!("print stmt");
        let expr = self.expr()?;
        self.consume(TT::Semicolon)?;
        tracing::debug!("print stmt ok");
        Ok(Stmt::Print(PrintStmt { expr }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LineError> {
        let mut statements = vec![];
        while !self.check(TT::RightBrace) && !self.at_end() {
            let stmt = self.decl()?;
            statements.push(stmt);
        }
        self.consume(TT::RightBrace)?;
        Ok(statements)
    }

    fn block_stmt(&mut self) -> Result<Stmt, LineError> {
        debug!("block_stmt");
        let statements = self.block()?;
        Ok(Stmt::Block(BlockStmt { statements }))
    }

    fn expr_stmt(&mut self) -> Result<Stmt, LineError> {
        debug!("expr_stmt");
        let expr = self.expr()?;
        self.consume(TT::Semicolon)?;
        debug!("expr_stmt ok");
        Ok(Stmt::Expr(ExprStmt { expr }))
    }

    // expression -> equality ;
    fn expr(&mut self) -> Result<Expr, LineError> {
        self.assignment()
    }

    // assignment → ( call "." )? IDENTIFIER "=" assignment
    //              | logic_or ;
    //
    // assignment is right-associative so we recurse to build the RHS
    fn assignment(&mut self) -> Result<Expr, LineError> {
        let expr = self.or()?;
        if self.match_any(TT::Equal) {
            // TODO: we match on a ref so that we don't move the expr into the match, but it makes
            // us need to clone the fields for the other assign/set exprs. is there a way to do
            // this better?
            match &expr {
                Expr::Var(VarExpr { name }) => {
                    let equal = self.previous();
                    let value = self.assignment()?;
                    return Ok(Expr::from(AssignExpr {
                        name: name.clone(),
                        value: value.into(),
                    }));
                }
                // here we convert a GetExpr into a SetExpr since an '=' follows it.
                Expr::Get(GetExpr { object, name }) => {
                    return Ok(Expr::from(SetExpr {
                        object: object.clone(),
                        name: name.clone(),
                        value: expr.into(),
                    }));
                }
                _ => {
                    return dbg!(Err(self.expected_typ_error(TT::Equal)));
                }
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.and()?;
        while self.match_any(TT::Or) {
            let op = self.previous();
            let right = self.and()?;
            expr = Expr::logical(expr, op, right);
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.equality()?;
        while self.match_any(TT::And) {
            let op = self.previous();
            let right = self.equality()?;
            expr = Expr::logical(expr, op, right);
        }
        Ok(expr)
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.comparison()?;
        while self.match_any([TT::BangEqual, TT::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expr::binary(expr, op, right);
        }
        Ok(expr)
    }

    // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.term()?;
        while self.match_any([TT::Less, TT::LessEqual, TT::Greater, TT::GreaterEqual]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::binary(expr, op, right);
        }
        Ok(expr)
    }

    // term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.factor()?;
        while self.match_any([TT::Minus, TT::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::binary(expr, op, right);
        }
        Ok(expr)
    }

    // fractor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.unary()?;
        while self.match_any([TT::Slash, TT::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::binary(expr, op, right);
        }
        Ok(expr)
    }

    // unary → ( "!" | "-" ) unary
    //         | call;
    fn unary(&mut self) -> Result<Expr, LineError> {
        if self.match_any([TT::Bang, TT::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::unary(op, right));
        }
        self.call()
    }

    // call      -> primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
    // arguments -> expression ( "," expression )* ;
    fn call(&mut self) -> Result<Expr, LineError> {
        let mut expr = self.primary()?;
        loop {
            if self.match_any(TT::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_any(TT::Dot) {
                let name = self
                    .consume(TT::Identifier)
                    .context("expect property name after '.'")?;
                expr = Expr::from(GetExpr {
                    name,
                    object: expr.into(),
                })
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, expr: Expr) -> Result<Expr, LineError> {
        let mut args = vec![];
        let mut args_err = None;
        if !self.check(TT::RightParen) {
            loop {
                args.push(self.expr()?);
                if args.len() >= 255 {
                    // we report the error but keep going
                    args_err.replace(LineError::TooManyArgs { token: self.peek() });
                }
                if !self.match_any(TT::Comma) {
                    break;
                }
            }
        }
        // we report the error but keep going
        if let Some(err) = args_err {
            self.smol_error(err);
        }
        let paren = self.consume(TT::RightParen)?;
        Ok(Expr::Call(CallExpr {
            callee: expr.into(),
            paren,
            args,
        }))
    }

    fn smol_error(&mut self, err: LineError) {
        error!("{err}");
        self.errs.push(err)
    }

    // primary → NUMBER | STRING | "true" | "false" | "nil"
    //           | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, LineError> {
        if self.match_any(TT::False) {
            return Ok(Expr::literal(Value::Bool(false)));
        }
        if self.match_any(TT::True) {
            return Ok(Expr::literal(Value::Bool(true)));
        }
        if self.match_any(TT::Nil) {
            return Ok(Expr::literal(Value::Nil));
        }
        if self.match_any([TT::Number, TT::String]) {
            let prev = self.previous();
            return Ok(Expr::literal(prev.literal.unwrap()));
        }
        if self.match_any(TT::Identifier) {
            let name = self.previous();
            return Ok(Expr::Var(VarExpr { name }));
        }
        if self.match_any(TT::LeftParen) {
            let expr = self.expr()?;
            self.consume(TT::RightParen)?;
            return Ok(Expr::group(expr));
        }
        Err(LineError::ExpectedExpr {
            line: self.peek().line,
        })
    }

    fn match_any(&mut self, types: impl IntoIterator<Item = TT>) -> bool {
        for typ in types {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, typ: TT) -> Result<Token, LineError> {
        if self.match_any(typ) {
            return Ok(self.previous());
        }
        Err(self.expected_typ_error(typ))
    }

    fn expected_typ_error(&self, typ: TT) -> LineError {
        LineError::Expected {
            expected: typ,
            actual: self.peek().typ,
            line: self.peek().line,
        }
    }

    fn check(&self, typ: TT) -> bool {
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
        self.tokens.get(self.current).map(|t| t.typ) == Some(TT::Eof)
    }
}
