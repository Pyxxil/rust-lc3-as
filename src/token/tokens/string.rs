use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::string;

#[derive(Debug, PartialEq, Clone)]
pub struct String {
    token: string::String,
    column: u64,
    line: u64,
}

impl String {
    pub fn new(token: string::String, column: u64, line: u64) -> Self {
        Self {
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

    fn assembled(self) -> Vec<(u16, std::string::String)> {
        Vec::new()
    }
}

impl Requirements for String {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

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
