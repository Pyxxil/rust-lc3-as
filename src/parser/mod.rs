use notifier;
use token::traits::Requirements;
use token::Token;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) {
        let mut tokens: Vec<Token> = self.tokens.drain(..).collect();

        while !tokens.is_empty() {
            let mut token = tokens.remove(0);
            tokens = token.consume(tokens);
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
