use token::tokens::traits::*;

use token::Token;

use std::cell::Cell;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Not {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Not {
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

impl Assemble for Not {
    fn assembled(mut self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let destination_register = match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        } as u16;

        let source_register = match self.operands.first() {
            Some(token) => match token {
                Token::Register(register) => u16::from(register.register),
                _ => unreachable!(),
            },
            None => destination_register,
        } as u16;

        let instruction = 0x901F | destination_register << 9 | source_register << 6;

        vec![(
            instruction,
            format!(
                "({0:4X}) {1:04X} {1:016b} ({2: >4}) NOT R{3} R{4}",
                *program_counter - 1,
                instruction,
                self.line,
                destination_register,
                source_register,
            ),
        )]
    }
}

impl Requirements for Not {
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, max) = self.require_range();

        let count = Cell::new(0);

        self.operands = tokens
            .drain_while(|token| match token {
                Token::Register(_) => {
                    count.set(count.get() + 1);
                    count.get() <= max
                }
                _ => false,
            })
            .collect();

        if count.get() < min {
            too_few_operands(
                min,
                count.get(),
                self.token(),
                (self.column, self.line, self.token().len()),
            );
        }

        tokens
    }
}
