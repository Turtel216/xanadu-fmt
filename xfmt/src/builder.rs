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
        self.tokens.iter().for_each(|token| match token {
            Token::Space => self.output_str.push(' '),
            Token::Tab => self.output_str.push_str("   "),
            Token::NewLine => self.output_str.push('\n'),
            Token::Semicolon => self.output_str.push(';'),
            Token::OpenBrace => self.output_str.push('{'),
            Token::ClosedBrace => self.output_str.push('}'),
            Token::Literal(s) => self.output_str.push_str(s),
            Token::Operator(s) => self.output_str.push(*s),
            _ => todo!(),
        });

        &self.output_str
    }
}

#[cfg(test)]
mod tests {

    use crate::tokenizer::Token;

    use super::Builder;

    #[test]
    fn test_build_output_string() {
        let input = vec![
            Token::Space,
            Token::Operator('+'),
            Token::Space,
            Token::Literal("b".to_string()),
            Token::Space,
            Token::OpenBrace,
            Token::NewLine,
            Token::Tab,
            Token::Literal("a".to_string()),
            Token::Space,
            Token::ClosedBrace,
            Token::NewLine,
        ];

        let mut builder = Builder::new(&input);
        let output = builder.build();

        let expected_output = " + b {\n   a }\n";

        assert_eq!(output, expected_output);
    }
}
