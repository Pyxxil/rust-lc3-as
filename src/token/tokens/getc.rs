use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Getc {
    token: String,
    column: u64,
    line: u64,
}

impl Getc {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Getc {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;
        vec![(
            0xF020,
            format!(
                "({:04X}) F020 1111000000100000 ({: >4}) GETC",
                *program_counter - 1,
                self.line
            ),
        )]
    }
}

impl Requirements for Getc {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    // As GETC takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
