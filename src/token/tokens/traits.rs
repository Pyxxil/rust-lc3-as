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

    /// Get the memory requirement for this token
    fn memory_requirement(&self) -> u16;

    /// Consume from the token stream until we have all the required
    /// operands for this token
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token>;
}
