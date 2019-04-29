use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    token: String,
    column: u64,
    line: u64,
    pub register: u8,
}

impl Register {
    pub fn new(token: String, column: u64, line: u64) -> Register {
        let register = token.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
        Register {
            token,
            column,
            line,
            register,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Register {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Register {
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
