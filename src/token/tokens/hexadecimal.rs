use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Hexadecimal {
    token: String,
    column: u64,
    line: u64,
    pub value: i16,
}

impl Hexadecimal {
    pub fn new(token: String, column: u64, line: u64) -> Hexadecimal {
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
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
                column as usize,
                line as usize,
                token.len(),
                format!(
                    "Value ({}) is too large to represent in two's complement 16 bits\n",
                    token
                ),
            )));
            0
        }) as i16;
        Hexadecimal {
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

    fn assembled(self) -> Vec<(u16, String)> {
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
    fn consume(&mut self, mut _tokens: Vec<TokenType>) -> Vec<TokenType> {
        notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
            DiagnosticType::Error,
            self.column as usize,
            self.line as usize,
            self.token.len(),
            format!(
                "Expected Instruction, Directive, or Label, but found\n {:#?}\n",
                self
            ),
        )));
        _tokens
    }
}
