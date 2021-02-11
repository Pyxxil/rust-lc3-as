use std::collections::VecDeque;

use crate::{
    listing,
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

token!(Jsr, 1);

impl Assemble for Jsr {
    fn assembled(self, program_counter: &mut i16, symbols: &SymbolTable, symbol: &str) -> Listings {
        *program_counter += 1;

        let value = match self.operands.first().unwrap() {
            Token::Immediate(imm) => imm.value,
            Token::Label(label) => {
                if let Some(symbol) = symbols.get(label.token()) {
                    symbol.address() as i16 - *program_counter
                } else {
                    undefined!(label);
                    0
                }
            }
            _ => unreachable!(),
        } as u16;

        let instruction = 0x4800 | value & 0x7FF;

        vec![listing!(
            instruction,
            *program_counter - 1,
            self.line,
            symbol,
            "JSR",
            match self.operands.first().unwrap() {
                Token::Immediate(imm) => format!("#{}", imm.value),
                Token::Label(label) => label.token().to_string(),
                _ => unreachable!(),
            }
        )]
    }
}

impl Requirements for Jsr {
    fn min_operands(&self) -> u64 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Label, Immediate);

        operands_check!(self);

        tokens
    }
}
