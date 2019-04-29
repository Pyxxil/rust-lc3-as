use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    token: String,
    column: u64,
    line: u64,
    pub register: u8,
}

impl Register {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        let register = token.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
        Self {
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
