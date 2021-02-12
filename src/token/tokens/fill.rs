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

token!(Fill);

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

        vec![listing!(
            value,
            *program_counter - 1,
            self.line,
            symbol,
            ".FILL",
            format!("#{}", value as i16)
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
