use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Orig {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Orig {
    pub fn new(token: String, column: u64, line: u64) -> Orig {
        Orig {
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

impl Assemble for Orig {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Orig {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<TokenType>) -> Vec<TokenType> {
        let (min, _) = self.require_range();
        if let Some(token) = tokens.first() {
            match token {
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
                            "Expected to find argument of type Immediate, but found {:#?}",
                            token
                        ),
                    )));
                }
            }
        } else {
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
        }

        tokens
    }
}
