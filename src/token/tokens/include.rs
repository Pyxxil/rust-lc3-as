use token::tokens::traits::*;

use token::tokens::{expected, too_few_operands};

use token::Token;

use std::collections::VecDeque;

token!(Include, 1);

impl Assemble for Include {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Include {
    fn memory_requirement(&self) -> u16 {
        0
    }
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::String, "String");
        }

        operands_check!(self);

        tokens
    }
}
