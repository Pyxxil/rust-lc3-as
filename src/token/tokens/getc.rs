use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

token!(Getc);

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
    fn memory_requirement(&self) -> u16 { 0 } fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    // As GETC takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
