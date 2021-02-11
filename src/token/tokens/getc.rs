use crate::{
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

        vec![(
            0xF020,
            format!(
                "({:04X}) F020 1111000000100000 ({: >4}) {: <20} GETC",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}
