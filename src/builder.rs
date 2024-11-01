// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

use crate::tokenizer::Token;

const TAB_SIZE: &str = "   ";

pub struct Builder<'b> {
    tokens: &'b Vec<Token>,
    output_str: String,
}

impl<'b> Builder<'b> {
    pub fn new(tokens: &'b Vec<Token>) -> Self {
        Self {
            tokens,
            output_str: String::new(),
        }
    }

    pub fn build(&mut self) -> &String {
        self.tokens.iter().for_each(|token| match token {
            Token::Space => self.output_str.push(' '),
            Token::Comma => self.output_str.push(','),
            Token::Tab => self.output_str.push_str(TAB_SIZE),
            Token::NewLine => self.output_str.push('\n'),
            Token::Semicolon => self.output_str.push(';'),
            Token::OpenBrace => self.output_str.push('{'),
            Token::ClosedBrace => self.output_str.push('}'),
            Token::OpenParen => self.output_str.push('('),
            Token::ClosedParen => self.output_str.push(')'),
            Token::Literal(s) => self.output_str.push_str(s),
            Token::Operator(s) => self.output_str.push(*s),
            t => panic!("{} is not implemented", t),
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
            Token::Literal("a".to_string()),
            Token::Space,
            Token::Operator('+'),
            Token::Space,
            Token::Literal("a".to_string()),
            Token::Space,
            Token::OpenBrace,
            Token::NewLine,
            Token::Tab,
            Token::Literal("a".to_string()),
            Token::Space,
            Token::ClosedBrace,
            Token::NewLine,
            Token::Literal("a".to_string()),
            Token::OpenParen,
            Token::ClosedParen,
        ];

        let mut builder = Builder::new(&input);
        let output = builder.build();

        let expected_output = " a + a {\n   a }\na()";

        assert_eq!(output, expected_output);
    }
}
