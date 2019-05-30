use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::Symbol;
use token::Token;

token!(Rti);

impl Assemble for Rti {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        *program_counter += 1;

        vec![(
            0x8000,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} RTI",
                *program_counter - 1,
                0x8000,
                self.line,
                symbol
            ),
        )]
    }
}

impl Requirements for Rti {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    // As RTI takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
