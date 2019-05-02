use token::tokens::traits::*;

use token::tokens::{expected, too_few_operands};

use token::Token;

use std::collections::VecDeque;

token!(Not, 2);

impl Assemble for Not {
    fn assembled(mut self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let destination_register = match self.operands.remove(0) {
            Token::Register(register) => u16::from(register.register),
            _ => unreachable!(),
        };

        let source_register = match self.operands.first() {
            Some(token) => match token {
                Token::Register(register) => u16::from(register.register),
                _ => unreachable!(),
            },
            None => destination_register,
        };

        let instruction = 0x901F | destination_register << 9 | source_register << 6;

        vec![(
            instruction,
            format!(
                "({0:4X}) {1:04X} {1:016b} ({2: >4}) NOT R{3} R{4}",
                *program_counter - 1,
                instruction,
                self.line,
                destination_register,
                source_register,
            ),
        )]
    }
}

impl Requirements for Not {
    fn memory_requirement(&self) -> u16 { 0 } fn require_range(&self) -> (u64, u64) {
        (1, 2)
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
