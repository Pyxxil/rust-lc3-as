use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

token!(St, 2);

impl Assemble for St {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        Vec::new()
    }
}

impl Requirements for St {
    fn memory_requirement(&self) -> u16 {
        1
    }

    fn require_range(&self) -> (u64, u64) {
        (2, 2)
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
