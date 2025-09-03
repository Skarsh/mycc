use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(u32),

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

    Eof,
    Illegal,
}

// By implementing Display here we get the String::to_string trait for free.
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(val) => write!(f, "{}", val),
            Self::Constant(val) => write!(f, "{}", val),
            Self::Int => write!(f, "int"),
            Self::Void => write!(f, "void"),
            Self::Return => write!(f, "return"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::Semicolon => write!(f, ";"),
            Self::Eof => write!(f, "EOF"),
            Self::Illegal => write!(f, "illegal"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        token: Token,
        literal: &'a str,
    }

    #[test]
    fn test_token_to_string() {
        let test_cases: &[_] = &[
            TestCase {
                token: Token::Identifier("my_var".to_string()),
                literal: "my_var",
            },
            TestCase {
                token: Token::Constant(42),
                literal: "42",
            },
            TestCase {
                token: Token::Int,
                literal: "int",
            },
            TestCase {
                token: Token::Void,
                literal: "void",
            },
            TestCase {
                token: Token::Return,
                literal: "return",
            },
            TestCase {
                token: Token::LParen,
                literal: "(",
            },
            TestCase {
                token: Token::RParen,
                literal: ")",
            },
            TestCase {
                token: Token::LBrace,
                literal: "{",
            },
            TestCase {
                token: Token::RBrace,
                literal: "}",
            },
            TestCase {
                token: Token::Semicolon,
                literal: ";",
            },
        ];

        for case in test_cases {
            assert_eq!(case.token.to_string(), case.literal)
        }
    }
}
