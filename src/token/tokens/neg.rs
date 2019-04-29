use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

#[derive(Debug, PartialEq, Clone)]
pub struct Neg {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Neg {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
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

    fn consume(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        if let Some(token) = tokens.first() {
            match token {
                Token::Register(_) => {
                    self.operands.push(tokens.remove(0));
                    if let Token::Register(_) = tokens.first().unwrap() {
                        self.operands.push(tokens.remove(0))
                    }
                }
                ref token => {
                    notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                        DiagType::Error,
                        self.column,
                        self.line,
                        self.token.len(),
                        format!("Expected an Immediate Literal, but found\n {:#?}", token),
                    )));
                }
            }
        } else {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                "Expected an argument, but found nothing".to_owned(),
            )));
        }

        tokens
    }
}
