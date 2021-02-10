use std::collections::{HashMap, VecDeque};

use crate::{
    token::{
        tokens::traits::{Assemble, Requirements},
        Symbol, Token,
    },
    types::Listings,
};

token!(Ret);

impl Assemble for Ret {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![(
            0xC1C0,
            format!(
                "({:04X}) C1C0 1100000111000000 ({: >4}) {: <20} RET",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}

impl Requirements for Ret {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    // As RET takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
