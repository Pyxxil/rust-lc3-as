use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Ldr {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Ldr {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
            operands: Vec::with_capacity(3),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Ldr {
    fn assembled(mut self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let destination_register = u16::from(match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        });
        let source_one = u16::from(match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        });
        let source_two = match self.operands.remove(0) {
            Token::Immediate(imm) => imm.value & 0x3F,
            _ => unreachable!(),
        } as u16;

        let instruction: u16 = 0x6000 | destination_register << 9 | source_one << 6 | source_two;

        *program_counter += 1;

        vec![(
            instruction,
            format!(
                "{0:4X} {1:04X} {1:016b} ({2}) ADD R{3} R{4} #{5}",
                *program_counter - 1,
                instruction,
                self.line,
                destination_register,
                source_one,
                (source_two << 10) as i16 >> 10,
            ),
        )]
    }
}

impl Requirements for Ldr {
    fn require_range(&self) -> (u64, u64) {
        (3, 3)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            match token {
                Token::Register(_) => self.operands.push(tokens.pop_front().unwrap()),
                tok => {
                    expected(
                        &["Register"],
                        &tok,
                        (self.column, self.line, self.token().len()),
                    );
                    return tokens;
                }
            }
        }

        if let Some(token) = tokens.front() {
            match token {
                Token::Register(_) => self.operands.push(tokens.pop_front().unwrap()),
                tok => {
                    expected(
                        &["Register"],
                        &tok,
                        (self.column, self.line, self.token().len()),
                    );
                    return tokens;
                }
            }
        }

        if let Some(token) = tokens.front() {
            match token {
                Token::Immediate(_) => self.operands.push(tokens.pop_front().unwrap()),
                tok => {
                    expected(
                        &["Immediate"],
                        &tok,
                        (self.column, self.line, self.token().len()),
                    );
                    return tokens;
                }
            }
        }

        if self.operands.len() < 3 {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                format!(
                    "LDR expects three operands, but only {} were found",
                    self.operands.len()
                ),
            )));
        }

        tokens
    }
}
