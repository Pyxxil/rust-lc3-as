use std::collections::HashMap;

use lexer::Lexer;
use parser::Parser;
use token;
use token::tokens::traits::Assemble;
use token::Symbol;

pub struct Assembler {
    file: String,
}

impl Assembler {
    pub fn new(file: String) -> Self {
        Self { file }
    }

    pub fn assemble(&self, _do_print_ast: bool) {
        println!("Assembling file {}", self.file);
        let mut lexer = Lexer::new(&self.file);

        lexer.lex();

        if lexer.is_okay() {
            let mut parser = Parser::new(lexer.tokens());
            parser.parse();
            if parser.is_okay() {
                let (tokens, symbols) = self.do_first_pass(parser);
                let tokens = self.do_second_pass(tokens, symbols);
                println!("{:#?}", tokens);
                return;
            }
        }

        println!("Assembly failed for {}", self.file);
    }

    fn do_first_pass(&self, parser: Parser) -> (Vec<token::Token>, HashMap<String, Symbol>) {
        parser.tokens_and_symbols()
    }

    fn do_second_pass(
        &self,
        tokens: Vec<token::Token>,
        symbols: HashMap<String, Symbol>,
    ) -> Vec<(u16, String)> {
        let mut program_counter: i16 = 0;
        tokens
            .into_iter()
            .flat_map(|token| {
                let symbol = if let Some(symbol) = symbols
                    .iter()
                    .find(|(_, sym)| sym.address() == program_counter as u16)
                {
                    symbol.1.symbol()
                } else {
                    ""
                };
                token.assembled(&mut program_counter, &symbols, symbol)
            })
            .collect()
    }
}
