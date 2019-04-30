use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Character {
    token: String,
    column: u64,
    line: u64,
}

impl Character {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Character {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Character {
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
                "Expected Instruction, Directive, or Label, but found\n {:#?}\n",
                self
            ),
        )));
        tokens
    }
}
