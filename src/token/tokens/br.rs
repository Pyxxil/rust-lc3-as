use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Token;

token!(Br, 1, n: bool, z: bool, p: bool);

impl Assemble for Br {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;
        Vec::new()
    }
}

impl Requirements for Br {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(
                self,
                tokens,
                token,
                Token::Label,
                "Label",
                Token::Immediate,
                "Immediate"
            );
        }

        operands_check!(self);

        tokens
    }
}
