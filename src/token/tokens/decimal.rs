use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Decimal {
    token: String,
    column: u64,
    line: u64,
}

impl Decimal {
    pub fn new(token: String, column: u64, line: u64) -> Decimal {
        Decimal {
            token,
            column,
            line,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Decimal {
    fn assemble(&mut self) {}
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
