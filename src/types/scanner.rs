use std::collections::VecDeque;

use super::{token::Token, token_type::TokenType};

pub struct Scanner {
    source: VecDeque<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Self {
            source: source.chars().collect::<VecDeque<char>>(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(EOF, String::new(), None, self.line));    
    }

    fn is_at_end(&self) -> bool {
        self.source.len() == 0
    }

    fn scan_token(&self) {
        let c: char = self.advance();

        match c {
            '(' => self.addToken(TokenType::LEFT_PAREN),
            ')' => self.addToken(TokenType::RIGHT_PAREN),
            '{' => self.addToken(TokenType::LEFT_BRACE),
            '}' => self.addToken(TokenType::RIGHT_BRACE),
            ',' => self.addToken(TokenType::COMMA),
            '.' => self.addToken(TokenType::DOT),
            '-' => self.addToken(TokenType::MINUS),
            '+' => self.addToken(TokenType::PLUS),
            ';' => self.addToken(TokenType::SEMICOLON),
            '*' => self.addToken(TokenType::STAR),
        };
    }

    fn advance(&mut self) -> char {
        if let Some(c) = self.source.pop_front() {
            return c;
        }

        ' '
    }
}
