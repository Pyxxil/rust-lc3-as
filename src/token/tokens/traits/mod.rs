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
    /* The amount of tokens that this token requires for operands
     *
     * @return A tuple in the form (min, max)
     */
    fn require_range(&self) -> (u64, u64);

    /* The amount of memory that is required by this token
     *
     * @return The memory requirement
     */
    fn memory_requirement(&self) -> u16;

    /* Consume a range of tokens corresponding to Requirements::require_range.1 (at most).
     *
     * @param: tokens The queue containing the tokens we can consume from.
     *
     * @return The tokens not consumed
     */
    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token>;
}
