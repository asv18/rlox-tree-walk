use std::fmt::{Display, Formatter};

use super::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Token {
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
        match &self.literal {
            Some(literal) => write!(f, "{:?} {} {}", self.token_type, &self.lexeme, literal),
            _ => write!(f, "{:?} {} None", self.token_type, self.lexeme)
        }
    }
}
