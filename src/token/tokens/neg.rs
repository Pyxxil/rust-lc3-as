use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Symbol;
use token::Token;

token!(Neg, 2);

impl Assemble for Neg {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
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
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn memory_requirement(&self) -> u16 {
        2
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        if let Some(token) = tokens.front() {
            maybe_expect!(self, tokens, token, Token::Register);
        }

        operands_check!(self);

        tokens
    }
}
