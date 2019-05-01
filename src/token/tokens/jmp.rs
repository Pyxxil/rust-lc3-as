use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Jmp {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Jmp {
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

impl Assemble for Jmp {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        let register = match self.operands.first().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        } as u16;

        let instruction = 0xC000 | register << 6;

        vec![(
            instruction,
            format!(
                "({0:4X}) {1:04X} {1:016b} ({2: >4}) JMP R{3}",
                *program_counter - 1,
                instruction,
                self.line,
                register,
            ),
        )]
    }
}

impl Requirements for Jmp {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            match token {
                Token::Register(_) => self.operands.push(tokens.pop_front().unwrap()),
                tok => expected(
                    &["Register"],
                    &tok,
                    (self.column, self.line, self.token().len()),
                ),
            }
        } else {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                self.column,
                self.line,
                self.token.len(),
                "Expected an argument to JMP instruction, but found the end of file instead."
                    .to_owned(),
            )));
        }

        tokens
    }
}
