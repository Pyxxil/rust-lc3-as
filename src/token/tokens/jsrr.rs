use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::{Assemble, Requirements};
use token::tokens::{expected, too_few_operands};
use token::{Symbol, Token};

token!(Jsrr, 1);

impl Assemble for Jsrr {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        *program_counter += 1;

        let register = match self.operands.first().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let instruction = 0x4000 | register << 6;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} JSRR R{4}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                register,
            ),
        )]
    }
}

impl Requirements for Jsrr {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        operands_check!(self);

        tokens
    }
}
