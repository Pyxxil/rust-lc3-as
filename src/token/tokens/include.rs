use std::collections::VecDeque;

use crate::token::{
    tokens::{expected, too_few_operands, traits::Requirements},
    Token,
};

token!(Include);

impl Requirements for Include {
    fn memory_requirement(&self) -> u16 {
        0
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, String);

        operands_check!(self);

        tokens
    }
}
