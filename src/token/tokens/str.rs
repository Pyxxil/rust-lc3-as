use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Str {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Str {
    pub fn new(token: String, column: u64, line: u64) -> Str {
        Str {
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

impl Assemble for Str {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Str {
    fn require_range(&self) -> (u64, u64) {
        (3, 3)
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
                    "Expected two arguments to STR instruction, but only {} were found",
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
        }

        match &tokens[1] {
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
        }

        match &tokens[2] {
            &TokenType::Decimal(_) | &TokenType::Hexadecimal(_) | &TokenType::Binary(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                    DiagnosticType::Error,
                    self.column as usize,
                    self.line as usize,
                    self.token.len(),
                    format!(
                        "Expected to find argument of type Immediate, but found\n{:#?}",
                        token
                    ),
                )));
                return tokens;
            }
        }

        for _ in 0..3 {
            self.operands.push(tokens.remove(0));
        }

        tokens
    }
}
