use std::fmt::{Display, Formatter};


pub struct Token {
    token_type: super::TokenType,
    lexeme: String,
    literal: super::LiteralType,
    line: usize,
}

impl Token {
    pub fn new(token_type: super::TokenType, lexeme: String, literal: super::LiteralType, line: usize) -> Token {
        Self {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?} {} {:?}", self.token_type, &self.lexeme, self.literal)
    }
}
