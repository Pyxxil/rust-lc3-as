use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Fill {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Fill {
    pub fn new(token: String, column: u64, line: u64) -> Fill {
        Fill {
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

impl Assemble for Fill {
    fn assemble(&mut self) {}
}

impl Requirements for Fill {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<TokenType>) -> Vec<TokenType> {
        let (min, _) = self.require_range();

        if min > tokens.len() as u64 {
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
                self.column as usize,
                self.line as usize,
                self.token.len(),
                "Expected an argument to .FILL directive, but found the end of file instead."
                    .to_owned(),
            )));

            return tokens;
        }

        match &tokens[0] {
            &TokenType::Binary(_)
            | &TokenType::Decimal(_)
            | &TokenType::Hexadecimal(_)
            | &TokenType::Character(_)
            | &TokenType::Label(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                    DiagnosticType::Error,
                    self.column as usize,
                    self.line as usize,
                    self.token.len(),
                    format!(
                        "Expected to find argument of type Immediate, or Label, but found {:#?}",
                        token
                    ),
                )));
                return tokens;
            }
        };

        self.operands.push(tokens.remove(0));
        tokens
    }
}
