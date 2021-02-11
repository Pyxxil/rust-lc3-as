use std::collections::VecDeque;

use crate::{
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Token,
    },
    types::{Listings, SymbolTable},
};

token!(And, 3);

impl Assemble for And {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
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
                Token::Register(register) => register.register,
                Token::Immediate(imm) => (0x20 | (imm.value & 0x1F)) as u16,
                _ => unreachable!(),
            }
        } else {
            source_one
        };

        let instruction: u16 = 0x5000 | destination_register << 9 | source_one << 6 | source_two;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} AND R{4} R{5} {6}{7}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                destination_register,
                source_one,
                if (instruction & 0x20) == 0 { 'R' } else { '#' },
                ((source_two & 0x1F) << 11) as i16 >> 11
            ),
        )]
    }
}

impl Requirements for And {
    fn min_operands(&self) -> u64 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        maybe_expect!(self, tokens, Register);

        // We want to allow AND R1, R2[, #2] but not AND R2, #2
        if self.operands.len() == 2 {
            // This will mean the above maybe_expect! succeeded, and so we can accept an immediate value here
            maybe_expect!(self, tokens, Immediate, Register);
        } else {
            maybe_expect!(self, tokens, Register);
        }

        operands_check!(self);

        tokens
    }
}
