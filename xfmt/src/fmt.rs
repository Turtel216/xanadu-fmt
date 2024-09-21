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
        }

        let mut builder = Builder::new(&self.tokens);
        return builder.build().to_string();
    }

    fn update_tokens(&mut self) -> () {
        match self.peek() {
            &Token::Operator(_) => self.space_out(),
            &Token::Literal(_) => self.space_out(),
            &Token::NewLine => {
                if self.depth != 0 {
                    for _ in 0..=self.depth {
                        self.advance();
                        self.tokens.insert(self.current, Token::Tab);
                    }
                }
            }
            &Token::OpenBrace => {
                self.advance();
                self.tokens.insert(self.current, Token::NewLine);
                self.depth += 1;
            }
            &Token::ClosedBrace => {
                self.advance();
                self.tokens.insert(self.current, Token::NewLine);
                self.depth -= 1;
            }
            &Token::Comma => {
                self.advance();
                self.tokens.insert(self.current, Token::NewLine);
            }
            &Token::Semicolon => {
                self.advance();
                self.tokens.insert(self.current, Token::NewLine);
            }
            _ => todo!(),
        }
    }

    fn space_out(&mut self) -> () {
        if self.tokens[self.previous] == Token::Space {
            self.advance();
            self.tokens.insert(self.current, Token::Space);
        } else {
            self.tokens.insert(self.current, Token::Space);
            self.advance();
            self.advance();
            self.tokens.insert(self.current, Token::Space);
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
        let input = vec![Token::Operator('+'), Token::Literal("skldfj".to_string())];

        let expected_output = vec![
            Token::Space,
            Token::Operator('+'),
            Token::Space,
            Token::Literal("skldfj".to_string()),
            Token::Space,
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

        assert_eq!(formatter.tokens.len(), expected_output.len());

        for (index, token) in formatter.tokens.iter().enumerate() {
            assert_eq!(*token, expected_output[index]);
        }
    }
}
