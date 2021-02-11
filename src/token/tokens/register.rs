use std::convert::TryInto;

use crate::token::{tokens::traits::Requirements, Token};

token!(Register, 0, register: u16);

impl Register {
    #[must_use]
    pub fn from_str(token: String, file: String, column: u64, line: u64) -> Self {
        let register = token.chars().nth(1).unwrap().to_digit(10).unwrap();

        Self::new(token, file, column, line, register.try_into().unwrap())
    }
}

impl Requirements for Register {}
