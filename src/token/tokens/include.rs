use std::collections::VecDeque;

use crate::token::{
    tokens::{expected, too_few_operands, traits::Requirements},
    Token,
};

token!(Include, 1);

impl Requirements for Include {
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }
    fn memory_requirement(&self) -> u16 {
        0
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Token::String, "String");

        operands_check!(self);

        tokens
    }
}
