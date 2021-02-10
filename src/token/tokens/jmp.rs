use std::collections::{HashMap, VecDeque};

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

token!(Jmp, 1);

impl Assemble for Jmp {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let register = match self.operands.first().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let instruction = 0xC000 | register << 6;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} JMP R{4}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                register,
            ),
        )]
    }
}

impl Requirements for Jmp {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }
    fn memory_requirement(&self) -> u16 {
        0
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Token::Register, "Register");

        operands_check!(self);

        tokens
    }
}
