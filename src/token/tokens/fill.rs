use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

#[derive(Debug, PartialEq, Clone)]
pub struct Fill {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Fill {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
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

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Fill {
    fn require_range(&self) -> (u64, u64) {
        (1, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        if let Some(token) = tokens.first() {
            match token {
                Token::Binary(_)
                | Token::Decimal(_)
                | Token::Hexadecimal(_)
                | Token::Character(_)
                | Token::Label(_) => self.operands.push(tokens.remove(0)),
                ref token => {
                    notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                        DiagType::Error,
                        self.column,
                        self.line,
                        self.token.len(),
                        format!(
                            "Expected to find argument of type Immediate, or Label, but found {:#?}",
                            token
                        ),
                    )));
                }
            }
        } else {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                "Expected an argument to .FILL directive, but found the end of file instead."
                    .to_owned(),
            )));
        }

        tokens
    }
}
