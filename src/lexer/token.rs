#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier,
    Constant,

    // Keywords
    Int,
    Void,
    Return,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,

    _Eof,
    _Illegal,
}

#[derive(Debug, PartialEq)]
pub struct TokenSpan {
    start: usize,
    end: usize,
}

impl TokenSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: TokenSpan,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: TokenSpan, literal: String) -> Self {
        Self {
            kind,
            span,
            literal,
        }
    }
}
