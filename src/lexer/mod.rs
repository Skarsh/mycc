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
        self.skip_whitespace();
        let mut token = Token::Illegal;
        if let Some(ch) = self.read_char() {
            match ch {
                '(' => token = Token::LParen,
                ')' => token = Token::RParen,
                '{' => token = Token::LBrace,
                '}' => token = Token::RBrace,
                ';' => token = Token::Semicolon,
                _ => {
                    if ch.is_digit(10) {
                        let constant: u32 = self.read_constant();
                        token = Token::Constant(constant);
                    } else if ch.is_alphabetic() {
                        let ident = self.read_identifier();

                        match ident.as_str() {
                            "int" => token = Token::Int,
                            "void" => token = Token::Void,
                            "return" => token = Token::Return,
                            _ => token = Token::Identifier(ident),
                        }
                    }
                }
            }
        }
        self.advance();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while let Some(ch) = self.read_char() {
            if ch.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
        self.source[start_pos..self.position].iter().collect()
    }

    fn read_constant(&mut self) -> u32 {
        let start_pos = self.position;
        while let Some(ch) = self.read_char() {
            if ch.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }
        let lit: String = self.source[start_pos..self.position].iter().collect();
        let constant = lit.parse().unwrap();
        constant
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.read_char() {
            if ch.is_whitespace() {
                self.advance()
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn read_char(&self) -> Option<&char> {
        self.source.get(self.position)
    }

    fn peek_char(&self) -> Option<&char> {
        self.source.get(self.position + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_tokens() {
        let expected_tokens: &[_] = &[
            Token::Int,
            Token::Identifier("var".to_string()),
            Token::Constant(123),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Semicolon,
            Token::Void,
            Token::Return,
        ];

        let my_str = "int var 123 () {}; void return";
        let mut lexer = Lexer::new(my_str);

        for token in expected_tokens {
            let tkn = lexer.next_token();
            assert_eq!(*token, tkn);
        }
    }
}
