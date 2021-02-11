use crate::{
    token::tokens::traits::{Assemble, Requirements},
    types::{Listings, SymbolTable},
};

token!(Puts);

impl Assemble for Puts {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![(
            0xF022,
            format!(
                "({:04X}) F022 1111000000100010 ({: >4}) {: <20} PUTS",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}
