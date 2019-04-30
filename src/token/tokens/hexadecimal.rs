use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

#[derive(Debug, PartialEq, Clone)]
pub struct Hexadecimal {
    token: String,
    column: u64,
    line: u64,
    pub value: i16,
}

impl Hexadecimal {
    pub fn new(token: String, column: u64, line: u64) -> Self {
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

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Hexadecimal {
    fn assemble(&mut self) {}

    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Hexadecimal {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    // As a Hexadecimal Immediate shouldn't be consumed from, throw an error at the user.
    fn consume(&mut self, tokens: Vec<Token>) -> Vec<Token> {
        notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
            DiagType::Error,
            self.column,
            self.line,
            self.token.len(),
            format!(
                "Expected Instruction, Directive, or Label, but found\n {:#?}\n",
                self
            ),
        )));
        tokens
    }
}
