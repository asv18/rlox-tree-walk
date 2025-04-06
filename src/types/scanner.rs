use std::collections::VecDeque;

use crate::error;

use super::{token::Token, token_type::TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Self {
            source: source.chars().collect::<Vec<char>>(),
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

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.check_next('=') {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            },
            '=' => {
                if self.check_next('=') {
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            },
            '<' => {
                if self.check_next('=') {
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            },
            '>' => {
                if self.check_next('=') {
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            },
            _ => {
               error(self.line, "Unknown Character") 
            } 
        };
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];

        self.current += 1;
        c
    }

    fn check_next(&mut self, expected_char: char) -> bool {
        if self.is_at_end() { return false }

        let res = self.source[self.current] == expected_char;
        self.current += 1;

        res
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_inner(token_type, None);
    }

    fn add_token_inner(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].iter().collect::<String>();

        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }
}
