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

token!(Add, 3);

impl Assemble for Add {
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

        let instruction: u16 = 0x1000 | destination_register << 9 | source_one << 6 | source_two;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} ADD R{4} R{5} {6}{7}",
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

impl Requirements for Add {
    fn min_operands(&self) -> u64 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        maybe_expect!(self, tokens, Register);

        if self.operands.len() == 2 {
            maybe_expect!(self, tokens, Immediate, Register);
        } else {
            maybe_expect!(self, tokens, Register);
        }

        operands_check!(self);

        tokens
    }
}
