use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Neg {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Neg {
    pub fn new(token: String, column: u64, line: u64) -> Neg {
        Neg {
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

impl Assemble for Neg {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Neg {
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<TokenType>) -> Vec<TokenType> {
        if let Some(token) = tokens.first() {
            match token {
                TokenType::Register(_) => {
                    self.operands.push(tokens.remove(0));
                    if let TokenType::Register(_) = tokens.first().unwrap() {
                        self.operands.push(tokens.remove(0))
                    }
                }
                ref token => {
                    notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                        DiagnosticType::Error,
                        self.column as usize,
                        self.line as usize,
                        self.token.len(),
                        format!("Expected an Immediate Literal, but found\n {:#?}", token),
                    )));
                }
            }
        } else {
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
                self.column as usize,
                self.line as usize,
                self.token.len(),
                "Expected an argument, but found nothing".to_owned(),
            )));
        }

        tokens
    }
}
