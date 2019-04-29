use token::tokens::traits::*;

use token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Puts {
    token: String,
    column: u64,
    line: u64,
}

impl Puts {
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

impl Assemble for Puts {
    fn assemble(&mut self) {}

    fn assembled(self) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Puts {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    // As PUTS takes no operands, just do nothing here.
    fn consume(&mut self, tokens: Vec<Token>) -> Vec<Token> {
        tokens
    }
}
