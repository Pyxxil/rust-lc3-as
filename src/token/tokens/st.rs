use std::collections::HashMap;
use std::collections::VecDeque;

use token::Symbol;
use token::Token;
use token::tokens::{expected, too_few_operands};
use token::tokens::traits::*;

token!(St, 2);

impl Assemble for St {
    fn assembled(mut self, program_counter: &mut i16, symbols: &HashMap<String, Symbol>, symbol: &String) -> Vec<(u16, String)> {
        *program_counter += 1;

        Vec::new()
    }
}

impl Requirements for St {
    fn require_range(&self) -> (u64, u64) {
        (2, 2)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        if let Some(token) = tokens.front() {
            expect!(
                self,
                tokens,
                token,
                Token::Immediate,
                "Immediate",
                Token::Label,
                "Label"
            );
        }

        operands_check!(self);

        tokens
    }
}
