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
    Operator(char),
    Literal(String),
    OpenBrace,
    ClosedBrace,
    OpenParen,
    ClosedParen,
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
        self.skip_whitespace();
        self.start = self.current;

        match self.advance() {
            '"' => self.skip_string(),
            '{' => self.tokens.push(Token::OpenBrace),
            '}' => self.tokens.push(Token::ClosedBrace),
            '(' => self.tokens.push(Token::OpenParen),
            ')' => self.tokens.push(Token::ClosedParen),
            ',' => self.tokens.push(Token::Comma),
            ';' => self.tokens.push(Token::Semicolon),
            c => {
                if self.is_operator(c) {
                    self.tokens.push(Token::Operator(c));
                } else {
                    let result = self.tokenize_literal();
                    self.tokens.push(result);
                }
            }
        };
    }

    fn tokenize_literal(&mut self) -> Token {
        while (self.peek().is_alphabetic() || self.peek().is_numeric()) && !self.is_at_end() {
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

    //TODO check for is_at_end
    fn skip_string(&mut self) -> () {
        while self.peek() != '"' {
            self.advance();
        }

        let value = self.source[self.start..self.current].to_string();

        self.tokens.push(Token::String(value));
    }

    fn skip_whitespace(&mut self) -> () {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                }
                _ => return,
            }
        }
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap_or_else(|| {
            panic!("Out of bounce index in source string at {}", self.current);
        })
    }

    fn advance(&mut self) -> char {
        let char = self.peek();
        self.previous = self.current;
        self.current += 1;

        return char;
    }

    fn is_at_end(&self) -> bool {
        return self.current == self.source.len() - 1;
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
            Token::Operator(s) => write!(f, "Operator: {}", s),
            Token::Literal(s) => write!(f, "Literal: {}", s),
            Token::OpenParen => write!(f, "OpenParen"),
            Token::ClosedParen => write!(f, "ClosedParen"),
            Token::OpenBrace => write!(f, "OpenBrace"),
            Token::ClosedBrace => write!(f, "ClosedBrace"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, Token};

    #[test]
    fn test_scan_source() {
        let input = String::from("pink x =1+2 ; overtune{ something(1, 2)  other }\0");
        let expected_output = vec![
            Token::Literal("pink".to_string()),
            Token::Literal("x".to_string()),
            Token::Operator('='),
            Token::Literal("1".to_string()),
            Token::Operator('+'),
            Token::Literal("2".to_string()),
            Token::Semicolon,
            Token::Literal("overtune".to_string()),
            Token::OpenBrace,
            Token::Literal("something".to_string()),
            Token::OpenParen,
            Token::Literal("1".to_string()),
            Token::Comma,
            Token::Literal("2".to_string()),
            Token::ClosedParen,
            Token::Literal("other".to_string()),
            Token::ClosedBrace,
        ];

        let mut scanner = Scanner::new(&input);
        let output = scanner.scan_source();

        assert_eq!(output.len(), expected_output.len());

        for (index, token) in output.iter().enumerate() {
            assert_eq!(expected_output[index], *token);
        }
    }
}
