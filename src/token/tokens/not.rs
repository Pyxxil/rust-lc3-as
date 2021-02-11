use std::collections::VecDeque;

use crate::{
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Token,
    },
    types::{Listings, SymbolTable},
};

token!(Not, 2);

impl Assemble for Not {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let destination_register = match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let source_register = match self.operands.first() {
            Some(token) => match token {
                Token::Register(register) => register.register,
                _ => unreachable!(),
            },
            None => destination_register,
        };

        let instruction = 0x903F | destination_register << 9 | source_register << 6;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} NOT R{4} R{5}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                destination_register,
                source_register,
            ),
        )]
    }
}

impl Requirements for Not {
    fn min_operands(&self) -> u64 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        maybe_expect!(self, tokens, Register);

        operands_check!(self);

        tokens
    }
}
