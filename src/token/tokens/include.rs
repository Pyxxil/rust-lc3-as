use std::collections::VecDeque;

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::Token;

token!(Include, 1);

impl Requirements for Include {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }
    fn memory_requirement(&self) -> u16 {
        0
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::String, "String");
        }

        operands_check!(self);

        tokens
    }
}
