#![allow(unused)]

pub mod env;
pub mod expr;
pub mod interpret;
pub mod parse;
pub mod prelude;
pub mod scanner;
pub mod stmt;

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

#[cfg(test)]
mod tests {
    use crate::{parse::Parser, Lexeme, Literal, Lox, ParseError, Scanner, Token, TokenType};
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn test_tokens() {
        for (prog, ex) in [
            (
                "",
                vec![Token {
                    typ: TokenType::Eof,
                    lexeme: Lexeme::default(),
                    literal: None,
                    line: 1,
                }],
            ),
            (
                "3",
                vec![
                    Token {
                        typ: TokenType::Number,
                        lexeme: Lexeme::from("3"),
                        literal: Some(Literal::Number(3.0)),
                        line: 1,
                    },
                    Token {
                        typ: TokenType::Eof,
                        lexeme: Lexeme::default(),
                        literal: None,
                        line: 1,
                    },
                ],
            ),
            (
                r#""foo""#,
                vec![
                    Token {
                        typ: TokenType::String,
                        lexeme: Lexeme::from("foo"),
                        literal: Some(Literal::String("foo".into())),
                        line: 1,
                    },
                    Token {
                        typ: TokenType::Eof,
                        lexeme: Lexeme::default(),
                        literal: None,
                        line: 1,
                    },
                ],
            ),
        ] {
            let mut s = Scanner::new(prog);
            let toks = s.scan_tokens().unwrap();
            assert_eq!(toks, ex, "prog '{prog}' produced {toks:#?}");
        }
    }
}
