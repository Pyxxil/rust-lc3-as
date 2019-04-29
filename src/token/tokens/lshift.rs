use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Lshift {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Lshift {
    pub fn new(token: String, column: u64, line: u64) -> Lshift {
        Lshift {
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

impl Assemble for Lshift {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Lshift {
    fn require_range(&self) -> (u64, u64) {
        (2, 2)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<TokenType>) -> Vec<TokenType> {
        let (min, _) = self.require_range();

        if min > (tokens.len() as u64) {
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
                self.column as usize,
                self.line as usize,
                self.token.len(),
                format!(
                    "Expected {} arguments, found {}, for ADD instruction.",
                    min,
                    tokens.len() as u64
                ),
            )));

            return tokens;
        }

        let destination = tokens.first().unwrap();
        match destination {
            TokenType::Register(_) => self.operands.push(tokens.remove(0)),
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

                return tokens;
            }
        }

        let value = tokens.first().unwrap();
        match value {
            TokenType::Decimal(_) | TokenType::Hexadecimal(_) | TokenType::Binary(_) => {
                self.operands.push(tokens.remove(0))
            }
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                    DiagnosticType::Error,
                    self.column as usize,
                    self.line as usize,
                    self.token.len(),
                    format!(
                        "Expected argument of type Immediate, but found\n{:#?}",
                        token
                    ),
                )));
            }
        }

        tokens
    }
}
