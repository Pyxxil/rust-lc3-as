use crate::{
    token::tokens::traits::{Assemble, Requirements},
    types::{Listings, SymbolTable},
};

token!(Halt);

impl Assemble for Halt {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        vec![(
            0xF025,
            format!(
                "({:04X}) F025 1111000000100101 ({: >4}) {: <20} HALT",
                *program_counter - 1,
                self.line,
                symbol
            ),
        )]
    }
}
