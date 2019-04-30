use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Immediate {
    token: String,
    column: u64,
    line: u64,
    pub value: i16,
}

impl Immediate {
    pub fn from_decimal(token: String, column: u64, line: u64) -> Self {
        let value = token
            .chars()
            .skip(token.chars().position(|c| c.is_digit(10)).unwrap())
            .collect::<String>()
            .parse::<i16>()
            .unwrap_or_else(|_| {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    column,
                    line,
                    token.len(),
                    format!(
                        "Value {} is too large to be represented in signed 16 bits\n",
                        token
                    ),
                )));
                0
            });

        let value = if token.find('-').is_some() {
            -value
        } else {
            value
        };

        Self {
            token,
            column,
            line,
            value,
        }
    }

    pub fn from_hexadecimal(token: String, column: u64, line: u64) -> Self {
        let value = u16::from_str_radix(
            token
                .chars()
                .skip(
                    1 + token
                        .chars()
                        .position(|c| c.to_ascii_uppercase() == 'X')
                        .unwrap(),
                )
                .collect::<String>()
                .as_ref(),
            16,
        )
        .unwrap_or_else(|_| {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                column,
                line,
                token.len(),
                format!(
                    "Value {} is too large to be represented in signed 16 bits\n",
                    token
                ),
            )));
            0
        }) as i16;
        Self {
            token,
            column,
            line,
            value,
        }
    }

    pub fn from_binary(token: String, column: u64, line: u64) -> Self {
        let value = u16::from_str_radix(
            token
                .chars()
                .skip(
                    1 + token
                        .chars()
                        .position(|c| c.to_ascii_uppercase() == 'B')
                        .unwrap(),
                )
                .collect::<String>()
                .as_ref(),
            2,
        )
        .unwrap_or_else(|_| {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                column,
                line,
                token.len(),
                format!(
                    "Value {} is too large to be represented in signed 16 bits\n",
                    token
                ),
            )));
            0
        }) as i16;

        let value = if token.find('-').is_some() {
            -value
        } else {
            value
        };

        Self {
            token,
            column,
            line,
            value,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Immediate {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Immediate {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
            DiagType::Error,
            self.column,
            self.line,
            self.token.len(),
            format!(
                "Expected Instruction, Directive, or Label, but found\n{:#?}\n",
                self
            ),
        )));
        tokens
    }
}
