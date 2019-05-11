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
                let (tokens, symbols) = self.do_first_pass(parser);
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

                match writeln!(sym_f, "{: <20} {}", "Symbol", "Address") {
                    _ => {}
                }
                match writeln!(sym_f, "-------------------- -------") {
                    _ => {}
                }

                symbols.iter().for_each(|(_, symbol)| {
                    match writeln!(sym_f, "{: <20} {:04X}", symbol.symbol(), symbol.address()) {
                        _ => {}
                    };
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
                        .unwrap(),
                );

                assembled.iter().for_each(|assembled_token| {
                    match writeln!(bin_f, "{:016b}", assembled_token.0) {
                        _ => {}
                    };
                    match writeln!(hex_f, "{:04X}", assembled_token.0) {
                        _ => {}
                    };
                    match writeln!(lst_f, "{}", assembled_token.1) {
                        _ => {}
                    };
                    match obj_f.write(&[
                        (assembled_token.0 >> 8 & 0xFF) as u8,
                        (assembled_token.0 & 0xFF) as u8,
                    ]) {
                        _ => {}
                    };
                });
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
