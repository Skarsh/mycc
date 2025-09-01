use std::fmt::Display;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Constant(u32),

    // Keywords
    Int(i32),
    Void,
    Return,

    // Operators
    LParen,
    RParen,
    LBrace,
    RBrace,
    SemiColon,
}

// By implementing Display here we get the String::to_string trait for free.
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(val) => write!(f, "{}", val),
            Self::Constant(val) => write!(f, "{}", val),
            Self::Int(val) => write!(f, "{}", val),
            Self::Void => write!(f, "void"),
            Self::Return => write!(f, "return"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::SemiColon => write!(f, ";"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        token: Token,
        value: String,
    }

    #[test]
    fn test_token_to_string() {
        let test_cases: &[_] = &[
            TestCase {
                token: Token::Identifier("my_var".to_string()),
                value: "my_var".to_string(),
            },
            TestCase {
                token: Token::Constant(42),
                value: "42".to_string(),
            },
            TestCase {
                token: Token::Int(14),
                value: "14".to_string(),
            },
            TestCase {
                token: Token::Int(-14),
                value: "-14".to_string(),
            },
            TestCase {
                token: Token::Void,
                value: "void".to_string(),
            },
            TestCase {
                token: Token::Return,
                value: "return".to_string(),
            },
            TestCase {
                token: Token::LParen,
                value: "(".to_string(),
            },
            TestCase {
                token: Token::RParen,
                value: ")".to_string(),
            },
            TestCase {
                token: Token::LBrace,
                value: "{".to_string(),
            },
            TestCase {
                token: Token::RBrace,
                value: "}".to_string(),
            },
            TestCase {
                token: Token::SemiColon,
                value: ";".to_string(),
            },
        ];

        for case in test_cases {
            assert_eq!(case.token.to_string(), case.value)
        }
    }
}
