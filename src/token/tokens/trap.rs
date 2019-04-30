use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Trap {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Trap {
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

impl Assemble for Trap {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        let instruction = 0xF000
            | (match self.operands.first().unwrap() {
                Token::Immediate(imm) => imm.value,
                _ => unreachable!(),
            } & 0xFF) as u16;

        vec![(
            instruction,
            format!(
                "{0:04X} {1:4X} {1:016b} ({2}) TRAP 0x{3:02X}",
                *program_counter - 1,
                instruction,
                instruction & 0xFF,
                self.line
            ),
        )]
    }
}

impl Requirements for Trap {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, _) = self.require_range();

        if (min) > (tokens.len() as u64) {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                "Expected an argument for TRAP instruction, but found the end of file instead."
                    .to_owned(),
            )));

            return tokens;
        }

        match &tokens[0] {
            &Token::Immediate(_) => {}
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
                return tokens;
            }
        };

        self.operands.push(tokens.pop_front().unwrap());
        tokens
    }
}
