use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

token!(Set, 2);

impl Assemble for Set {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Set {
    fn memory_requirement(&self) -> u16 { 0 } fn require_range(&self) -> (u64, u64) {
        (2, 2)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, _) = self.require_range();

        if (min) > (tokens.len() as u64) {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                format!(
                    "Expected {} arguments, found {}, for .SET directive.",
                    min,
                    tokens.len()
                ),
            )));

            return tokens;
        }

        match &tokens[0] {
            &Token::Register(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    self.column,
                    self.line,
                    self.token.len(),
                    format!(
                        "Expected argument of type Register, but found\n{:#?}",
                        token
                    ),
                )));
            }
        };

        match &tokens[1] {
            &Token::Immediate(_) | &Token::Register(_) | &Token::Label(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    self.column,
                    self.line,
                    self.token.len(),
                    format!(
                        "Expected argument of type Immediate, Label, or Register, but found\n{:#?}",
                        token
                    ),
                )));
            }
        };

        for _ in 0..min {
            self.operands.push(tokens.pop_front().unwrap());
        }

        tokens
    }
}
