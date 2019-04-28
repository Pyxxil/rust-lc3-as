use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Ld {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Ld {
    pub fn new(token: String, column: u64, line: u64) -> Ld {
        Ld {
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

impl Assemble for Ld {
    fn assemble(&mut self) {}
}

impl Requirements for Ld {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
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
                format!(
                    "Expected two arguments to LD instruction, but only {} were found",
                    (min) - tokens.len() as u64
                ),
            )));

            return tokens;
        }

        match &tokens[0] {
            &TokenType::Register(_) => {}
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

        match &tokens[1] {
            &TokenType::Label(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                    DiagnosticType::Error,
                    self.column as usize,
                    self.line as usize,
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
            self.operands.push(tokens.remove(0));
        }

        tokens
    }
}
