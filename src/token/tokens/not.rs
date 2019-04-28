use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

#[derive(Debug, PartialEq, Clone)]
pub struct Not {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl Not {
    pub fn new(token: String, column: u64, line: u64) -> Not {
        Not {
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

impl Assemble for Not {
    fn assemble(&mut self) {}
}

impl Requirements for Not {
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
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
                    "Expected at least one argument for NOT instruction, but found end of file instead.".to_owned()
            )));

            return tokens;
        }

        let mut consumed = 0;

        match &tokens[0] {
            &TokenType::Register(_) => {
                consumed += 1;
            }
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
        };

        if 1 < tokens.len() as u64 {
            if let TokenType::Register(_) = tokens[1] {
                consumed += 1;
            }
        }

        for _ in 0..consumed {
            self.operands.push(tokens.remove(0));
        }

        tokens
    }
}
