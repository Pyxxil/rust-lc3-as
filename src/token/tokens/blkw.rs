use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

use std::iter;

#[derive(Debug, PartialEq, Clone)]
pub struct Blkw {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Blkw {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
            operands: Vec::with_capacity(2),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Blkw {
    fn assemble(&mut self) {}

    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let value = match self.operands.last().unwrap() {
            Token::Binary(binary) => binary.value,
            Token::Decimal(decimal) => decimal.value,
            Token::Hexadecimal(hexadecimal) => hexadecimal.value,
            _ => 0,
        } as u16;
        iter::repeat(value)
            .take(match self.operands.first().unwrap() {
                Token::Binary(binary) => binary.value,
                Token::Decimal(decimal) => decimal.value,
                Token::Hexadecimal(hexadecimal) => hexadecimal.value,
                _ => 0,
            } as usize)
            .map(|val| {
                *program_counter += 1;
                (
                    val,
                    format!(
                        "{0:4X} {1:04X} {1:016b} ({2}) .FILL #{1}",
                        program_counter, value as i16, self.line,
                    ),
                )
            })
            .collect()
    }
}

impl Requirements for Blkw {
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            match token {
                Token::Binary(_)
                | Token::Character(_)
                | Token::Decimal(_)
                | Token::Hexadecimal(_) => {
                    self.operands.push(tokens.pop_front().unwrap());
                    if let Some(second) = tokens.front() {
                        match second {
                            Token::Binary(_)
                            | Token::Character(_)
                            | Token::Decimal(_)
                            | Token::Hexadecimal(_)
                            | Token::Label(_) => self.operands.push(tokens.pop_front().unwrap()),
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
