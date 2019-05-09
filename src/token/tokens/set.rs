use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Symbol;
use token::Token;

token!(Set, 2);

impl Assemble for Set {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        _symbol: &str,
    ) -> Vec<(u16, String)> {
        *program_counter += 1;

        Vec::new()
    }
}

impl Requirements for Set {
    fn require_range(&self) -> (u64, u64) {
        (2, 2)
    }

    fn memory_requirement(&self) -> u16 {
        0
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
                Token::Register,
                "Register",
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
