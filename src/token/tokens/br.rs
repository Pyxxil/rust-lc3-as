use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

#[derive(Debug, PartialEq, Clone)]
pub struct Br {
    token: String,
    column: u64,
    line: u64,
    n: bool,
    z: bool,
    p: bool,
    operands: Vec<Token>,
}

impl Br {
    pub fn new(token: String, column: u64, line: u64, n: bool, z: bool, p: bool) -> Self {
        Self {
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

    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Br {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        let (min, _) = self.require_range();

        if (min) > (tokens.len() as u64) {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
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
            &Token::Binary(_)
            | &Token::Character(_)
            | &Token::Decimal(_)
            | &Token::Hexadecimal(_)
            | &Token::Label(_) => {
                consumed += 1;
            }
            token => {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    self.column,
                    self.line,
                    self.token.len(),
                    format!(
                        "Expected an Immediate Literal or Label, but found\n {:#?}",
                        token
                    ),
                )));
            }
        }

        if consumed < min {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
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
