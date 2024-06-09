use crate::prelude::*;
use anyhow::bail;
use itertools::Itertools;
use std::{fmt::Display, io};
use tracing_subscriber::field::display;

#[derive(thiserror::Error, Debug)]
pub enum LoxError {
    #[error(transparent)]
    Scan(#[from] ScanError),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Interpret(#[from] interpreter::Error),
}

#[derive(Default)]
pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, prog: impl AsRef<str>) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(prog);
        let tokens = scanner.scan_tokens().map_err(LoxError::Scan)?;
        // Parser::parse should take a &[Token] instead.
        if let Ok(expr) = parser::Parser::new(tokens.clone()).single_expr() {
            let val = self.interpreter.evaluate(&expr)?;
            println!("{val}");
        } else {
            let mut parser = parser::Parser::new(tokens);
            let stmts = parser.parse().map_err(LoxError::Parse)?;
            self.interpreter
                .interpret(&stmts)
                .map_err(LoxError::Interpret)?;
        }
        Ok(())
    }

    pub fn stdout(mut self, w: impl Into<Box<dyn io::Write>>) -> Self {
        self.interpreter = self.interpreter.with_stdout(w.into());
        self
    }

    pub fn stderr(mut self, w: impl Into<Box<dyn io::Write>>) -> Self {
        self.interpreter = self.interpreter.with_stderr(w.into());
        self
    }
}
