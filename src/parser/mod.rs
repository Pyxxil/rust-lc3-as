use token::traits::Requirements;
use token::Symbol;
use token::Token;

use std::collections::HashMap;
use std::collections::VecDeque;

use notifier;
use notifier::{Diagnostic, DiagType, Highlight};

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
        let mut tokens = self.tokens.drain(..).collect::<VecDeque<_>>();

        while let Some(mut token) = tokens.pop_front() {
            tokens = token.consume(tokens);

            match &token {
                Token::Label(ref tok) => {
                    if self.symbols.contains_key(tok.token()) {
                        notifier::add_diagnostic(
                            Diagnostic::Highlight(Highlight::new(
                                DiagType::Error,
                                tok.column(),
                                tok.line(),
                                tok.token().len(),
                                format!(
                                    "Duplicate symbol found {}",
                                    tok.token()
                                )
                            ))
                        );
                    } else {
                        self.symbols.insert(
                            tok.token().to_string(),
                            Symbol::new(address, tok.token().to_string()),
                        );
                    }
                }
                Token::Orig(ref tok) => {
                    address = tok.starting_address;
                }
                _ => {
                    address += 1;
                }
            }

            self.tokens.push(token);
        }

        println!("{:#?}", self.symbols);
    }

    pub fn is_okay(&self) -> bool {
        notifier::error_count() == 0
    }

    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }

    pub fn symbols(self) -> HashMap<String, Symbol> {
        self.symbols
    }
}
