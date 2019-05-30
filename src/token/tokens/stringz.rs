use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::expected;
use token::tokens::traits::*;
use token::Symbol;
use token::Token;

token!(Stringz, 1);

impl Assemble for Stringz {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        let mut assembled = Vec::new();

        match self.operands.first().unwrap() {
            Token::String(string) => {
                if let Some(character) = string.token().chars().next() {
                    assembled.push((
                        character as u16,
                        format!(
                            "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} .FILL #{1}",
                            *program_counter, character as u16, self.line, symbol
                        ),
                    ));
                }
                string.token().chars().skip(1).for_each(|c| {
                    *program_counter += 1;
                    assembled.push((
                        c as u16,
                        format!(
                            "({0:04X}) {1:04X} {1:016b} ({2: >4})                      .FILL #{1}",
                            *program_counter, c as u16, self.line
                        ),
                    ));
                });
                *program_counter += 1;
                assembled.push((
                    0,
                    format!(
                        "({0:04X}) 0000 0000000000000000 ({1: >4})                      .FILL #0",
                        *program_counter, self.line
                    ),
                ))
            }
            _ => unreachable!(),
        }

        self.operands.iter().skip(1).for_each(|token| match token {
            Token::String(string) => {
                string.token().chars().for_each(|c| {
                    *program_counter += 1;
                    assembled.push((
                        c as u16,
                        format!(
                            "({0:04X}) {1:04X} {1:016b} ({2: >4})                      .FILL #{1}",
                            *program_counter, c as u16, self.line
                        ),
                    ))
                });
                assembled.push((
                    0,
                    format!(
                        "({0:04X}) 0000 0000000000000000 ({1: >4})                      .FILL #0",
                        *program_counter, self.line
                    ),
                ))
            }
            _ => unreachable!(),
        });

        *program_counter += 1;

        assembled
    }
}

impl Requirements for Stringz {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn memory_requirement(&self) -> u16 {
        self.operands.iter().fold(0_u16, |acc, token| match token {
            Token::String(string) => acc + string.token().len() as u16 + 1, // Don't forget the '\0'
            _ => unreachable!(),
        })
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::String, "String");
        }

        let position = tokens.iter().position(|token| {
            if let Token::String(_) = token {
                false
            } else {
                true
            }
        });

        if let Some(position) = position {
            let mut extra_strings = tokens.drain(..position).collect::<Vec<_>>();

            self.operands.append(&mut extra_strings);
        }

        tokens
    }
}
