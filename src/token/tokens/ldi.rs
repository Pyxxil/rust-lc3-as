use std::collections::{HashMap, VecDeque};

use crate::{
    notifier::{self, DiagType, Diagnostic, Highlight},
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Symbol, Token,
    },
    types::Listings,
};

token!(Ldi, 2);

impl Assemble for Ldi {
    fn assembled(
        self,
        program_counter: &mut i16,
        symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let destination_register = match self.operands.first().unwrap() {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let offset = match self.operands.last().unwrap() {
            Token::Immediate(imm) => imm.value,
            Token::Label(label) => {
                if let Some(symbol) = symbols.get(label.token()) {
                    symbol.address() as i16 - *program_counter
                } else {
                    undefined!(self, label);
                    0
                }
            }
            _ => unreachable!(),
        } as u16;

        let instruction = 0xA000 | destination_register << 9 | offset & 0x1FF;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} LDI R{4} {5}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                destination_register,
                match self.operands.last().unwrap() {
                    Token::Immediate(imm) => format!("#{}", imm.value),
                    Token::Label(label) => label.token().to_string(),
                    _ => unreachable!(),
                }
            ),
        )]
    }
}

impl Requirements for Ldi {
    fn require_range(&self) -> (u64, u64) {
        (2, 2)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Token::Register, "Register");

        expect!(
            self,
            tokens,
            Token::Label,
            "Label",
            Token::Immediate,
            "Immediate"
        );

        operands_check!(self);

        tokens
    }
}
