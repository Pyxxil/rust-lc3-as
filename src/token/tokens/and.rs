use token::tokens::traits::*;

use token::TokenType;

use notifier;
use notifier::{Diagnostic, DiagnosticType, HighlightDiagnostic};

use std::cell::Cell;

#[derive(Debug, PartialEq, Clone)]
pub struct And {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<TokenType>,
}

impl And {
    pub fn new(token: String, column: u64, line: u64) -> And {
        And {
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
    fn assemble(&mut self) {}

    fn assembled(mut self) -> Vec<(u16, String)> {
        let destination_register = u16::from(match self.operands.remove(0) {
            TokenType::Register(register) => register.register,
            _ => 0,
        });
        let source_one = u16::from(match self.operands.remove(0) {
            TokenType::Register(register) => register.register,
            _ => 0,
        });
        let source_two = if let Some(token) = self.operands.first() {
            match token {
                TokenType::Register(register) => i16::from(register.register),
                TokenType::Decimal(decimal) => 0x20 | (decimal.value & 0x1F),
                TokenType::Hexadecimal(hexadecimal) => 0x20 | (hexadecimal.value & 0x1F),
                TokenType::Binary(binary) => 0x20 | (binary.value & 0x1F),
                _ => 0,
            }
        } else {
            source_one as i16
        } as u16;

        let instruction: u16 = 0x5000 | destination_register << 9 | source_one << 6 | source_two;

        vec![(
            instruction,
            format!(
                "{0} {1:4X} {1:016b} ({2}) AND R{3} R{4} {5}{6}",
                0,
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

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, mut tokens: Vec<TokenType>) -> Vec<TokenType> {
        let (min, max) = self.require_range();
        let (column, line, length) = (self.column as usize, self.line as usize, self.token.len());

        let count = Cell::new(0);

        self.operands = tokens
            .drain_while(|token| match token {
                TokenType::Binary(_)
                | TokenType::Decimal(_)
                | TokenType::Character(_)
                | TokenType::Hexadecimal(_)
                | TokenType::Register(_) => {
                    count.set(count.get() + 1);
                    count.get() <= max
                }
                _ => false,
            })
            .collect();

        if count.get() < min {
            notifier::add_diagnostic(Diagnostic::Highlight(HighlightDiagnostic::new(
                DiagnosticType::Error,
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
