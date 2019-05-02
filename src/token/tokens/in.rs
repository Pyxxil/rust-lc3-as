use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

token!(In);

impl Assemble for In {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;
        vec![(
            0xF023,
            format!(
                "({:04X}) F023 1111000000100011 ({: >4}) TRAP 0x23",
                *program_counter - 1,
                self.line
            ),
        )]
    }
}

impl Requirements for In {
    fn memory_requirement(&self) -> u16 { 0 } fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    // As IN takes no operands, do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
