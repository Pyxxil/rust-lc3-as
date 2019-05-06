use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

token!(Putsp);

impl Assemble for Putsp {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        vec![(
            0xF024,
            format!(
                "({:04X}) F024 1111000000100100 ({: >4}) PUTSP",
                *program_counter - 1,
                self.line
            ),
        )]
    }
}

impl Requirements for Putsp {
    fn memory_requirement(&self) -> u16 {
        1
    }

    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    // As PUTSP takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
