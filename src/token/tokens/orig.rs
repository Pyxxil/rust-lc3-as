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

        if (min) > tokens.len() as u64 {
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
        };

        match &tokens[0] {
            &TokenType::Decimal(_) | &TokenType::Hexadecimal(_) | &TokenType::Binary(_) => {}
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

                return tokens;
            }
        }

        self.operands.push(tokens.remove(0));
        tokens
    }
}
