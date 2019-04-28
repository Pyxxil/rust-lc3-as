use token::tokens::traits::*;

use token::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub struct Ret {
    token: String,
    column: u64,
    line: u64,
}

impl Ret {
    pub fn new(token: String, column: u64, line: u64) -> Ret {
        Ret {
            token,
            column,
            line,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Ret {
    fn assemble(&mut self) {}
}

impl Requirements for Ret {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    // As RET takes no operands, just do nothing here.
    fn consume(&mut self, mut _tokens: Vec<TokenType>) -> Vec<TokenType> {
        _tokens
    }
}
