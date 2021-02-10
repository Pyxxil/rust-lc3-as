use std::collections::HashMap;
use std::collections::VecDeque;

use crate::{
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Symbol, Token,
    },
    types::Listings,
};

token!(Trap, 1);

impl Assemble for Trap {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let instruction = 0xF000
            | (match self.operands.first().unwrap() {
                Token::Immediate(imm) => imm.value,
                _ => unreachable!(),
            } & 0xFF) as u16;

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
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Token::Immediate, "Immediate");

        operands_check!(self);

        tokens
    }
}
