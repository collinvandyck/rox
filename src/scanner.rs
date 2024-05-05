use crate::prelude::*;

#[derive(Debug)]
pub struct ScanError {
    errs: Vec<LineError>,
}

impl std::error::Error for ScanError {}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = self.errs.iter().map(|err| err.to_string()).join("\n");
        write!(f, "{msg}")
    }
}

#[derive(thiserror::Error, Debug)]
#[error("line: {line}: {msg}")]
pub struct LineError {
    line: usize,
    msg: String,
}

#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    chars: Vec<(usize, char)>,
    start: usize,
    current: usize,
    line: usize,
    errs: Vec<LineError>,
}

impl Scanner {
    pub fn new(source: impl AsRef<str>) -> Self {
        let source = source.as_ref().to_string();
        let chars = source.char_indices().collect_vec();
        Self {
            source,
            chars,
            line: 1,
            ..Default::default()
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, ScanError> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token_lexeme(TokenType::Eof, Lexeme::default());
        if !self.errs.is_empty() {
            Err(ScanError { errs: self.errs })
        } else {
            Ok(self.tokens)
        }
    }

    fn scan_token(&mut self) {
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
        ch.is_ascii_uppercase() || ch.is_ascii_lowercase() || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
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
        let err = LineError {
            line: self.line,
            msg: msg.to_string(),
        };
        eprintln!("{err}");
        self.errs.push(err);
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
        Lexeme(self.chars[start..end].iter().map(|t| t.1).collect())
    }

    fn at_end(&self) -> bool {
        self.current >= self.chars.len()
    }
}

#[derive(Default, Clone, Debug, derive_more::Display, PartialEq, derive_more::From)]
pub struct Lexeme(String);

impl From<&str> for Lexeme {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

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

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub lexeme: Lexeme,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: Lexeme, literal: Option<Literal>, line: usize) -> Self {
        Self {
            typ,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {} {}]", self.typ, self.lexeme, self.line)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{n}"),
            Literal::String(s) => write!(f, r#""{s}""#),
            Literal::Bool(v) => write!(f, "{v}"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone, Copy, Debug, strum_macros::Display, PartialEq)]
pub enum TokenType {
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
