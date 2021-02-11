use std::collections::VecDeque;

use crate::{
    notifier::{self, DiagType, Diagnostic, Highlight},
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Token,
    },
    types::{Listings, SymbolTable},
};

token!(Blkw, 2);

impl Assemble for Blkw {
    fn assembled(self, program_counter: &mut i16, symbols: &SymbolTable, symbol: &str) -> Listings {
        let value = if self.operands.len() == 1 {
            0
        } else {
            match self.operands.last().unwrap() {
                Token::Immediate(imm) => imm.value as u16,
                Token::Label(label) => {
                    if let Some(symbol) = symbols.get(label.token()) {
                        symbol.address()
                    } else {
                        undefined!(label);
                        0
                    }
                }
                _ => unreachable!(),
            }
        };

        let mut assembled = vec![(
            value,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} .FILL #{1}",
                *program_counter, value as i16, self.line, symbol,
            ),
        )];

        let count = match self.operands.first().unwrap() {
            Token::Immediate(imm) => imm.value,
            _ => unreachable!(),
        } as usize;

        (1..count).for_each(|_| {
            *program_counter += 1;
            assembled.push((
                value,
                format!(
                    "({0:4X}) {1:04X} {1:016b} ({2: >4})                      .FILL #{1}",
                    *program_counter, value as i16, self.line,
                ),
            ));
        });

        *program_counter += 1;

        assembled
    }
}

impl Requirements for Blkw {
    fn min_operands(&self) -> u64 {
        1
    }

    fn memory_requirement(&self) -> u16 {
        match self.operands.first().unwrap() {
            Token::Immediate(imm) => imm.value as u16,
            _ => unreachable!(),
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Immediate);

        maybe_expect!(self, tokens, Immediate, Character, Label);

        operands_check!(self);

        tokens
    }
}
