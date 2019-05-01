use token::tokens::traits::*;

use token::Token;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    token: String,
    column: u64,
    line: u64,
    pub register: u8,
}

impl Register {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        let register = token.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
        Self {
            token,
            column,
            line,
            register,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn column(&self) -> u64 {
        self.column
    }

    pub fn line(&self) -> u64 {
        self.line
    }
}
