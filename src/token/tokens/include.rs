use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

token!(Include, 1);

impl Assemble for Include {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Include {
    fn memory_requirement(&self) -> u16 { 0 } fn require_range(&self) -> (u64, u64) {
        (0, 1)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, _) = self.require_range();

        if (min) > (tokens.len() as u64) {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                "Expected an argument to .INCLUDE directive, but found the end of file instead."
                    .to_owned(),
            )));

            return tokens;
        }

        match &tokens[0] {
            &Token::String(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    self.column,
                    self.line,
                    self.token.len(),
                    format!(
                        "Expected to find argument of type String, but found\n{:#?}",
                        token
                    ),
                )));
                return tokens;
            }
        };

        self.operands.push(tokens.pop_front().unwrap());
        tokens
    }
}
