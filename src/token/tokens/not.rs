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

token!(Not);

impl Assemble for Not {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let destination_register = if let Token::Register(register) = self.operands.remove(0) {
            register.register
        } else {
            unreachable!()
        };

        let source_register = match self.operands.first() {
            Some(Token::Register(register)) => register.register,
            Some(_) => unreachable!(),
            None => destination_register,
        };

        let instruction = 0x903F | destination_register << 9 | source_register << 6;

        vec![listing!(
            instruction,
            *program_counter - 1,
            self.line,
            symbol,
            "NOT",
            format!("R{}", destination_register),
            format!("R{}", source_register)
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
