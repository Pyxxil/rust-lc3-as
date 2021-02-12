use std::collections::{HashMap, VecDeque};

use crate::{
    assembler::Assembler,
    err,
    notifier::{self, DiagType, Diagnostic, Highlight},
    token::{traits::Requirements, Symbol, Token},
    types::SymbolTable,
    warn,
};

#[must_use]
pub fn parse(mut tokens: Vec<Token>) -> Option<(Vec<Token>, SymbolTable)> {
    let mut address = 0;
    let mut parsed_tokens: VecDeque<Token> = tokens.drain(..).collect();

    let mut symbols: SymbolTable = HashMap::new();

    while let Some(mut token) = parsed_tokens.pop_front() {
        parsed_tokens = token.consume(parsed_tokens);

        match &token {
            Token::Label(ref tok) => {
                if symbols.contains_key(tok.token()) {
                    err!(
                        Highlight,
                        (*tok.file()).clone(),
                        tok.column(),
                        tok.line(),
                        tok.token().len(),
                        format!("Duplicate symbol found {}", tok.token())
                    );
                } else if symbols.values().any(|symbol| symbol.address() == address) {
                    warn!(
                        Highlight,
                        (*tok.file()).clone(),
                        tok.column(),
                        tok.line(),
                        tok.token().len(),
                        format!("Multiple symbols found for address {:#X}", address)
                    );
                } else {
                    symbols.insert(
                        tok.token().to_string(),
                        Symbol::new(address, tok.token().to_string()),
                    );
                }
            }
            Token::Include(ref token) => {
                if let Token::String(string) = token.operands().first().unwrap() {
                    let file = string
                        .file()
                        .chars()
                        .take(string.file().rfind(|c| c == '/').unwrap() + 1)
                        .collect::<String>()
                        + string.token();

                    Assembler::from_file(file)
                        .ok()
                        .and_then(|assembler| {
                            assembler.lex().map(|ast| {
                                let length = parsed_tokens.len();
                                parsed_tokens.extend(ast.into_iter().rev());
                                parsed_tokens.rotate_left(length);
                            })
                        })
                        .unwrap();
                } else {
                    unreachable!()
                }
            }
            Token::Orig(ref tok) => {
                address = tok.memory_requirement();
            }
            token => {
                address += token.memory_requirement();
            }
        }

        tokens.push(token);
    }

    (notifier::error_count() == 0).then(|| (tokens, symbols))
}
