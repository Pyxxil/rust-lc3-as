use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

token!(Halt);

impl Assemble for Halt {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;
        vec![(
            0xF025,
            format!(
                "({:04X}) F025 1111000000100101 ({: >4}) HALT",
                *program_counter - 1,
                self.line
            ),
        )]
    }
}

impl Requirements for Halt {
    fn memory_requirement(&self) -> u16 { 0 } fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    // As HALT takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
