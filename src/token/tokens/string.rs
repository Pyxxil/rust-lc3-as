use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

use std::string;

#[derive(Debug, PartialEq, Clone)]
pub struct String {
    token: string::String,
    column: u64,
    line: u64,
}

impl String {
    pub fn new(token: string::String, column: u64, line: u64) -> String {
        String {
            token,
            column,
            line,
        }
    }

    pub fn token(&self) -> &string::String {
        &self.token
    }
}

impl Assemble for String {
    fn assemble(&mut self) {}
}

impl Requirements for String {
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
                "Expected Instruction, Directive, or Label, but found\n {:#?}\n",
                self
            ),
        )));
        _tokens
    }
}
