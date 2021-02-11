use crate::{
    token::tokens::traits::{Assemble, Requirements},
    types::{Listings, SymbolTable},
};

token!(Out);

impl Assemble for Out {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![(
            0xF021,
            format!(
                "({:04X}) F021 1111000000100001 ({: >4}) {: <20} OUT",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}
