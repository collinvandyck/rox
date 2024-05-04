#![allow(unused)]

pub mod prelude;

use std::fmt::Display;

use anyhow::bail;
use itertools::Itertools;
use prelude::*;
use tracing_subscriber::field::display;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("line: {line}: {msg}")]
    Line { line: usize, msg: String },
}

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

#[derive(Default)]
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    chars: Vec<(usize, char)>,
    start: usize,
    current: usize,
    line: usize,
    errs: Vec<Error>,
}

impl Scanner {
    fn new(source: String) -> Self {
        let chars = source.char_indices().collect_vec();
        Self {
            source,
            chars,
            line: 1,
            ..Default::default()
        }
    }

    fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.add_token(TokenType::Eof);
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<()> {
        use TokenType::*;
        let (idx, ch) = self.advance();
        match ch {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                if self.try_match('=') {
                    self.add_token(BangEqual);
                } else {
                    self.add_token(Bang);
                }
            }
            '=' => {
                if self.try_match('=') {
                    self.add_token(EqualEqual);
                } else {
                    self.add_token(Equal);
                }
            }
            '<' => {
                if self.try_match('=') {
                    self.add_token(LessEqual);
                } else {
                    self.add_token(Less);
                }
            }
            '>' => {
                if self.try_match('=') {
                    self.add_token(GreaterEqual);
                } else {
                    self.add_token(Greater);
                }
            }
            '/' => {
                if self.try_match('/') {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            '\n' => {
                self.line += 1;
            }
            ' ' | '\r' | '\t' => {}
            '"' => self.string(),
            _ => {
                if Self::is_digit(ch) {
                    self.number();
                } else if Self::is_alpha(ch) {
                    self.identifier();
                } else {
                    self.error("unexpected char.");
                }
            }
        }
        todo!()
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let lexeme = self.lexeme();
        let typ = lexeme.identifier_type();
        self.add_token_lexeme(typ, lexeme);
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
        }
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        let lexeme = self.lexeme();
        let literal = lexeme.number();
        self.add_token_lexeme_literal(TokenType::Number, lexeme, Some(literal));
    }

    fn is_alpha_numeric(ch: char) -> bool {
        Self::is_alpha(ch) || Self::is_digit(ch)
    }

    fn is_alpha(ch: char) -> bool {
        (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z') || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
        }
        if self.at_end() {
            self.error("unterminated string");
            return;
        }
        self.advance(); // "
        let lexeme = self.lexeme_at(self.start + 1, self.current - 1);
        let literal = lexeme.string();
        self.add_token_lexeme_literal(TokenType::String, lexeme, Some(literal));
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.current_char()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            '\0'
        } else {
            self.char_at(self.current + 1)
        }
    }

    fn try_match(&mut self, ch: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.current_char() != ch {
            return false;
        }
        self.current += 1;
        true
    }

    fn current_char(&self) -> char {
        self.char_at(self.current)
    }

    fn char_at(&self, pos: usize) -> char {
        self.chars[pos].1
    }

    fn error(&mut self, msg: &str) {
        self.errs.push(Error::Line {
            line: self.line,
            msg: msg.to_string(),
        });
    }

    fn advance(&mut self) -> (usize, char) {
        let next = self.chars[self.current];
        self.current += 1;
        next
    }

    fn add_token(&mut self, typ: TokenType) {
        self.add_token_lexeme(typ, self.lexeme());
    }

    fn add_token_lexeme(&mut self, typ: TokenType, lexeme: Lexeme) {
        self.add_token_lexeme_literal(typ, lexeme, None);
    }

    fn add_token_lexeme_literal(
        &mut self,
        typ: TokenType,
        lexeme: Lexeme,
        literal: Option<Literal>,
    ) {
        let mut token = Token {
            typ,
            lexeme,
            literal,
            line: self.line,
        };
        self.tokens.push(token);
    }

    fn lexeme(&self) -> Lexeme {
        self.lexeme_at(self.start, self.current)
    }

    fn lexeme_at(&self, start: usize, end: usize) -> Lexeme {
        Lexeme(self.chars[start..end].into_iter().map(|t| t.1).collect())
    }

    fn at_end(&self) -> bool {
        self.current >= self.chars.len()
    }
}

#[derive(Default, Clone, Debug, derive_more::Display)]
struct Lexeme(String);

impl Lexeme {
    fn number(&self) -> Literal {
        Literal::Number(self.0.parse::<f64>().unwrap())
    }
    fn string(&self) -> Literal {
        Literal::String(self.0.clone())
    }
    fn identifier_type(&self) -> TokenType {
        use TokenType::*;
        match self.0.as_str() {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            _ => Identifier,
        }
    }
}

#[derive(Clone, Debug, derive_more::Display)]
#[display(fmt = "{typ} {lexeme:?} {literal:?} {line}")]
struct Token {
    pub typ: TokenType,
    pub lexeme: Lexeme,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {}

#[derive(Clone, Debug, strum_macros::Display)]
pub enum Literal {
    Number(f64),
    String(String),
}

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
