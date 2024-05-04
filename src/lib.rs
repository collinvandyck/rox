#![allow(unused)]

pub mod prelude;

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
