use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

#[derive(Debug, PartialEq, Clone)]
pub struct Orig {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Orig {
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

impl Assemble for Orig {
    fn assemble(&mut self) {}

    fn assembled(mut self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let instruction = match self.operands.remove(0) {
            Token::Register(register) => i16::from(register.register),
            Token::Decimal(decimal) => decimal.value,
            Token::Hexadecimal(hexadecimal) => hexadecimal.value,
            Token::Binary(binary) => binary.value,
            _ => unreachable!(),
        } as u16;

        *program_counter = instruction as i16;

        vec![(
            instruction,
            format!(
                "0000 {0:4X} {0:016b} ({1}) .ORIG {0:#4X}",
                instruction, self.line,
            ),
        )]
    }
}

impl Requirements for Orig {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        let (min, _) = self.require_range();
        if let Some(token) = tokens.first() {
            match token {
                Token::Decimal(_) | Token::Hexadecimal(_) | Token::Binary(_) => {
                    self.operands.push(tokens.remove(0))
                }
                token => {
                    notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                        DiagType::Error,
                        self.column,
                        self.line,
                        self.token.len(),
                        format!(
                            "Expected to find argument of type Immediate, but found {:#?}",
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
                format!(
                    "Expected {} arguments, found {}, for ADD instruction.",
                    min,
                    tokens.len() as u64
                ),
            )));
        }

        tokens
    }
}
