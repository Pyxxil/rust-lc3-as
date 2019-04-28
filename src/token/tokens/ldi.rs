use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Ldi {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Ldi {
    pub fn new(token: String, column: u64, line: u64) -> Ldi {
        Ldi {
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

impl Assemble for Ldi {
    fn assemble(&mut self) {}
}

impl Requirements for Ldi {
    fn require_range(&self) -> (u64, u64) {
        (2, 2)
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
                    "Expected two arguments to LDI instruction, but only {} were found",
                    tokens.len() as u64
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

        for _ in 0..min {
            self.operands.push(tokens.remove(0));
        }

        tokens
    }
}
