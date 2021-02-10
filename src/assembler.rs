use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Error, Read},
    sync::Mutex,
};

use crate::{
    lexer::Lexer,
    notifier,
    parser::Parser,
    token::{self, tokens::traits::Assemble, Token},
    types::{Program, SymbolTable},
};

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

#[must_use]
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

    #[must_use]
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

    #[must_use]
    pub fn from_string(content: String) -> Self {
        let file = String::from("temp.asm");
        add_file(file.clone());
        Self { file, content }
    }

    #[must_use]
    pub fn lex(&self) -> Option<Vec<Token>> {
        let mut lexer = Lexer::new(&self.file, &self.content);

        lexer.lex();

        if Lexer::is_okay() {
            Some(lexer.tokens())
        } else {
            None
        }
    }

    #[must_use]
    pub fn parse(ast: Vec<Token>) -> Option<(Vec<Token>, SymbolTable)> {
        let mut parser = Parser::new(ast);
        parser.parse();

        if Parser::is_okay() {
            Some(parser.tokens_and_symbols())
        } else {
            None
        }
    }

    #[must_use]
    pub fn assemble(self, _do_print_ast: bool) -> Option<Program> {
        self.lex()
            .and_then(Self::parse)
            .and_then(Self::do_second_pass)
    }

    fn do_second_pass((tokens, symbols): (Vec<token::Token>, SymbolTable)) -> Option<Program> {
        let mut program_counter: i16 = 0;
        let listings = tokens
            .into_iter()
            .flat_map(|token| {
                let symbol = symbols
                    .iter()
                    .find(|(_, sym)| sym.address() == program_counter as u16)
                    .map_or("", |(_, symbol)| symbol.symbol());

                token.assembled(&mut program_counter, &symbols, symbol)
            })
            .collect();

        if notifier::error_count() > 0 {
            None
        } else {
            Some((symbols, listings))
        }
    }
}

lazy_static! {
    pub static ref FILE_CONTROLLER: Mutex<FileController> = Mutex::new(FileController::default());
}
