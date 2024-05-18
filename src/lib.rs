#![allow(unused)]

pub mod env;
pub mod expr;
pub mod func;
pub mod interpret;
pub mod parse;
pub mod prelude;
pub mod scanner;
pub mod stmt;
#[cfg(test)]
mod tests;

use anyhow::bail;
use itertools::Itertools;
use prelude::*;
use std::fmt::Display;
use tracing_subscriber::field::display;

#[derive(thiserror::Error, Debug)]
pub enum LoxError {
    #[error(transparent)]
    Scan(#[from] ScanError),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Interpret(#[from] interpret::Error),
}

#[derive(Default)]
pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, prog: String) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(prog);
        let tokens = scanner.scan_tokens().map_err(LoxError::Scan)?;
        // Parser::parse should take a &[Token] instead.
        if let Ok(expr) = parse::Parser::new(tokens.clone()).single_expr() {
            let val = self.interpreter.evaluate(&expr)?;
            println!("{val}");
        } else {
            let mut parser = parse::Parser::new(tokens);
            let stmts = parser.parse().map_err(LoxError::Parse)?;
            self.interpreter
                .interpret(&stmts)
                .map_err(LoxError::Interpret)?;
        }
        Ok(())
    }
}
