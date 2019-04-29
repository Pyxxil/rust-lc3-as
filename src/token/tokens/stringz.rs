use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

#[derive(Debug, PartialEq, Clone)]
pub struct Stringz {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Stringz {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
            operands: Vec::with_capacity(1),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Stringz {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Stringz {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        let (min, _) = self.require_range();

        if (min) > (tokens.len() as u64) {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                "Expected an argument to .STRINGZ directive, but found the end of file instead."
                    .to_owned(),
            )));

            return tokens;
        }

        let mut consumed: usize = 0;

        match &tokens[0] {
            &Token::String(_) => {
                consumed += 1;
            }
            token => {
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

                return tokens;
            }
        };

        while consumed < tokens.len() {
            match tokens[consumed] {
                Token::String(_) => consumed += 1,
                _ => break,
            }
        }

        for _ in 0..consumed {
            self.operands.push(tokens.remove(0));
        }
        tokens
    }
}
