use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Symbol;
use token::Token;

token!(Not, 2);

impl Assemble for Not {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
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
                "({0:4X}) {1:04X} {1:016b} ({2: >4}) {3: <20} NOT R{4} R{5}",
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
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn memory_requirement(&self) -> u16 {
        1
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
