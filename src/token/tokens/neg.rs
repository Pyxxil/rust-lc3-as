use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Symbol;
use token::Token;

token!(Neg, 2);

impl Assemble for Neg {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        *program_counter += self.memory_requirement() as i16;

        Vec::new()
    }
}

impl Requirements for Neg {
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn memory_requirement(&self) -> u16 {
        self.operands.len() as u16
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
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
