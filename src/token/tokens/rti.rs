use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

token!(Rti);

impl Assemble for Rti {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        vec![(
            0x8000,
            format!(
                "({0:4X}) {1:04X} {1:016b} ({2: >4}) RTI",
                *program_counter - 1,
                0x8000,
                self.line,
            ),
        )]
    }
}

impl Requirements for Rti {
    fn memory_requirement(&self) -> u16 {
        1
    }

    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    // As RTI takes no operands, just do nothing here.
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
