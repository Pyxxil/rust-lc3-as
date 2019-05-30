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
        _symbols: &HashMap<String, Symbol>,
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

        let mut assembled = vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} ADD R{4} R{4} R{4}",
                *program_counter, instruction, self.line, symbol, register,
            ),
        )];

        iter::repeat(instruction)
            .take(count as usize - 1)
            .map(|val| {
                *program_counter += 1;
                (
                    val,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4})                      ADD R{3} R{3} R{3}",
                        *program_counter, val as i16, self.line, register,
                    ),
                )
            })
            .for_each(|line| assembled.push(line));

        *program_counter += 1;

        assembled
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
