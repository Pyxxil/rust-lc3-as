use std::collections::VecDeque;

use crate::{
    listing,
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Token,
    },
    types::{Listings, SymbolTable},
};

token!(And);

impl Assemble for And {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let destination_register = if let Token::Register(register) = self.operands.remove(0) {
            register.register
        } else {
            unreachable!()
        };

        let source_one = match self.operands.first() {
            Some(Token::Register(register)) => register.register,
            Some(_) => unreachable!(),
            None => destination_register,
        };

        let source_two = match self.operands.last() {
            Some(Token::Register(register)) => register.register,
            Some(Token::Immediate(imm)) => (0x20 | (imm.value & 0x1F)) as u16,
            Some(_) => unreachable!(),
            None => source_one,
        };

        let instruction: u16 = 0x5000 | destination_register << 9 | source_one << 6 | source_two;

        vec![listing!(
            instruction,
            *program_counter - 1,
            self.line,
            symbol,
            "AND",
            format!("R{}", destination_register),
            format!("R{}", source_one),
            format!(
                "{}{}",
                if (instruction & 0x20) == 0 { 'R' } else { '#' },
                ((source_two & 0x1F) << 11) as i16 >> 11
            )
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
