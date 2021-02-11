use crate::{
    listing,
    token::tokens::traits::{Assemble, Requirements},
    types::{Listings, SymbolTable},
};

token!(Getc);

impl Assemble for Getc {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![listing!(
            0xF020,
            *program_counter - 1,
            self.line,
            symbol,
            "GETC"
        )]
    }
}
