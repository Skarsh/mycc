use crate::lexer::token::Token;

mod token;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            source: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        todo!()
    }

    fn read_identifier(&mut self) -> String {
        todo!()
    }

    fn read_constant(&mut self) -> String {
        todo!()
    }

    fn skip_whitespace(&mut self) {
        todo!()
    }

    fn advance(&mut self) {
        todo!()
    }

    fn current_char(&self) -> Option<char> {
        todo!()
    }

    fn peek_char(&self) -> Option<char> {
        todo!()
    }
}
