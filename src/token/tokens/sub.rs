use std::collections::HashMap;
use std::collections::VecDeque;

use token::r#type::Token;
use token::tokens::traits::*;
use token::tokens::*;
use token::Symbol;

token!(Sub, 3);

impl Assemble for Sub {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        let destination_register = match self.operands.first().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let source_register_one = if self.operands.len() > 2 {
            match &self.operands[1] {
                Token::Register(ref register) => register.register,
                _ => unreachable!(),
            }
        } else {
            destination_register
        };

        let source_register_two = match self.operands.last().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        if source_register_one == source_register_two {
            *program_counter += 1;
            let instruction = 0x5000 | destination_register << 9 | source_register_one << 6 | 0x20;
            vec![(
                instruction,
                format!(
                    "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} AND R{4} R{5} #0",
                    *program_counter - 1,
                    instruction,
                    self.line,
                    symbol,
                    destination_register,
                    source_register_one,
                ),
            )]
        } else {
            *program_counter += 3;
            let not_instruction = 0x903F | source_register_two << 9 | source_register_two << 6;
            let add_instruction = 0x1021 | source_register_two << 9 | source_register_two << 6;

            let subtract_instruction =
                0x1000 | destination_register << 9 | source_register_one << 6 | source_register_two;

            let mut assembled = vec![
                (
                    not_instruction,
                    format!(
                        "({0:04X}) {1:04X} {1:0>16b} ({2: >4}) {3: <20} NOT R{4} R{4}",
                        *program_counter - 3,
                        not_instruction,
                        self.line,
                        symbol,
                        source_register_two
                    ),
                ),
                (
                    add_instruction,
                    format!(
                        "({0:04X}) {1:04X} {1:0>16b} ({2: >4})                      ADD R{3} R{3} #1",
                        *program_counter - 2,
                        add_instruction,
                        self.line,
                        source_register_two,
                    ),
                ),
                (
                    subtract_instruction,
                    format!(
                        "({0:04X}) {1:04X} {1:0>16b} ({2: >4})                      ADD R{3} R{4} R{5}",
                        *program_counter - 1,
                        subtract_instruction,
                        self.line,
                        destination_register,
                        source_register_one,
                        source_register_two,
                    ),
                ),
            ];

            if destination_register != source_register_two {
                *program_counter += 2;
                assembled.push((
                    not_instruction,
                    format!(
                        "({0:04X}) {1:04X} {1:0>16b} ({2: >4})                      NOT R{3} R{3}",
                        *program_counter - 2,
                        not_instruction,
                        self.line,
                        source_register_two
                    ),
                ));
                assembled.push(
                (
                    add_instruction,
                    format!(
                        "({0:04X}) {1:04X} {1:0>16b} ({2: >4})                      ADD R{3} R{3} #1",
                        *program_counter - 1,
                        add_instruction,
                        self.line,
                        source_register_two,
                    ),
                ));
            }

            assembled
        }
    }
}

impl Requirements for Sub {
    fn require_range(&self) -> (u64, u64) {
        (2, 3)
    }

    fn memory_requirement(&self) -> u16 {
        let destination_register = match self.operands.first().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let source_register_one = if self.operands.len() > 2 {
            match &self.operands[1] {
                Token::Register(ref register) => register.register,
                _ => unreachable!(),
            }
        } else {
            destination_register
        };

        let source_register_two = match self.operands.last().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        if source_register_one == source_register_two {
            1
        } else if destination_register == source_register_two {
            3
        } else {
            5
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

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
