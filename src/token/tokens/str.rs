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

token!(Str);

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

        vec![listing!(
            instruction,
            *program_counter - 1,
            self.line,
            symbol,
            "STR",
            format!("R{}", destination_register),
            format!("R{}", source_one),
            format!("#{}", (source_two << 10) as i16 >> 10)
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
