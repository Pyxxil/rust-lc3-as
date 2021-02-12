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

token!(Trap);

impl Assemble for Trap {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let instruction = if let Token::Immediate(immediate) = self.operands.first().unwrap() {
            0xF000 | (immediate.value as u16 & 0xFF)
        } else {
            unreachable!()
        };

        vec![listing!(
            instruction,
            *program_counter - 1,
            self.line,
            symbol,
            "TRAP",
            format!("0x{:02X}", instruction & 0xFF)
        )]
    }
}

impl Requirements for Trap {
    fn min_operands(&self) -> u64 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Immediate);

        operands_check!(self);

        tokens
    }
}
