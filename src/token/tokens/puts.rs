use std::collections::VecDeque;

use token::tokens::traits::*;
use token::Token;

token!(Puts);

impl Assemble for Puts {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        vec![(
            0xF022,
            format!(
                "({:04X}) F022 1111000000100010 ({: >4}) PUTS",
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

    fn memory_requirement(&self) -> u16 {
        1
    }

    // As PUTS takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
