use crate::lexer::token::{Token, TokenKind, TokenSpan};

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
                    token = Some(Token::new(
                        TokenKind::LParen,
                        TokenSpan::new(0, 0),
                        "(".to_string(),
                    ));
                    self.advance();
                }
                ')' => {
                    token = Some(Token::new(
                        TokenKind::RParen,
                        TokenSpan::new(0, 0),
                        ")".to_string(),
                    ));
                    self.advance();
                }
                '{' => {
                    token = Some(Token::new(
                        TokenKind::LBrace,
                        TokenSpan::new(0, 0),
                        "{".to_string(),
                    ));
                    self.advance();
                }
                '}' => {
                    token = Some(Token::new(
                        TokenKind::RBrace,
                        TokenSpan::new(0, 0),
                        "}".to_string(),
                    ));
                    self.advance();
                }
                ';' => {
                    token = Some(Token::new(
                        TokenKind::Semicolon,
                        TokenSpan::new(0, 0),
                        ";".to_string(),
                    ));
                    self.advance();
                }
                _ => {
                    if ch.is_digit(10) {
                        let constant: String = self.read_constant();
                        token = Some(Token::new(
                            TokenKind::Constant,
                            TokenSpan::new(0, 0),
                            constant,
                        ));
                    } else if ch.is_alphabetic() {
                        let ident = self.read_identifier();

                        match ident.as_str() {
                            "int" => {
                                token = Some(Token::new(
                                    TokenKind::Int,
                                    TokenSpan::new(0, 0),
                                    "int".to_string(),
                                ))
                            }
                            "void" => {
                                token = Some(Token::new(
                                    TokenKind::Void,
                                    TokenSpan::new(0, 0),
                                    "void".to_string(),
                                ))
                            }
                            "return" => {
                                token = Some(Token::new(
                                    TokenKind::Return,
                                    TokenSpan::new(0, 0),
                                    "return".to_string(),
                                ))
                            }
                            _ => {
                                token = Some(Token::new(
                                    TokenKind::Identifier,
                                    TokenSpan::new(0, 0),
                                    ident,
                                ))
                            }
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

    fn read_constant(&mut self) -> String {
        let start_pos = self.position;
        while let Some(ch) = self.read_char() {
            if ch.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }
        self.source[start_pos..self.position].iter().collect()
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
            Token::new(TokenKind::Int, TokenSpan::new(0, 0), "int".to_string()),
            Token::new(
                TokenKind::Identifier,
                TokenSpan::new(0, 0),
                "main".to_string(),
            ),
            Token::new(TokenKind::LParen, TokenSpan::new(0, 0), "(".to_string()),
            Token::new(TokenKind::Void, TokenSpan::new(0, 0), "void".to_string()),
            Token::new(TokenKind::RParen, TokenSpan::new(0, 0), ")".to_string()),
            Token::new(TokenKind::LBrace, TokenSpan::new(0, 0), "{".to_string()),
            Token::new(
                TokenKind::Return,
                TokenSpan::new(0, 0),
                "return".to_string(),
            ),
            Token::new(TokenKind::Constant, TokenSpan::new(0, 0), "2".to_string()),
            Token::new(TokenKind::Semicolon, TokenSpan::new(0, 0), ";".to_string()),
            Token::new(TokenKind::RBrace, TokenSpan::new(0, 0), "}".to_string()),
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
