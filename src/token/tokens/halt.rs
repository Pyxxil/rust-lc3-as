use token::tokens::traits::*;

use token::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub struct Halt {
    token: String,
    column: u64,
    line: u64,
}

impl Halt {
    pub fn new(token: String, column: u64, line: u64) -> Halt {
        Halt {
            token,
            column,
            line,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Halt {
    fn assemble(&mut self) {}
}

impl Requirements for Halt {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    // As HALT takes no operands, just do nothing here.
    fn consume(&mut self, mut _tokens: Vec<TokenType>) -> Vec<TokenType> {
        _tokens
    }
}
