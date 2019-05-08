use std::collections::HashMap;
use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Symbol;
use token::Token;

token!(Fill, 1);

impl Assemble for Fill {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        symbols: &HashMap<String, Symbol>,
        symbol: &String,
    ) -> Vec<(u16, String)> {
        *program_counter += 1;
        Vec::new()
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
        if let Some(token) = tokens.front() {
            expect!(
                self,
                tokens,
                token,
                Token::Immediate,
                "Immediate",
                Token::Character,
                "Character",
                Token::Label,
                "Label"
            );
        }

        operands_check!(self);

        tokens
    }
}
