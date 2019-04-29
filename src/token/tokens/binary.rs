use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
    token: String,
    column: u64,
    line: u64,
    pub value: i16,
}

impl Binary {
    pub fn new(token: String, column: u64, line: u64) -> Binary {
        let value = i16::from_str_radix(
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
        });

        let value = if token.find('-').is_some() { -value } else { value };

        Binary {
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

impl Assemble for Binary {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Binary {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, tokens: Vec<TokenType>) -> Vec<TokenType> {
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
        tokens
    }
}
