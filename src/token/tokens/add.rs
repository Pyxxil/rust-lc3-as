use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::cell::Cell;

#[derive(Debug, PartialEq, Clone)]
pub struct Add {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Add {
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

impl Assemble for Add {
    fn assemble(&mut self) {}

    fn assembled(mut self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let destination_register = u16::from(match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        });
        let source_one = u16::from(match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        });
        let source_two = if let Some(token) = self.operands.first() {
            match token {
                Token::Register(register) => i16::from(register.register),
                Token::Decimal(decimal) => 0x20 | (decimal.value & 0x1F),
                Token::Hexadecimal(hexadecimal) => 0x20 | (hexadecimal.value & 0x1F),
                Token::Binary(binary) => 0x20 | (binary.value & 0x1F),
                _ => unreachable!(),
            }
        } else {
            source_one as i16
        } as u16;

        let instruction: u16 = 0x1000 | destination_register << 9 | source_one << 6 | source_two;

        *program_counter += 1;

        vec![(
            instruction,
            format!(
                "{0:4X} {1:4X} {1:016b} ({2}) ADD R{3} R{4} {5}{6}",
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

impl Requirements for Add {
    fn require_range(&self) -> (u64, u64) {
        (1, 3)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<Token>) -> Vec<Token> {
        let (min, max) = self.require_range();
        let (column, line, length) = (self.column, self.line, self.token.len());

        let count = Cell::new(0);

        self.operands = tokens
            .drain_while(|token| match token {
                Token::Binary(_)
                | Token::Decimal(_)
                | Token::Character(_)
                | Token::Hexadecimal(_)
                | Token::Register(_) => {
                    count.set(count.get() + 1);
                    count.get() <= max
                }
                _ => false,
            })
            .collect();

        if count.get() < min {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                column,
                line,
                length,
                if tokens.is_empty() {
                    "Expected to find argument of type Immediate or Register, but found nothing"
                        .to_owned()
                } else {
                    format!(
                        "Expected to find argument of type Immediate or Register, but found\n{:#?}",
                        tokens.first().unwrap()
                    )
                },
            )));
        }

        tokens
    }
}
