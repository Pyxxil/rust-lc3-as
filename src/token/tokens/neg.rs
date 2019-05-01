use token::tokens::traits::*;

use token::Token;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Neg {
    token: String,
    column: u64,
    line: u64,
    operands: Vec<Token>,
}

impl Neg {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
            operands: Vec::with_capacity(2),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl Assemble for Neg {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        Vec::new()
    }
}

impl Requirements for Neg {
    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            match token {
                Token::Register(_) => {
                    self.operands.push(tokens.pop_front().unwrap());
                    if let Some(Token::Register(_)) = tokens.front() {
                        self.operands.push(tokens.pop_front().unwrap())
                    }
                }
                token => {
                    expected(
                        &["Immediate value"],
                        token,
                        (self.column, self.line, self.token().len()),
                    );
                }
            }
        } else {
            too_few_operands(
                1,
                0,
                self.token(),
                (self.column, self.line, self.token().len()),
            );
        }

        tokens
    }
}
