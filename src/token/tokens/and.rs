use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

extern crate drain_while;
use drain_while::DrainWhileable;

use std::cell::Cell;

#[derive(Debug, PartialEq, Clone)]
pub struct And {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl And {
    pub fn new(token: String, column: u64, line: u64) -> And {
        And {
            token,
            column,
            line,
            operands: Vec::with_capacity(3),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for And {
    fn assemble(&mut self) {}
}

impl Requirements for And {
    fn require_range(&self) -> (u64, u64) {
        (2, 3)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<TokenType>) -> Vec<TokenType> {
        let (min, _max) = self.require_range();
        let (column, line, length) = (self.column as usize, self.line as usize, self.token.len());

        let count = Cell::new(0);

        self.operands = tokens
            .drain_while(|token| match token {
                TokenType::Binary(_)
                | TokenType::Decimal(_)
                | TokenType::Character(_)
                | TokenType::Hexadecimal(_)
                | TokenType::Register(_) => {
                    count.set(count.get() + 1);
                    true
                }
                _ => false,
            })
            .collect();

        if count.get() < min {
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
                column,
                line,
                length,
                if tokens.is_empty() {
                    "Expected to find argument of type Immediate or Register, but found nothing"
                        .to_owned()
                } else {
                    format!(
                        "Expected to find argument of type Immediate or Register, but found\n{:#?}",
                        tokens.first().unwrap()
                    )
                },
            )));
        }

        tokens
    }
}
