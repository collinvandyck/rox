#![allow(unused)]

pub mod prelude;

use std::fmt::Display;

use anyhow::bail;
use prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

type Result<T> = std::result::Result<T, Error>;

pub struct Lox {
    err: Option<Error>,
}

impl Lox {
    pub fn new() -> Self {
        Self { err: None }
    }

    pub fn run(&mut self, prog: String) -> Result<()> {
        Ok(())
    }

    pub fn had_error(&self) -> bool {
        self.err.is_some()
    }

    pub fn clear_err(&mut self) {
        self.err = None;
    }
}

#[derive(Clone, Debug)]
struct Token {
    pub typ: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.typ, self.lexeme, self.literal)
    }
}

#[derive(Clone, Debug, strum_macros::Display)]
pub enum Literal {}

#[derive(Clone, Copy, Debug, strum_macros::Display)]
enum TokenType {
    // single character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // more than one char
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
