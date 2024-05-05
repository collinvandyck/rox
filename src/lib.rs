#![allow(unused)]

pub mod expr;
pub mod parse;
pub mod prelude;
pub mod scanner;

use std::fmt::Display;

use anyhow::bail;
use itertools::Itertools;
use prelude::*;
use tracing_subscriber::field::display;

#[derive(Default)]
pub struct Lox {
    err: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, prog: String) {
        let mut scanner = Scanner::new(prog);
        let tokens = scanner.scan_tokens();
        self.err = scanner.had_error();
        if self.err {
            return;
        }
        let mut parser = parse::Parser::new(tokens);
        let expr = match parser.parse() {
            Ok(expr) => expr,
            Err(err) => {
                eprintln!("{err}");
                return;
            }
        };
        let printer = AstPrinter {};
        let s = printer.print(&expr);
        println!("{s}");
    }

    pub fn had_error(&self) -> bool {
        self.err
    }

    pub fn clear_err(&mut self) {
        self.err = false;
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse::Parser, Lexeme, Literal, Lox, ParseError, Scanner, Token, TokenType};
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn test_parser() {
        let toks = vec![
            Token {
                typ: TokenType::Identifier,
                lexeme: Lexeme::from("hi"),
                literal: None,
                line: 1,
            },
            Token {
                typ: TokenType::Eof,
                lexeme: Lexeme::default(),
                literal: None,
                line: 1,
            },
        ];
        let mut parser = Parser::new(toks);
        assert!(matches!(parser.parse(), Err(ParseError::Failed)));
    }

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
            let toks = s.scan_tokens();
            assert_eq!(toks, ex, "prog '{prog}' produced {toks:#?}");
        }
    }
}
