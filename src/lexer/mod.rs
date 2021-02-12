use std::iter::Iterator;

use crate::{lexer::tokenizer::Tokenizer, notifier, token::Token};

pub mod tokenizer;

/// Lex a file given its content
#[must_use]
pub fn lex(file: &str, content: &str) -> Option<Vec<Token>> {
    let tokens = content
        .lines()
        .enumerate()
        .flat_map(|(line_number, line)| Tokenizer::new(file, &line, line_number as u64 + 1))
        .collect();

    (notifier::error_count() == 0).then(|| tokens)
}
