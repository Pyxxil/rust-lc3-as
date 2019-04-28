use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Blkw {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Blkw {
    pub fn new(token: String, column: u64, line: u64) -> Blkw {
        Blkw {
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

impl Assemble for Blkw {
    fn assemble(&mut self) {}
}

impl Requirements for Blkw {
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
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
                    "Expected {} arguments, found {}, for BLKW Directive.",
                    min,
                    tokens.len() as u64
                ),
            )));

            return tokens;
        }

        let mut consumed = 0;

        match tokens[0] {
            TokenType::Binary(_)
            | TokenType::Character(_)
            | TokenType::Decimal(_)
            | TokenType::Hexadecimal(_) => {
                consumed += 1;
                if 1 < tokens.len() as u64 {
                    match tokens[0] {
                        TokenType::Binary(_)
                        | TokenType::Character(_)
                        | TokenType::Decimal(_)
                        | TokenType::Hexadecimal(_)
                        | TokenType::Label(_) => {
                            consumed += 1;
                        }
                        _ => {}
                    };
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

        if consumed < min {
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
                self.column as usize,
                self.line as usize,
                self.token.len(),
                format!(
                    "Expected atleast {} argument(s), found {}, for BLKW Directive.",
                    min, consumed
                ),
            )));

            return tokens;
        }

        for _ in 0..consumed {
            self.operands.push(tokens.remove(0));
        }

        tokens
    }
}
