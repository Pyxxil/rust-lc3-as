use token::tokens::traits::*;

use token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct End {
    token: String,
    column: u64,
    line: u64,
}

impl End {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for End {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for End {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    // As .END takes not arguments, just do nothing here.
    fn consume(&mut self, tokens: Vec<Token>) -> Vec<Token> {
        tokens
    }
}
