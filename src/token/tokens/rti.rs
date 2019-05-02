use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

token!(Rti);

impl Assemble for Rti {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Rti {
    fn memory_requirement(&self) -> u16 { 0 } fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    // As RTI takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
