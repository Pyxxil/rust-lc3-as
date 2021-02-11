use crate::{
    listing,
    token::tokens::traits::Assemble,
    types::{Listings, SymbolTable},
};

use super::traits::Requirements;

token!(Rti);

impl Assemble for Rti {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![listing!(
            0x8000,
            *program_counter - 1,
            self.line,
            symbol,
            "RTI"
        )]
    }
}
