use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::Symbol;
use token::Token;

token!(In);

impl Assemble for In {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        *program_counter += 1;

        vec![(
            0xF023,
            format!(
                "({:04X}) F023 1111000000100011 ({: >4}) {: <20} IN",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}

impl Requirements for In {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    // As IN takes no operands, do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
