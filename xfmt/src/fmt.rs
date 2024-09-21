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
}

impl Formatter {
    pub fn new(source: &String) -> Self {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_source();

        Self {
            tokens: tokens.to_vec(),
            current: 0,
            previous: 0,
        }
    }

    pub fn format(&mut self) -> String {
        while self.is_at_end() {
            self.update_tokens();
        }

        let mut builder = Builder::new(&self.tokens);
        builder.build().to_string()
    }

    fn update_tokens(&mut self) -> () {
        todo!()
    }

    fn advance(&mut self) -> &Token {
        let token: &Token = &self.tokens[self.current];
        self.previous = self.current;
        self.current += 1;

        return token;
    }

    fn peek(&mut self) -> &Token {
        return &self.tokens[self.current];
    }

    fn peek_next(&mut self) -> &Token {
        return &self.tokens[self.current + 1];
    }

    fn is_at_end(&self) -> bool {
        self.tokens.len() == self.current
    }
}
