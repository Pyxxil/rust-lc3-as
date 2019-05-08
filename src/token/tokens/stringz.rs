use std::collections::HashMap;
use std::collections::VecDeque;

use token::Symbol;
use token::Token;
use token::tokens::expected;
use token::tokens::traits::*;

token!(Stringz, 1);

impl Assemble for Stringz {
    fn assembled(mut self, program_counter: &mut i16, symbols: &HashMap<String, Symbol>, symbol: &String) -> Vec<(u16, String)> {
        *program_counter += self.memory_requirement() as i16;
        Vec::new()
    }
}

impl Requirements for Stringz {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn memory_requirement(&self) -> u16 {
        self.operands.iter().fold(0_u16, |acc, token| match token {
            Token::String(string) => acc + string.token().len() as u16,
            _ => unreachable!(),
        })
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::String, "String");
        }

        let position = tokens.iter().position(|token| {
            if let Token::String(_) = token {
                false
            } else {
                true
            }
        });

        if let Some(position) = position {
            let mut extra_strings = tokens.drain(..position).collect::<Vec<_>>();

            self.operands.append(&mut extra_strings);
        }

        tokens
    }
}
