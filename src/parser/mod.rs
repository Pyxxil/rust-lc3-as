use std::collections::HashMap;
use std::collections::VecDeque;

use notifier;
use notifier::{Diagnostic, DiagType, Highlight};
use token::Symbol;
use token::Token;
use token::traits::Requirements;

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
                        notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                            DiagType::Error,
                            (*tok.file()).clone(),
                            tok.column(),
                            tok.line(),
                            tok.token().len(),
                            format!("Duplicate symbol found {}", tok.token()),
                        )));
                    } else {
                        self.symbols.insert(
                            tok.token().to_string(),
                            Symbol::new(address, tok.token().to_string()),
                        );
                    }
                }
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

    pub fn tokens_and_symbols(self) -> (Vec<Token>, HashMap<String, Symbol>)  {
        (self.tokens, self.symbols)
    }
}
