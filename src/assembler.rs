use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Error, Read},
    sync::Mutex,
};

use crate::{
    lexer, notifier, parser,
    token::{tokens::traits::Assemble, traits::Requirements, Token},
    types::{Program, SymbolTable},
};

#[derive(Default)]
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
    /// Create an assembler for a specific file
    ///
    /// # Errors
    ///   If the file fails to be opened/read from
    pub fn from_file(file: String) -> Result<Self, Error> {
        add_file(file.to_string());

        let mut content = String::new();

        BufReader::new(File::open(file.clone())?).read_to_string(&mut content)?;

        Ok(Self { file, content })
    }

    #[must_use]
    pub fn from_string(content: String) -> Self {
        let file = String::from("temp.asm");
        add_file(file.clone());
        Self { file, content }
    }

    #[must_use]
    pub(crate) fn lex(&self) -> Option<Vec<Token>> {
        lexer::lex(&self.file, &self.content)
    }

    #[must_use]
    fn parse(ast: Vec<Token>) -> Option<(Vec<Token>, SymbolTable)> {
        parser::parse(ast)
    }

    #[must_use]
    pub fn assemble(self, _do_print_ast: bool) -> Option<Program> {
        self.lex()
            .and_then(Self::parse)
            .and_then(Self::do_second_pass)
    }

    fn do_second_pass((tokens, symbols): (Vec<Token>, SymbolTable)) -> Option<Program> {
        let mut program_counter: i16 = 0;

        // Initially order the symbols by their addresses, so that the search for a
        // symbol is O(1)
        let mut syms = symbols.values().collect::<Vec<_>>();
        syms.sort_by_key(|sym| sym.address());

        let mut syms = syms.iter().peekable();

        let listings = tokens
            .into_iter()
            .flat_map(|token| {
                // Ignore anything that doesn't have a memory requirement (which should basically be just
                // labels, origins and ends)
                let symbol = if token.memory_requirement() > 0 {
                    match syms.peek() {
                        Some(&&symbol) if symbol.address() == program_counter as u16 => {
                            let _ = syms.next();
                            symbol.symbol()
                        }
                        _ => "",
                    }
                } else {
                    ""
                };

                token.assembled(&mut program_counter, &symbols, symbol)
            })
            .collect();

        if notifier::error_count() == 0 {
            Some((symbols, listings))
        } else {
            None
        }
    }
}

lazy_static! {
    pub static ref FILE_CONTROLLER: Mutex<FileController> = Mutex::new(FileController::default());
}
