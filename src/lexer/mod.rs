use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind;
use std::iter::Iterator;
use std::sync::Mutex;

use lexer::tokenizer::Tokenizer;
use notifier;
use notifier::{DiagType, Diagnostic, Note};
use token::Token;

pub mod tokenizer;

#[derive(Default, Debug)]
pub struct FileController {
    files: HashMap<String, Vec<String>>,
}

pub fn add_file(file: String) {
    let mut guard = FILE_CONTROLLER.lock().unwrap();
    guard.add_file(file);
}

pub fn add_line(file: String, line: String) {
    let mut guard = FILE_CONTROLLER.lock().unwrap();
    guard.add_line(file, line);
}

pub fn get_line(file: &str, line: u64) -> String {
    let guard = FILE_CONTROLLER.lock().unwrap();
    guard.get_line(file, line)
}

impl FileController {
    fn add_file(&mut self, file: String) {
        self.files.insert(file, Vec::new());
    }

    pub fn add_line(&mut self, file: String, line: String) {
        self.files.get_mut(&file).unwrap().push(line);
    }

    pub fn get_line(&self, file: &str, line: u64) -> String {
        self.files.get(file).unwrap()[(line - 1) as usize].clone()
    }

    pub fn remove(&mut self, file: &str) {
        self.files.remove(file);
    }
}

pub struct Lexer<'a> {
    file: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a str) -> Lexer<'a> {
        Lexer {
            file,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        match File::open(self.file) {
            Ok(file) => {
                add_file(self.file.to_string());
                self.tokens = BufReader::new(file)
                    .lines()
                    .enumerate()
                    .flat_map(|(line_number, line)| {
                        Tokenizer::new(self.file, &line.unwrap(), (line_number + 1) as u64)
                            .collect::<Vec<_>>()
                    })
                    .collect();
            }
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                notifier::add_diagnostic(Diagnostic::Note(Note::new(
                    DiagType::Error,
                    0,
                    0,
                    format!("File '{}' doesn't exist", self.file),
                )));
            }
            Err(_) => {
                notifier::add_diagnostic(Diagnostic::Note(Note::new(
                    DiagType::Error,
                    0,
                    0,
                    format!("Unable to open file '{}'", self.file),
                )));
            }
        }
    }

    #[inline]
    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }

    #[inline]
    pub fn is_okay(&self) -> bool {
        notifier::error_count() == 0
    }
}

lazy_static! {
    pub static ref FILE_CONTROLLER: Mutex<FileController> = Mutex::new(FileController::default());
}
