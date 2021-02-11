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

token!(Fill, 1);

impl Assemble for Fill {
    fn assembled(self, program_counter: &mut i16, symbols: &SymbolTable, symbol: &str) -> Listings {
        *program_counter += 1;

        let value = match self.operands.first().unwrap() {
            Token::Label(label) => {
                if let Some(symbol) = symbols
                    .iter()
                    .find(|(_, symbol)| symbol.symbol() == label.token())
                {
                    symbol.1.address()
                } else {
                    undefined!(label);
                    0
                }
            }
            Token::Character(character) => character.token().chars().next().unwrap() as u16,
            Token::Immediate(immediate) => immediate.value as u16,
            _ => unreachable!(),
        };

        vec![(
            value,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} .FILL #{1}",
                *program_counter - 1,
                value as i16,
                self.line,
                symbol
            ),
        )]
    }
}

impl Requirements for Fill {
    fn min_operands(&self) -> u64 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Immediate, Character, Label);

        operands_check!(self);

        tokens
    }
}
