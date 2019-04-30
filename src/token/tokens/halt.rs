use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Halt {
    token: String,
    column: u64,
    line: u64,
}

impl Halt {
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

impl Assemble for Halt {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;
        vec![(
            0xF025,
            format!(
                "{:04X} F025 1111000000100101 ({}) TRAP 0x25",
                *program_counter - 1,
                self.line
            ),
        )]
    }
}

impl Requirements for Halt {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    // As HALT takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
