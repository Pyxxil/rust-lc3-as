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

token!(Ld);

impl Assemble for Ld {
    fn assembled(self, program_counter: &mut i16, symbols: &SymbolTable, symbol: &str) -> Listings {
        *program_counter += 1;

        let destination_register = if let Token::Register(register) = self.operands.first().unwrap()
        {
            register.register
        } else {
            unreachable!()
        };

        let offset = match self.operands.last().unwrap() {
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

        let instruction = 0x2000 | destination_register << 9 | offset & 0x1FF;

        vec![listing!(
            instruction,
            *program_counter - 1,
            self.line,
            symbol,
            "LD",
            format!("R{}", destination_register),
            match self.operands.last().unwrap() {
                Token::Immediate(imm) => format!("#{}", imm.value),
                Token::Label(label) => label.token().to_string(),
                _ => unreachable!(),
            }
        )]
    }
}

impl Requirements for Ld {
    fn min_operands(&self) -> u64 {
        2
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        expect!(self, tokens, Immediate, Label);

        operands_check!(self);

        tokens
    }
}
