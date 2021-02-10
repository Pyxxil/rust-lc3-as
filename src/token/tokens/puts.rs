use std::collections::{HashMap, VecDeque};

use crate::{
    token::{
        tokens::traits::{Assemble, Requirements},
        Symbol, Token,
    },
    types::Listings,
};

token!(Puts);

impl Assemble for Puts {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![(
            0xF022,
            format!(
                "({:04X}) F022 1111000000100010 ({: >4}) {: <20} PUTS",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}

impl Requirements for Puts {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    // As PUTS takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
