use std::collections::{HashMap, VecDeque};

use crate::{
    token::{Symbol, Token},
    types::Listings,
};

pub trait Assemble {
    fn assembled(
        self,
        program_counter: &mut i16,
        symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Listings;
}

pub trait Requirements {
    /// The amount of tokens that this token requires for operands
    /// in the form of (min, max)
    fn require_range(&self) -> (u64, u64);

    /// Every token will have specific requirements for memory,
    /// and we need to be able to know what that is so we can
    /// place symbols appropriately.
    fn memory_requirement(&self) -> u16;

    /// Consume from the token stream until we have all the required
    /// operands for this token
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token>;
}
