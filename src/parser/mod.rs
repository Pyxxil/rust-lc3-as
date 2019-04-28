use notifier;
use token::traits::Requirements;
use token::TokenType;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<TokenType>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&mut self) {
        let mut tokens: Vec<TokenType> = self.tokens.drain(..).collect();

        while !tokens.is_empty() {
            let mut token = tokens.remove(0);
            tokens = token.consume(tokens);
            self.tokens.push(token);
        }

        println!("{:#?}", self.tokens);
    }

    pub fn is_okay(&self) -> bool {
        notifier::error_count() == 0
    }

    pub fn tokens(self) -> Vec<TokenType> {
        self.tokens
    }
}
