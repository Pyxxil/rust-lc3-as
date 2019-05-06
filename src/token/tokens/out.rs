use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

token!(Out);

impl Assemble for Out {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        vec![(
            0xF022,
            format!(
                "({:04X}) F021 1111000000100001 ({: >4}) OUT",
                *program_counter - 1,
                self.line
            ),
        )]
    }
}

impl Requirements for Out {
    fn memory_requirement(&self) -> u16 {
        1
    }

    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    // As OUT takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
