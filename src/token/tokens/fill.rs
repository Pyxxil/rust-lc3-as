use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

token!(Fill, 1);

impl Assemble for Fill {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;
        Vec::new()
    }
}

impl Requirements for Fill {
    fn memory_requirement(&self) -> u16 {
        1
    }

    fn require_range(&self) -> (u64, u64) {
        (1, 0)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            match token {
                Token::Immediate(_) | Token::Character(_) | Token::Label(_) => {
                    self.operands.push(tokens.pop_front().unwrap())
                }
                ref token => {
                    notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                        DiagType::Error,
                        self.column,
                        self.line,
                        self.token.len(),
                        format!(
                            "Expected to find argument of type Immediate, or Label, but found {:#?}",
                            token
                        ),
                    )));
                }
            }
        } else {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                "Expected an argument to .FILL directive, but found the end of file instead."
                    .to_owned(),
            )));
        }

        tokens
    }
}
