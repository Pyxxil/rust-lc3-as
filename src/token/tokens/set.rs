use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Symbol;
use token::Token;

token!(Set, 2);

impl Assemble for Set {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        let immediate = match self.operands.last().unwrap() {
            Token::Immediate(immediate) => immediate.value,
            _ => unreachable!(),
        };

        let register = match self.operands.first().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        if immediate >= -16 && immediate <= 15 {
            *program_counter += 2;
            let and_instruction = 0x5020 | register << 9 | register << 6;
            let add_instruction =
                0x1020 | register << 9 | register << 6 | (immediate as u16 & 0x1F);
            vec![
                (
                    and_instruction,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} AND R{4} R{4} #0",
                        *program_counter - 2,
                        and_instruction,
                        self.line,
                        symbol,
                        register
                    ),
                ),
                (
                    add_instruction,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4})                      ADD R{3} R{3} #{4}",
                        *program_counter - 1,
                        add_instruction,
                        self.line,
                        register,
                        immediate
                    ),
                ),
            ]
        } else {
            *program_counter += 3;
            vec![
                (
                    0x0E01,
                    format!(
                        "({0:04X}) 0E01 0000111000000001 ({1: >4}) {2: <20} BRnzp #1",
                        *program_counter - 3,
                        self.line,
                        symbol
                    ),
                ),
                (
                    immediate as u16,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4})                      .FILL #{1}",
                        *program_counter - 2,
                        immediate as i16,
                        self.line
                    ),
                ),
                (
                    0x21FE,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4})                      LD R{3} #-2",
                        *program_counter - 1,
                        0x21FE | register << 9,
                        self.line,
                        register,
                    ),
                ),
            ]
        }
    }
}

impl Requirements for Set {
    fn require_range(&self) -> (u64, u64) {
        (2, 2)
    }

    fn memory_requirement(&self) -> u16 {
        match self.operands.last().unwrap() {
            Token::Immediate(immediate) => {
                if immediate.value > 15 || immediate.value < -16 {
                    3
                } else {
                    2
                }
            }
            _ => unreachable!(),
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        if let Some(token) = tokens.front() {
            expect!(
                self,
                tokens,
                token,
                Token::Register,
                "Register",
                Token::Immediate,
                "Immediate",
                Token::Label,
                "Label"
            );
        }

        operands_check!(self);

        tokens
    }
}
