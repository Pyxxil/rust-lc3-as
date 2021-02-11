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

token!(Str, 3);

impl Assemble for Str {
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

        let source_one = if let Token::Register(register) = self.operands.remove(0) {
            register.register
        } else {
            unreachable!()
        };

        let source_two = if let Token::Immediate(immediate) = self.operands.remove(0) {
            (immediate.value & 0x3F) as u16
        } else {
            unreachable!()
        };

        let instruction: u16 = 0x7000 | destination_register << 9 | source_one << 6 | source_two;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} STR R{4} R{5} #{6}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                destination_register,
                source_one,
                (source_two << 10) as i16 >> 10,
            ),
        )]
    }
}

impl Requirements for Str {
    fn min_operands(&self) -> u64 {
        3
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        expect!(self, tokens, Register);

        expect!(self, tokens, Immediate);

        operands_check!(self);

        tokens
    }
}
