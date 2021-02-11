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

token!(Neg, 2);

impl Assemble for Neg {
    #[allow(clippy::cast_sign_loss)]
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += self.memory_requirement() as i16;

        let destination_register = match self.operands.first().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let source_register = match self.operands.last().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let not_instruction = 0x903F | destination_register << 9 | source_register << 6;
        let add_instruction = 0x1021 | destination_register << 9 | source_register << 6;

        vec![
            (
                not_instruction,
                format!(
                    "({0:04X}) {1:04X} {1:0>16b} ({2: >4}) {3: <20} NOT R{4} R{5}",
                    *program_counter - 2,
                    not_instruction,
                    self.line,
                    symbol,
                    destination_register,
                    source_register,
                ),
            ),
            (
                add_instruction,
                format!(
                    "({0:04X}) {1:04X} {1:0>16b} ({2: >4})                      ADD R{3} R{4} #1",
                    *program_counter - 1,
                    add_instruction,
                    self.line,
                    destination_register,
                    source_register,
                ),
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
