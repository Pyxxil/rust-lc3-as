use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Symbol;
use token::Token;

token!(Orig, 1, starting_address: u16);

impl Assemble for Orig {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        let instruction = match self.operands.remove(0) {
            Token::Immediate(imm) => imm.value,
            _ => unreachable!(),
        } as u16;

        *program_counter = instruction as i16;

        vec![(
            instruction,
            format!(
                "(0000) {0:4X} {0:016b} ({1: >4}) {2: <20} .ORIG {0:#4X}",
                instruction, self.line, symbol
            ),
        )]
    }
}

impl Requirements for Orig {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn memory_requirement(&self) -> u16 {
        if self.operands.is_empty() {
            0
        } else {
            match self.operands.first().unwrap() {
                Token::Immediate(imm) => imm.value as u16,
                _ => unreachable!(),
            }
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Immediate, "Immediate");
        }

        operands_check!(self);

        tokens
    }
}
