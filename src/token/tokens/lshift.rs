use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Symbol;
use token::Token;

token!(Lshift, 2);

impl Assemble for Lshift {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        let register = match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let count = match self.operands.remove(0) {
            Token::Immediate(immediate) => immediate.value as u16,
            _ => unreachable!(),
        };

        let instruction = 0x1000 | register << 9 | register << 6 | register;

        iter::repeat(instruction)
            .take(count as usize)
            .map(|val| {
                *program_counter += 1;
                (
                    val,
                    format!(
                        "({0:4X}) {1:04X} {1:016b} ({2: >4}) ADD R{3} R{3} R{3}",
                        *program_counter - 1,
                        val as i16,
                        self.line,
                        register,
                    ),
                )
            })
            .collect()
    }
}

impl Requirements for Lshift {
    fn require_range(&self) -> (u64, u64) {
        (2, 2)
    }

    fn memory_requirement(&self) -> u16 {
        match self.operands.last().unwrap() {
            Token::Immediate(imm) => imm.value as u16,
            _ => unreachable!(),
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Immediate, "Immediate");
        }

        operands_check!(self);

        tokens
    }
}
