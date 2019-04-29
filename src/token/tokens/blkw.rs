use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

use std::iter;

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

    fn assembled(self) -> Vec<(u16, String)> {
        let value = match self.operands.last().unwrap() {
            TokenType::Binary(binary) => binary.value,
            TokenType::Decimal(decimal) => decimal.value,
            TokenType::Hexadecimal(hexadecimal) => hexadecimal.value,
            _ => 0,
        } as u16;
        iter::repeat(if self.operands.len() == 1 {
            (value, "".to_owned())
        } else {
            (
                value,
                format!(
                    "{0} {1:4X} {1:016b} ({2}) .FILL #{1}",
                    0, value as i16, self.line,
                ),
            )
        })
        .take(match self.operands.first().unwrap() {
            TokenType::Binary(binary) => binary.value,
            TokenType::Decimal(decimal) => decimal.value,
            TokenType::Hexadecimal(hexadecimal) => hexadecimal.value,
            _ => 0,
        } as usize)
        .collect()
    }
}

impl Requirements for Blkw {
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<TokenType>) -> Vec<TokenType> {
        if let Some(token) = tokens.first() {
            match token {
                TokenType::Binary(_)
                | TokenType::Character(_)
                | TokenType::Decimal(_)
                | TokenType::Hexadecimal(_) => {
                    self.operands.push(tokens.remove(0));
                    if let Some(second) = tokens.first() {
                        match second {
                            TokenType::Binary(_)
                            | TokenType::Character(_)
                            | TokenType::Decimal(_)
                            | TokenType::Hexadecimal(_)
                            | TokenType::Label(_) => self.operands.push(tokens.remove(0)),
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
        } else {
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
                self.column as usize,
                self.line as usize,
                self.token.len(),
                "Expected an argument, but found nothing.".to_owned(),
            )));
        }

        tokens
    }
}
