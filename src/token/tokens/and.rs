use token::tokens::traits::*;

use token::Token;

use std::cell::Cell;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct And {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl And {
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

impl Assemble for And {
    fn assembled(mut self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let destination_register = u16::from(match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => 0,
        });
        let source_one = u16::from(match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => 0,
        });
        let source_two = if let Some(token) = self.operands.first() {
            match token {
                Token::Register(register) => i16::from(register.register),
                Token::Immediate(imm) => 0x20 | (imm.value & 0x1F),
                _ => 0,
            }
        } else {
            source_one as i16
        } as u16;

        let instruction: u16 = 0x5000 | destination_register << 9 | source_one << 6 | source_two;

        *program_counter += 1;

        vec![(
            instruction,
            format!(
                "{0:4X} {1:04X} {1:016b} ({2}) AND R{3} R{4} {5}{6}",
                *program_counter - 1,
                instruction,
                self.line,
                destination_register,
                source_one,
                if (instruction & 0x20) == 0 { 'R' } else { '#' },
                ((source_two & 0x1F) << 11) as i16 >> 11
            ),
        )]
    }
}

impl Requirements for And {
    fn require_range(&self) -> (u64, u64) {
        (2, 3)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        let (min, max) = self.require_range();

        let count = Cell::new(0);

        self.operands = tokens
            .drain_while(|token| match token {
                Token::Immediate(_) | Token::Character(_) | Token::Register(_) => {
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
