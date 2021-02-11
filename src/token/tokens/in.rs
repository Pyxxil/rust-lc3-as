use crate::{
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

        vec![(
            0xF023,
            format!(
                "({:04X}) F023 1111000000100011 ({: >4}) {: <20} IN",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}
