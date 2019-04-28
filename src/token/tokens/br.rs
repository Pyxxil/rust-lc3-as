use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Br {
    token: String,
    column: u64,
    line: u64,
    n: bool,
    z: bool,
    p: bool,
    operands: Vec<TokenType>,
}

impl Br {
    pub fn new(token: String, column: u64, line: u64, n: bool, z: bool, p: bool) -> Br {
        Br {
            token,
            column,
            line,
            n,
            z,
            p,
            operands: Vec::with_capacity(1),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Br {
    fn assemble(&mut self) {}
}

impl Requirements for Br {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
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
                    "Expected {} arguments, found {}, for BR Instruction.",
                    min,
                    tokens.len() as u64
                ),
            )));

            return tokens;
        }

        let mut consumed = 0;

        match &tokens[0] {
            &TokenType::Binary(_)
            | &TokenType::Character(_)
            | &TokenType::Decimal(_)
            | &TokenType::Hexadecimal(_)
            | &TokenType::Label(_) => {
                consumed += 1;
            }
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                    DiagnosticType::Error,
                    self.column as usize,
                    self.line as usize,
                    self.token.len(),
                    format!(
                        "Expected an Immediate Literal or Label, but found\n {:#?}",
                        token
                    ),
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
                    "Expected atleast {} argument(s), found {}, for BR instruction.",
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
