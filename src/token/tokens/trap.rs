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

token!(Trap, 1);

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

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} TRAP 0x{4:02X}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                instruction & 0xFF,
            ),
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
