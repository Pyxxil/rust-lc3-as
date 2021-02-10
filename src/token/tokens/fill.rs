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

token!(Fill, 1);

impl Assemble for Fill {
    fn assembled(
        self,
        program_counter: &mut i16,
        symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let value = match self.operands.first().unwrap() {
            Token::Label(label) => {
                if let Some(symbol) = symbols
                    .iter()
                    .find(|(_, symbol)| symbol.symbol() == label.token())
                {
                    symbol.1.address()
                } else {
                    undefined!(self, label);
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
    fn require_range(&self) -> (u64, u64) {
        (1, 0)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(
            self,
            tokens,
            Token::Immediate,
            "Immediate",
            Token::Character,
            "Character",
            Token::Label,
            "Label"
        );

        operands_check!(self);

        tokens
    }
}
