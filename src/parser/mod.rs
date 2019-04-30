use notifier;
use token::traits::Requirements;
use token::Symbol;
use token::Token;

use std::collections::HashMap;
use std::collections::VecDeque;

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
                    self.symbols.insert(
                        tok.token().to_string(),
                        Symbol::new(address, tok.token().to_string()),
                    );
                }
                Token::Orig(ref tok) => {
                    address = tok.starting_address;
                }
                _ => {}
            }

            self.tokens.push(token);
        }
    }

    pub fn is_okay(&self) -> bool {
        notifier::error_count() == 0
    }

    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }
}
