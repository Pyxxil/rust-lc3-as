use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Stringz {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Stringz {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
            operands: Vec::with_capacity(1),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Stringz {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Stringz {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            match token {
                Token::String(_) => self.operands.push(tokens.pop_front().unwrap()),
                tok => expected(
                    &["String"],
                    &tok,
                    (self.column, self.line, self.token().len()),
                ),
            }
        } else {
            expected(
                &["String"],
                &Token::EOL,
                (self.column, self.line, self.token().len()),
            );
        }

        let mut extra_strings = tokens
            .drain_while(|token| match token {
                Token::String(_) => true,
                _ => false,
            })
            .collect::<Vec<_>>();

        self.operands.append(&mut extra_strings);

        tokens
    }
}
