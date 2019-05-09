use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::Symbol;
use token::Token;

token!(Halt);

impl Assemble for Halt {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        *program_counter += 1;

        vec![(
            0xF025,
            format!(
                "({:04X}) F025 1111000000100101 ({: >4}) {: <20} HALT",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}

impl Requirements for Halt {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    // As HALT takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
