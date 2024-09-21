// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

use crate::tokenizer::Token;

pub struct Builder<'b> {
    tokens: &'b Vec<Token>,
    current: usize,
    previous: usize,
    output_str: String,
}

impl<'b> Builder<'b> {
    pub fn new(tokens: &'b Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            previous: 0,
            output_str: String::new(),
        }
    }

    pub fn build(&mut self) -> &String {
        &self.output_str
    }
}
