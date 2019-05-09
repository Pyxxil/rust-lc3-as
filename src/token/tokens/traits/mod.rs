use std::collections::HashMap;
use std::collections::VecDeque;

use token::Symbol;
use token::Token;

pub trait Assemble {
    fn assembled(
        self,
        program_counter: &mut i16,
        symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)>;
}

pub trait Requirements {
    /* The amount of tokens that the token requires for operands.
     */
    fn require_range(&self) -> (u64, u64);

    /* The amount of memory that is required by this token
     */
    fn memory_requirement(&self) -> u16;

    /* Consume a range of tokens corresponding to Requirements::require_amount (at most).
     *
     * @param: tokens The vector containing the tokens we can consume from.
     * @param at The index to begin consuming from
     *
     * @return The tokens not consumed
     */
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token>;
}
