use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

token!(Br, 1, n: bool, z: bool, p: bool);

impl Assemble for Br {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;
        Vec::new()
    }
}

impl Requirements for Br {
    fn memory_requirement(&self) -> u16 {
        1
    }

    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, _) = self.require_range();

        if (min) > (tokens.len() as u64) {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.file.clone(),
                self.column,
                self.line,
                self.token.len(),
                format!(
                    "Expected {} arguments, found {}, for BR Instruction.",
                    min,
                    tokens.len() as u64
                ),
            )));

            return tokens;
        }

        let mut consumed = 0;

        match &tokens[0] {
            &Token::Immediate(_) | &Token::Character(_) | &Token::Label(_) => {
                consumed += 1;
            }
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    self.file.clone(),
                    self.column,
                    self.line,
                    self.token.len(),
                    format!(
                        "Expected an Immediate Literal or Label, but found\n {:#?}",
                        token
                    ),
                )));
            }
        }

        if consumed < min {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.file.clone(),
                self.column,
                self.line,
                self.token.len(),
                format!(
                    "Expected atleast {} argument(s), found {}, for BR instruction.",
                    min, consumed
                ),
            )));

            return tokens;
        }

        for _ in 0..consumed {
            self.operands.push(tokens.pop_front().unwrap());
        }

        tokens
    }
}
