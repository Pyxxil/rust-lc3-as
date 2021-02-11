use crate::{
    listing,
    token::tokens::traits::{Assemble, Requirements},
    types::{Listings, SymbolTable},
};

token!(Putsp);

impl Assemble for Putsp {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![listing!(
            0xF024,
            *program_counter - 1,
            self.line,
            symbol,
            "PUTSP"
        )]
    }
}
