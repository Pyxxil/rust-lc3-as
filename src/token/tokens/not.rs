use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Not {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Not {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
            operands: Vec::with_capacity(2),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Not {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Not {
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, _) = self.require_range();

        if (min) >= tokens.len() as u64 {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                    "Expected at least one argument for NOT instruction, but found end of file instead.".to_owned()
            )));

            return tokens;
        }

        let mut consumed = 0;

        match &tokens[0] {
            &Token::Register(_) => {
                consumed += 1;
            }
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

        if 1 < tokens.len() as u64 {
            if let Token::Register(_) = tokens[1] {
                consumed += 1;
            }
        }

        for _ in 0..consumed {
            self.operands.push(tokens.pop_front().unwrap());
        }

        tokens
    }
}
