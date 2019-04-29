use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

#[derive(Debug, PartialEq, Clone)]
pub struct Ldr {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Ldr {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
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

impl Assemble for Ldr {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Ldr {
    fn require_range(&self) -> (u64, u64) {
        (3, 3)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        let (min, _) = self.require_range();

        if (min) >= tokens.len() as u64 {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                format!(
                    "Expected two arguments to LDR instruction, but only {} were found",
                    (min) - tokens.len() as u64
                ),
            )));

            return tokens;
        }

        match &tokens[0] {
            &Token::Register(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    self.column,
                    self.line,
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
            &Token::Register(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    self.column,
                    self.line,
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
            &Token::Decimal(_) | &Token::Hexadecimal(_) | &Token::Binary(_) => {}
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    self.column,
                    self.line,
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
