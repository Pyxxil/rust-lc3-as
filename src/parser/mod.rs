use std::collections::HashMap;
use std::collections::VecDeque;

use assembler::Assembler;
use notifier;
use notifier::{DiagType, Diagnostic, Highlight};
use token::traits::Requirements;
use token::Symbol;
use token::Token;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    symbols: HashMap<String, Symbol>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            symbols: HashMap::new(),
        }
    }

    pub fn parse(&mut self) {
        let mut address = 0;
        let mut tokens: VecDeque<Token> = self.tokens.drain(..).collect();

        while let Some(mut token) = tokens.pop_front() {
            tokens = token.consume(tokens);

            match &token {
                Token::Label(ref tok) => {
                    if self.symbols.contains_key(tok.token()) {
                        notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                            DiagType::Error,
                            (*tok.file()).clone(),
                            tok.column(),
                            tok.line(),
                            tok.token().len(),
                            format!("Duplicate symbol found {}", tok.token()),
                        )));
                    } else if self
                        .symbols
                        .values()
                        .any(|symbol| symbol.address() == address)
                    {
                        notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                            DiagType::Warning,
                            (*tok.file()).clone(),
                            tok.column(),
                            tok.line(),
                            tok.token().len(),
                            format!("Multiple symbols found for address {:#X}", address),
                        )));
                    } else {
                        self.symbols.insert(
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
                                assembler.lex().and_then(|ast| {
                                    let length = tokens.len();
                                    tokens.extend(ast.into_iter().rev());
                                    tokens.rotate_left(length);

                                    Some(())
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

            self.tokens.push(token);
        }
    }

    pub fn is_okay(&self) -> bool {
        notifier::error_count() == 0
    }

    pub fn tokens_and_symbols(self) -> (Vec<Token>, HashMap<String, Symbol>) {
        (self.tokens, self.symbols)
    }
}
