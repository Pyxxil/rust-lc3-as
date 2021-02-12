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

token!(Neg);

impl Assemble for Neg {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += self.memory_requirement() as i16;

        let destination_register = if let Token::Register(register) = self.operands.first().unwrap()
        {
            register.register
        } else {
            unreachable!()
        };

        let source_register = if let Token::Register(register) = self.operands.last().unwrap() {
            register.register
        } else {
            unreachable!()
        };

        let not_instruction = 0x903F | destination_register << 9 | source_register << 6;
        let add_instruction = 0x1021 | destination_register << 9 | source_register << 6;
        let dest = format!("R{}", destination_register);
        let source = format!("R{}", source_register);

        vec![
            listing!(
                not_instruction,
                *program_counter - 2,
                self.line,
                symbol,
                "NOT",
                dest,
                source
            ),
            listing!(
                add_instruction,
                *program_counter - 1,
                self.line,
                "",
                "ADD",
                dest,
                source,
                "#1"
            ),
        ]
    }
}

impl Requirements for Neg {
    fn min_operands(&self) -> u64 {
        1
    }

    fn memory_requirement(&self) -> u16 {
        2
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        maybe_expect!(self, tokens, Register);

        operands_check!(self);

        tokens
    }
}
