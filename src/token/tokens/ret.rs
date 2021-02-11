use crate::{
    token::tokens::traits::{Assemble, Requirements},
    types::{Listings, SymbolTable},
};

token!(Ret);

impl Assemble for Ret {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![(
            0xC1C0,
            format!(
                "({:04X}) C1C0 1100000111000000 ({: >4}) {: <20} RET",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}
