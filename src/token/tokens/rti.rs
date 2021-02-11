use crate::{
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

        vec![(
            0x8000,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} RTI",
                *program_counter - 1,
                0x8000,
                self.line,
                symbol
            ),
        )]
    }
}
