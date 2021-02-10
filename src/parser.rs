use std::collections::{HashMap, VecDeque};

use crate::{
    assembler::Assembler,
    notifier::{self, DiagType, Diagnostic, Highlight},
    token::{traits::Requirements, Symbol, Token},
};

#[must_use]
pub fn parse(mut tokens: Vec<Token>) -> Option<(Vec<Token>, HashMap<String, Symbol>)> {
    let mut address = 0;
    let mut parsed_tokens: VecDeque<Token> = tokens.drain(..).collect();

    let mut symbols: HashMap<String, Symbol> = HashMap::new();

    while let Some(mut token) = parsed_tokens.pop_front() {
        parsed_tokens = token.consume(parsed_tokens);

        match &token {
            Token::Label(ref tok) => {
                if symbols.contains_key(tok.token()) {
                    notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                        DiagType::Error,
                        (*tok.file()).clone(),
                        tok.column(),
                        tok.line(),
                        tok.token().len(),
                        format!("Duplicate symbol found {}", tok.token()),
                    )));
                } else if symbols.values().any(|symbol| symbol.address() == address) {
                    notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                        DiagType::Warning,
                        (*tok.file()).clone(),
                        tok.column(),
                        tok.line(),
                        tok.token().len(),
                        format!("Multiple symbols found for address {:#X}", address),
                    )));
                } else {
                    symbols.insert(
                        tok.token().to_string(),
                        Symbol::new(address, tok.token().to_string()),
                    );
                }
            }
            Token::Include(ref token) => match token.operands().first().unwrap() {
                Token::String(string) => {
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
                }
                _ => unreachable!(),
            },
            Token::Orig(ref tok) => {
                address = tok.memory_requirement();
            }
            token => {
                address += token.memory_requirement();
            }
        }

        tokens.push(token);
    }

    if notifier::error_count() == 0 {
        Some((tokens, symbols))
    } else {
        None
    }
}
