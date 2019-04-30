pub mod tokenizer;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind;

use token::Token;

use lexer::tokenizer::Tokenizer;

use notifier;
use notifier::{DiagType, Diagnostic, Note};

pub struct Lexer<'a> {
    file: &'a str,
    tokens: Vec<Token>,
    okay: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a str) -> Lexer<'a> {
        Lexer {
            file,
            tokens: Vec::new(),
            okay: true,
        }
    }

    pub fn lex(&mut self) {
        match File::open(self.file) {
            Ok(file) => {
                BufReader::new(file)
                    .lines()
                    .enumerate()
                    .for_each(move |(line_number, line)| {
                        Tokenizer::new(&line.unwrap(), (line_number + 1) as u64)
                            .for_each(|token| self.tokens.push(token));
                    });
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
        self.okay && notifier::error_count() == 0
    }
}
