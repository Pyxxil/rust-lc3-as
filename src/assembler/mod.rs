use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{BufReader, Read};
use std::sync::Mutex;

use lexer::Lexer;
use parser::Parser;
use token;
use token::tokens::traits::Assemble;
use token::Symbol;
use token::Token;

use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

#[derive(Default, Debug)]
pub struct FileController {
    files: HashMap<String, Vec<String>>,
}

pub fn add_file(file: String) {
    let mut guard = FILE_CONTROLLER.lock().unwrap();
    guard.add_file(file);
}

pub fn add_line(file: &str, line: String) {
    let mut guard = FILE_CONTROLLER.lock().unwrap();
    guard.add_line(&file, line);
}

pub fn get_line(file: &str, line: u64) -> String {
    let guard = FILE_CONTROLLER.lock().unwrap();
    guard.get_line(file, line)
}

impl FileController {
    fn add_file(&mut self, file: String) {
        self.files.insert(file, Vec::new());
    }

    pub fn add_line(&mut self, file: &str, line: String) {
        self.files.get_mut(file).unwrap().push(line);
    }

    pub fn get_line(&self, file: &str, line: u64) -> String {
        self.files.get(file).unwrap()[(line - 1) as usize].clone()
    }

    pub fn remove(&mut self, file: &str) {
        self.files.remove(file);
    }
}

pub struct Assembler {
    file: String,
    content: String,
}

impl Assembler {
    pub fn from_file(file: String) -> Result<Self, Error> {
        add_file(file.to_string());

        let mut content = String::new();

        BufReader::new(File::open(file.clone())?)
            .read_to_string(&mut content)
            .unwrap();

        Ok(Self { file, content })
    }

    pub fn from_string(content: String) -> Self {
        let file = String::from("temp.asm");
        add_file(file.clone());
        Self { file, content }
    }

    pub fn lex(&self) -> Option<Vec<Token>> {
        let mut lexer = Lexer::new(&self.file, &self.content);

        lexer.lex();

        if lexer.is_okay() {
            Some(lexer.tokens())
        } else {
            None
        }
    }

    pub fn parse(&self, ast: Vec<Token>) -> Option<(Vec<Token>, HashMap<String, Symbol>)> {
        let mut parser = Parser::new(ast);
        parser.parse();

        if parser.is_okay() {
            Some(parser.tokens_and_symbols())
        } else {
            None
        }
    }

    pub fn assemble(
        self,
        _do_print_ast: bool,
    ) -> Option<(Self, HashMap<String, Symbol>, Vec<(u16, String)>)> {
        println!("Assembling file {}", self.file);

        self.lex()
            .and_then(|ast| self.parse(ast))
            .and_then(|(tokens, symbols)| {
                let assembled = self.do_second_pass(tokens, &symbols);
                Some((self, symbols, assembled))
            })
    }

    pub fn write(&self, symbols: HashMap<String, Symbol>, assembled: Vec<(u16, String)>) {
        let base_file_name: String = self
            .file
            .chars()
            .take(self.file.rfind(|c| c == '.').unwrap())
            .collect();

        let bin_file = base_file_name.clone() + ".bin";
        let hex_file = base_file_name.clone() + ".hex";
        let lst_file = base_file_name.clone() + ".lst";
        let obj_file = base_file_name.clone() + ".obj";
        let sym_file = base_file_name + ".sym";

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
                .write_all(&[(binary >> 8 & 0xFF as u16) as u8, (binary & 0xFF) as u8])
                .expect("There was a problem generating the binary file");
        });
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

lazy_static! {
    pub static ref FILE_CONTROLLER: Mutex<FileController> = Mutex::new(FileController::default());
}
