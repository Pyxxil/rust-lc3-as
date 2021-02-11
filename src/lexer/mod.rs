use std::iter::Iterator;

use crate::{lexer::tokenizer::Tokenizer, notifier, token::Token};

pub mod tokenizer;

#[must_use]
pub fn lex(file: &str, content: &str) -> Option<Vec<Token>> {
    let tokens = content
        .lines()
        .enumerate()
        .flat_map(|(line_number, line)| Tokenizer::new(file, &line, (line_number + 1) as u64))
        .collect();

    if notifier::error_count() == 0 {
        Some(tokens)
    } else {
        None
    }
}
