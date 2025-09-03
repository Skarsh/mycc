use crate::lexer::token::Token;

pub mod token;

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

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let mut token = None;
        if let Some(ch) = self.read_char() {
            match ch {
                '(' => {
                    token = Some(Token::LParen);
                    self.advance();
                }
                ')' => {
                    token = Some(Token::RParen);
                    self.advance();
                }
                '{' => {
                    token = Some(Token::LBrace);
                    self.advance();
                }
                '}' => {
                    token = Some(Token::RBrace);
                    self.advance();
                }
                ';' => {
                    token = Some(Token::Semicolon);
                    self.advance();
                }
                _ => {
                    if ch.is_digit(10) {
                        let constant: u32 = self.read_constant();
                        token = Some(Token::Constant(constant));
                    } else if ch.is_alphabetic() {
                        let ident = self.read_identifier();

                        match ident.as_str() {
                            "int" => token = Some(Token::Int),
                            "void" => token = Some(Token::Void),
                            "return" => token = Some(Token::Return),
                            _ => token = Some(Token::Identifier(ident)),
                        }
                    }
                }
            }
        }
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
        let test_program = r#"int main(void) {
    return 2;
}
        "#;

        let expected_tokens: &[_] = &[
            Token::Int,
            Token::Identifier("main".to_string()),
            Token::LParen,
            Token::Void,
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Constant(2),
            Token::Semicolon,
            Token::RBrace,
        ];

        let mut lexer = Lexer::new(test_program);

        for token in expected_tokens {
            println!("expected token: {:?}", token);
            let tkn = lexer.next_token().unwrap();
            println!("tkn: {:?}", tkn);
            assert_eq!(*token, tkn);
        }
    }
}
