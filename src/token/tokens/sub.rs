use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Sub {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Sub {
    pub fn new(token: String, column: u64, line: u64) -> Sub {
        Sub {
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

impl Assemble for Sub {
    fn assemble(&mut self) {}
}

impl Requirements for Sub {
    fn require_range(&self) -> (u64, u64) {
        (2, 3)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<TokenType>) -> Vec<TokenType> {
        let (min, _) = self.require_range();

        if (min) >= tokens.len() as u64 {
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
                self.column as usize,
                self.line as usize,
                self.token.len(),
                "Expected at least one argument for .SUB directive, but found end of file instead."
                    .to_owned(),
            )));

            return tokens;
        }

        let mut consumed = 0;

        match &tokens[0] {
            &TokenType::Register(_) => {
                consumed += 1;
            }
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                    DiagnosticType::Error,
                    self.column as usize,
                    self.line as usize,
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
            if let TokenType::Register(_) = tokens[1] {
                consumed += 1;
            }

            if 2 < tokens.len() as u64 {
                if let TokenType::Register(_) = tokens[2] {
                    consumed += 1;
                };
            }
        }

        for _ in 0..consumed {
            self.operands.push(tokens.remove(0));
        }

        tokens
    }
}
