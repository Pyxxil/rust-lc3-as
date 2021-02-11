use std::collections::VecDeque;

use crate::{
    listing,
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Token,
    },
    types::{Listings, SymbolTable},
};

token!(Jmpt, 1);

impl Assemble for Jmpt {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let register = if let Token::Register(register) = self.operands.first().unwrap() {
            register.register
        } else {
            unreachable!()
        };

        let instruction = 0xC001 | register << 6;

        vec![listing!(
            instruction,
            *program_counter - 1,
            instruction,
            self.line,
            symbol,
            "JMPT",
            format!("R{}", register)
        )]
    }
}

impl Requirements for Jmpt {
    fn min_operands(&self) -> u64 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        operands_check!(self);

        tokens
    }
}
