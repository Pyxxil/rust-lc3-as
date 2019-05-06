use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

token!(Lea, 2);

impl Assemble for Lea {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        Vec::new()
    }
}

impl Requirements for Lea {
    fn memory_requirement(&self) -> u16 {
        1
    }

    fn require_range(&self) -> (u64, u64) {
        (2, 2)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, _) = self.require_range();

        if (min) >= tokens.len() as u64 {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                format!(
                    "Expected two arguments to LEA instruction, but only {} were found",
                    (min) - tokens.len() as u64
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
                        "Expected to find argument of type Register, but found\n{:#?}",
                        token
                    ),
                )));
                return tokens;
            }
        };

        match &tokens[1] {
            &Token::Label(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    self.column,
                    self.line,
                    self.token.len(),
                    format!(
                        "Expected to find argument of type Label, but found\n{:#?}",
                        token
                    ),
                )));
                return tokens;
            }
        };

        for _ in 0..2 {
            self.operands.push(tokens.pop_front().unwrap());
        }

        tokens
    }
}
