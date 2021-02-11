use std::collections::VecDeque;

use crate::{
    token::Token,
    types::{Listings, SymbolTable},
};

pub trait Assemble {
    fn assembled(self, program_counter: &mut i16, symbols: &SymbolTable, symbol: &str) -> Listings;
}

pub trait Requirements {
    /// The minimum operands a token will require
    fn min_operands(&self) -> u64 {
        0
    }

    /// Every token will have specific requirements for memory,
    /// and we need to be able to know what that is so we can
    /// place symbols appropriately.
    /// This defaults to 1 as most instructions will only use 1 word.
    fn memory_requirement(&self) -> u16 {
        1
    }

    /// Consume from the token stream until we have all the required
    /// operands for this token
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        tokens
    }
}
