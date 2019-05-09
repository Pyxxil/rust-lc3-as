use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::Symbol;
use token::Token;

token!(Getc);

impl Assemble for Getc {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        *program_counter += 1;

        vec![(
            0xF020,
            format!(
                "({:04X}) F020 1111000000100000 ({: >4}) {: <20} GETC",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}

impl Requirements for Getc {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    // As GETC takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
