use std::iter::Iterator;

use lexer::tokenizer::Tokenizer;
use notifier;
use token::Token;

pub mod tokenizer;

pub struct Lexer<'a> {
    file: &'a str,
    content: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub fn new(file: &'a str, content: &'a str) -> Lexer<'a> {
        Lexer {
            file,
            content,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        self.tokens = self
            .content
            .lines()
            .enumerate()
            .flat_map(|(line_number, line)| {
                Tokenizer::new(self.file, &line, (line_number + 1) as u64).collect::<Vec<_>>()
            })
            .collect();
    }

    #[inline]
    #[must_use]
    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }

    #[inline]
    #[must_use]
    pub fn is_okay() -> bool {
        notifier::error_count() == 0
    }
}
