use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

use std::iter;

token!(Blkw, 2);

impl Assemble for Blkw {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let value = match self.operands.last().unwrap() {
            Token::Immediate(imm) => imm.value,
            Token::Label(_) => 0,
            _ => unreachable!(),
        } as u16;
        iter::repeat(value)
            .take(match self.operands.first().unwrap() {
                Token::Immediate(imm) => imm.value,
                _ => unreachable!(),
            } as usize)
            .map(|val| {
                *program_counter += 1;
                (
                    val,
                    format!(
                        "({0:4X}) {1:04X} {1:016b} ({2: >4}) .FILL #{1}",
                        *program_counter - 1,
                        val as i16,
                        self.line,
                    ),
                )
            })
            .collect()
    }
}

impl Requirements for Blkw {
    fn memory_requirement(&self) -> u16 {
        match self.operands.first().unwrap() {
            Token::Immediate(imm) => imm.value as u16,
            _ => unreachable!(),
        }
    }

    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            match token {
                Token::Immediate(_) | Token::Character(_) => {
                    self.operands.push(tokens.pop_front().unwrap());
                    if let Some(second) = tokens.front() {
                        match second {
                            Token::Immediate(_) | Token::Label(_) => {
                                self.operands.push(tokens.pop_front().unwrap())
                            }
                            _ => {}
                        };
                    }
                }
                ref token => {
                    notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                        DiagType::Error,
                        self.column,
                        self.line,
                        self.token.len(),
                        format!("Expected an Immediate Literal, but found\n {:#?}", token),
                    )));
                }
            }
        } else {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                "Expected an argument, but found nothing.".to_owned(),
            )));
        }

        tokens
    }
}
