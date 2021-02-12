use crate::token::tokens::Token;
use crate::{
    listing,
    token::tokens::traits::{Assemble, Requirements},
    types::{Listings, SymbolTable},
};

token!(In);

impl Assemble for In {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![listing!(
            0xF023,
            *program_counter - 1,
            self.line,
            symbol,
            "IN"
        )]
    }
}

impl Requirements for In {}
