use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Set {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Set {
    pub fn new(token: String, column: u64, line: u64) -> Set {
        Set {
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

impl Assemble for Set {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Set {
    fn require_range(&self) -> (u64, u64) {
        (2, 2)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<TokenType>) -> Vec<TokenType> {
        let (min, _) = self.require_range();

        if (min) > (tokens.len() as u64) {
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
                self.column as usize,
                self.line as usize,
                self.token.len(),
                format!(
                    "Expected {} arguments, found {}, for .SET directive.",
                    min,
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
                        "Expected argument of type Register, but found\n{:#?}",
                        token
                    ),
                )));
            }
        };

        match &tokens[1] {
            &TokenType::Decimal(_)
            | &TokenType::Hexadecimal(_)
            | &TokenType::Binary(_)
            | &TokenType::Register(_)
            | &TokenType::Label(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                    DiagnosticType::Error,
                    self.column as usize,
                    self.line as usize,
                    self.token.len(),
                    format!(
                        "Expected argument of type Immediate, Label, or Register, but found\n{:#?}",
                        token
                    ),
                )));
            }
        };

        for _ in 0..min {
            self.operands.push(tokens.remove(0));
        }

        tokens
    }
}
