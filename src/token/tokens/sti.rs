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

token!(Sti, 2);

impl Assemble for Sti {
    fn assembled(self, program_counter: &mut i16, symbols: &SymbolTable, symbol: &str) -> Listings {
        *program_counter += 1;

        let source_register = if let Token::Register(register) = self.operands.first().unwrap() {
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

        let instruction = 0xB000 | source_register << 9 | offset & 0x1FF;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} STI R{4} {5}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                source_register,
                match self.operands.last().unwrap() {
                    Token::Immediate(imm) => format!("#{}", imm.value),
                    Token::Label(label) => label.token().to_string(),
                    _ => unreachable!(),
                }
            ),
        )]
    }
}

impl Requirements for Sti {
    fn min_operands(&self) -> u64 {
        2
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        expect!(self, tokens, Label, Immediate);

        operands_check!(self);

        tokens
    }
}
