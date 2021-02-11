use crate::{
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

        vec![(
            0xF024,
            format!(
                "({:04X}) F024 1111000000100100 ({: >4}) {: <20} PUTSP",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}
