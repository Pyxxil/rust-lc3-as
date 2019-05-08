use std::collections::VecDeque;

use token::r#type::Token;
use token::tokens::traits::*;
use token::tokens::*;

token!(Sub, 3);

impl Assemble for Sub {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        Vec::new()
    }
}

impl Requirements for Sub {
    fn require_range(&self) -> (u64, u64) {
        (2, 3)
    }

    fn memory_requirement(&self) -> u16 {
        0
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        if let Some(token) = tokens.front() {
            maybe_expect!(self, tokens, token, Token::Register);
        }

        operands_check!(self);

        tokens
    }
}
