use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Token;

token!(And, 3);

impl Assemble for And {
    fn assembled(mut self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        let destination_register = match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let source_one = if let Some(token) = self.operands.first() {
            if let Token::Register(register) = token {
                register.register
            } else {
                unreachable!()
            }
        } else {
            destination_register
        };

        let source_two = if let Some(token) = self.operands.last() {
            match token {
                Token::Register(register) => register.register as i16,
                Token::Immediate(imm) => 0x20 | (imm.value & 0x1F),
                _ => unreachable!(),
            }
        } else {
            source_one as i16
        } as u16;

        let instruction: u16 = 0x5000 | destination_register << 9 | source_one << 6 | source_two;

        vec![(
            instruction,
            format!(
                "({0:4X}) {1:04X} {1:016b} ({2: >4}) AND R{3} R{4} {5}{6}",
                *program_counter - 1,
                instruction,
                self.line,
                destination_register,
                source_one,
                if (instruction & 0x20) == 0 { 'R' } else { '#' },
                ((source_two & 0x1F) << 11) as i16 >> 11
            ),
        )]
    }
}

impl Requirements for And {
    fn require_range(&self) -> (u64, u64) {
        (2, 3)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        if let Some(token) = tokens.front() {
            maybe_expect!(self, tokens, token, Token::Register);
        }

        if let Some(token) = tokens.front() {
            // We want to allow ADD R1, R2[, #2] but not ADD R2, #2
            if self.operands.len() == 2 {
                // This will mean the above maybe_expect! succeeded, and so we can accept an immediate value here
                maybe_expect!(self, tokens, token, Token::Immediate, Token::Register);
            } else {
                maybe_expect!(self, tokens, token, Token::Register);
            }
        }

        operands_check!(self);

        tokens
    }
}
