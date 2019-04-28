use token::tokens::traits::*;

use token::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub struct Out {
    token: String,
    column: u64,
    line: u64,
}

impl Out {
    pub fn new(token: String, column: u64, line: u64) -> Out {
        Out {
            token,
            column,
            line,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Out {
    fn assemble(&mut self) {}
}

impl Requirements for Out {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    // As OUT takes no operands, just do nothing here.
    fn consume(&mut self, mut _tokens: Vec<TokenType>) -> Vec<TokenType> {
        _tokens
    }
}
