use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Puts {
    token: String,
    column: u64,
    line: u64,
}

impl Puts {
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

impl Assemble for Puts {
    fn assemble(&mut self) {}

    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;
        vec![(
            0xF022,
            format!(
                "{:04X} F022 1111000000100010 ({}) TRAP 0x22",
                *program_counter - 1,
                self.line
            ),
        )]
    }
}

impl Requirements for Puts {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    // As PUTS takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
