use std::collections::HashMap;

use lexer::Lexer;
use parser::Parser;
use token;
use token::tokens::traits::Assemble;
use token::Symbol;

use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

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
                let (tokens, symbols) = parser.tokens_and_symbols();
                let assembled = self.do_second_pass(tokens, &symbols);

                let base_file_name: String = self
                    .file
                    .chars()
                    .take(self.file.rfind(|c| c == '.').unwrap())
                    .collect();

                let bin_file = base_file_name.clone() + ".bin";
                let hex_file = base_file_name.clone() + ".hex";
                let lst_file = base_file_name.clone() + ".lst";
                let obj_file = base_file_name.clone() + ".obj";
                let sym_file = base_file_name.clone() + ".sym";

                let mut sym_f = BufWriter::new(
                    OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(sym_file)
                        .unwrap(),
                );

                writeln!(
                    sym_f,
                    "{: <20} Assembler\n-------------------- -------",
                    "Symbol"
                )
                .unwrap();

                symbols.into_iter().for_each(|(_, symbol)| {
                    writeln!(sym_f, "{: <20} {:04X}", symbol.symbol(), symbol.address()).unwrap();
                });

                let mut bin_f = BufWriter::new(
                    OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(bin_file)
                        .unwrap(),
                );

                let mut hex_f = BufWriter::new(
                    OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(hex_file)
                        .unwrap(),
                );

                let mut lst_f = BufWriter::new(
                    OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(lst_file)
                        .unwrap(),
                );

                let mut obj_f = BufWriter::new(
                    OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(obj_file)
                        .expect("Can't create object file"),
                );

                assembled.iter().for_each(|(binary, listing)| {
                    writeln!(bin_f, "{:016b}", binary).unwrap();
                    writeln!(hex_f, "{:04X}", binary).unwrap();
                    writeln!(lst_f, "{}", listing).unwrap();
                    obj_f
                        .write(&[(binary >> 8 & 0xFF as u16) as u8, (binary & 0xFF) as u8])
                        .unwrap();
                });

                return;
            }
        }

        println!("Assembly failed for {}", self.file);
    }

    fn do_second_pass(
        &self,
        tokens: Vec<token::Token>,
        symbols: &HashMap<String, Symbol>,
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
