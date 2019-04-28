use token::TokenType;

pub trait Assemble {
    fn assemble(&mut self);
}

pub trait Requirements {
    /* The amount of tokens that the token requires for operands.
     */
    fn require_range(&self) -> (u64, u64);

    /* Whether or not the token's requirements have been satisfied.
     */
    fn is_satisfied(&self) -> bool;

    /* Consume a range of tokens corresponding to Requirements::require_amount (at most).
     *
     * @param: from The vector containing the tokens we can consume from.
     * @param at The index to begin consuming from
     *
     * @return The number of consumed tokens (TODO: This might not be required, as Vec::remove does update the length).
     */
    fn consume(&mut self, tokens: Vec<TokenType>) -> Vec<TokenType>;
}
