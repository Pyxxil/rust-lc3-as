use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

token!(Orig, 1, starting_address: u16);

impl Assemble for Orig {
    fn assembled(mut self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let instruction = match self.operands.remove(0) {
            Token::Immediate(imm) => imm.value,
            _ => unreachable!(),
        } as u16;

        *program_counter = instruction as i16;

        vec![(
            instruction,
            format!(
                "(0000) {0:4X} {0:016b} ({1: >4}) .ORIG {0:#4X}",
                instruction, self.line,
            ),
        )]
    }
}

impl Requirements for Orig {
    fn memory_requirement(&self) -> u16 { 0 } fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, _) = self.require_range();
        if let Some(token) = tokens.front() {
            match token {
                Token::Immediate(imm) => {
                    self.starting_address = imm.value as u16;
                    self.operands.push(tokens.pop_front().unwrap());
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
