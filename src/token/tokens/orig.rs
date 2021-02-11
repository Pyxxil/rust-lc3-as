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

token!(Orig, 1, starting_address: u16);

impl Assemble for Orig {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        let instruction = if let Token::Immediate(immediate) = self.operands.remove(0) {
            immediate.value as u16
        } else {
            unreachable!()
        };

        *program_counter = instruction as i16;

        vec![(
            instruction,
            format!(
                "(0000) {0:4X} {0:016b} ({1: >4}) {2: <20} .ORIG {0:#4X}",
                instruction, self.line, symbol
            ),
        )]
    }
}

impl Requirements for Orig {
    fn min_operands(&self) -> u64 {
        1
    }

    fn memory_requirement(&self) -> u16 {
        if let Token::Immediate(imm) = self.operands.first().unwrap() {
            imm.value as u16
        } else {
            unreachable!()
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Immediate);

        operands_check!(self);

        tokens
    }
}
