use token::tokens::traits::*;

use token::r#type::Token;

use std::cell::Cell;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Sub {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Sub {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
            operands: Vec::with_capacity(3),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Sub {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Sub {
    fn require_range(&self) -> (u64, u64) {
        (2, 3)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, max) = self.require_range();

        let count = Cell::new(0);

        self.operands = tokens
            .drain_while(|token| match token {
                Token::Register(_) => {
                    count.set(count.get() + 1);
                    count.get() <= max
                }
                _ => false,
            })
            .collect();

        if count.get() < min {
            too_few_operands(
                min,
                count.get(),
                self.token(),
                (self.column, self.line, self.token().len()),
            );
        }

        tokens
    }
}
