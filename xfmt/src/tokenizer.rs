// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    NewLine,
    Space,
    Tab,
    Comma,
    Semicolon,
    String(String),
    Class(String),
    Function(String),
    Operator(char),
    Literal(String),
    OpenBrace,
    ClosedBrace,
}

pub struct Scanner<'s> {
    current: usize,
    previous: usize,
    start: usize,
    source: &'s String,
    tokens: Vec<Token>,
}

impl<'s> Scanner<'s> {
    pub fn new(source: &'s String) -> Self {
        Self {
            current: 0,
            previous: 0,
            start: 0,
            source,
            tokens: Vec::new(),
        }
    }

    pub fn scan_source(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }

        return &self.tokens;
    }

    fn scan_token(&mut self) -> () {
        self.start = self.current;

        match self.advance() {
            '\n' => self.tokens.push(Token::NewLine),
            '"' => self.skip_string(),
            ',' => self.tokens.push(Token::Comma),
            ';' => self.tokens.push(Token::Semicolon),
            c if self.is_operator(c) => self.tokens.push(Token::Operator(c)),
            _ => {
                let result = self.tokenize_literal();

                self.tokens.push(result);
            }
        };
    }

    fn tokenize_literal(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_numeric() {
            self.advance();
        }

        // Create current lexeme
        let value: &str = self.source[self.start..self.current].into();

        match self.is_keyword(value) {
            Some(token) => token,
            None => Token::Literal(value.to_string()),
        }
    }

    fn is_keyword(&self, value: &str) -> Option<Token> {
        match value {
            //"overtune" => Some(Token::Class(value.to_string())),
            //"subdivision" => Some(Token::Function(value.to_string())),
            _ => None,
        }
    }

    fn is_operator(&mut self, character: char) -> bool {
        match character {
            '+' => true,
            '-' => true,
            '*' => true,
            '=' => true,
            ':' => true,
            _ => false,
        }
    }

    fn skip_string(&mut self) -> () {
        while self.peek() != '"' {
            self.advance();
        }

        let value = self.source[self.start..self.current].to_string();

        self.tokens.push(Token::String(value));
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or_else(|| {
            panic!("Out of bounce index in source string at {}", self.current);
        })
    }

    fn advance(&mut self) -> char {
        self.previous = self.current;
        self.source
            .chars()
            .nth(self.current)
            .unwrap_or_else(|| panic!("Out of bounce source String index at {}", self.current))
    }

    fn is_at_end(&mut self) -> bool {
        return self.source.len() == self.current;
    }
}

use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Space => write!(f, "Space"),
            Token::NewLine => write!(f, "NewLine"),
            Token::Tab => write!(f, "Tab"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::String(s) => write!(f, "String: {}", s),
            Token::Class(s) => write!(f, "Class: {}", s),
            Token::Function(s) => write!(f, "Function: {}", s),
            Token::Operator(s) => write!(f, "Operator: {}", s),
            Token::Literal(s) => write!(f, "Literal: {}", s),
            Token::OpenBrace => write!(f, "OpenBrace"),
            Token::ClosedBrace => write!(f, "ClosedBrace"),
        }
    }
}
