// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

use crate::{
    builder::Builder,
    tokenizer::{Scanner, Token},
};

pub struct Formatter {
    tokens: Vec<Token>,
    current: usize,
    previous: usize,
    depth: usize,
}

impl Formatter {
    pub fn new(source: &String) -> Self {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_source();

        Self {
            tokens: tokens.to_vec(),
            current: 0,
            previous: 0,
            depth: 0,
        }
    }

    pub fn format(&mut self) -> String {
        while !self.is_at_end() {
            self.update_tokens();
            self.advance();
        }

        let mut builder = Builder::new(&self.tokens);
        return builder.build().to_string();
    }

    fn update_tokens(&mut self) -> () {
        match self.peek() {
            &Token::Operator(_) => self.space_out(),
            &Token::Literal(_) => self.space_out(),
            &Token::OpenBrace => {
                self.advance();
                self.tokens.insert(self.current, Token::NewLine);
                self.depth += 1;
                for _ in 0..=self.depth - 1 {
                    self.advance();
                    self.tokens.insert(self.current, Token::Tab);
                }
            }
            &Token::ClosedBrace => {
                self.tokens.insert(self.current, Token::NewLine);
                self.advance();

                if self.depth > 0 {
                    self.depth -= 1;
                }

                self.add_intendation();
            }
            &Token::Comma => {
                if self.tokens[self.previous] == Token::Space {
                    self.tokens.remove(self.previous);
                }

                self.tokens.insert(self.current, Token::Space);
            }
            &Token::Semicolon => {
                if self.tokens[self.previous] == Token::Space {
                    self.tokens.remove(self.previous);
                    self.previous -= 1;
                    self.current -= 1;
                }
                self.advance();
                self.tokens.insert(self.current, Token::NewLine);
                self.add_intendation();
            }
            &Token::Space => self.advance(),
            &Token::NewLine => self.advance(),
            &Token::Tab => self.advance(),
            t => panic!("{} is not implemented", t),
        }
    }

    fn space_out(&mut self) -> () {
        if self.previous == self.current {
            self.advance();
            self.tokens.insert(self.current, Token::Space);
            return;
        }

        let previous = &self.tokens[self.previous];
        match previous {
            Token::Space | Token::Tab | Token::NewLine => {
                self.advance();
                self.tokens.insert(self.current, Token::Space);
            }
            _ => {
                self.tokens.insert(self.current, Token::Space);
                self.advance();
                self.advance();
                self.tokens.insert(self.current, Token::Space);
            }
        }
    }

    fn add_intendation(&mut self) -> () {
        if self.depth != 0 {
            for _ in 0..=self.depth {
                self.advance();
                self.tokens.insert(self.current, Token::Tab);
            }
        }
    }

    fn advance(&mut self) -> () {
        self.previous = self.current;
        self.current += 1;
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn peek_next(&self) -> &Token {
        return &self.tokens[self.current + 1];
    }

    fn is_at_end(&self) -> bool {
        self.tokens.len() == self.current
    }
}

#[cfg(test)]
mod tests {

    use crate::tokenizer::Token;

    use super::Formatter;

    #[test]
    fn test_update_tokens() {
        let input = vec![
            Token::Literal("a".to_string()),
            Token::Operator('='),
            Token::Literal("a".to_string()),
            Token::Semicolon,
            Token::Literal("a".to_string()),
            Token::OpenBrace,
            Token::Literal("a".to_string()),
            Token::ClosedBrace,
        ];

        let expected_output = vec![
            Token::Literal("a".to_string()),
            Token::Space,
            Token::Operator('='),
            Token::Space,
            Token::Literal("a".to_string()),
            Token::Semicolon,
            Token::NewLine,
            Token::Literal("a".to_string()),
            Token::Space,
            Token::OpenBrace,
            Token::NewLine,
            Token::Tab,
            Token::Literal("a".to_string()),
            Token::Space,
            Token::NewLine,
            Token::ClosedBrace,
        ];

        let mut formatter = Formatter {
            tokens: input,
            current: 0,
            previous: 0,
            depth: 0,
        };

        while !formatter.is_at_end() {
            formatter.update_tokens();
            formatter.advance();
        }

        for token in &formatter.tokens {
            println!("Token: {}", token);
        }

        assert_eq!(formatter.tokens.len(), expected_output.len());

        for (index, token) in formatter.tokens.iter().enumerate() {
            assert_eq!(*token, expected_output[index]);
        }
    }
}
