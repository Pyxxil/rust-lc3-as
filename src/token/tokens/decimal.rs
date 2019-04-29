use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Decimal {
    token: String,
    column: u64,
    line: u64,
    pub value: i16,
}

impl Decimal {
    pub fn new(token: String, column: u64, line: u64) -> Decimal {
        let value = token
            .chars()
            .skip(token.chars().position(|c| c.is_digit(10)).unwrap())
            .collect::<String>()
            .parse::<i16>()
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
            }
            );

        let value = if token.find('-').is_some() {
            -value
        } else {
            value
        };

        Decimal {
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

impl Assemble for Decimal {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Decimal {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut _tokens: Vec<TokenType>) -> Vec<TokenType> {
        notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
            DiagnosticType::Error,
            self.column as usize,
            self.line as usize,
            self.token.len(),
            format!(
                "Expected Instruction, Directive, or Label, but found\n{:#?}\n",
                self
            ),
        )));
        _tokens
    }
}
