use std::collections::VecDeque;

use crate::{
    notifier::{self, DiagType, Diagnostic, Highlight},
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Token,
    },
    types::{Listings, SymbolTable},
};

token!(Br, 1, n: bool, z: bool, p: bool);

impl Br {
    #[must_use]
    pub fn from_str(token: String, file: String, column: u64, line: u64) -> Self {
        let (n, z, p) = if token.len() == 2 {
            (true, true, true)
        } else {
            (
                token.contains('N') || token.contains('n'),
                token.contains('Z') || token.contains('z'),
                token.contains('P') || token.contains('p'),
            )
        };

        Self::new(token, file, column, line, n, z, p)
    }
}

impl Assemble for Br {
    fn assembled(self, program_counter: &mut i16, symbols: &SymbolTable, symbol: &str) -> Listings {
        *program_counter += 1;

        let value = match self.operands.first().unwrap() {
            Token::Immediate(imm) => imm.value,
            Token::Label(label) => {
                if let Some(symbol) = symbols.get(label.token()) {
                    symbol.address() as i16 - *program_counter
                } else {
                    undefined!(label);
                    0
                }
            }
            _ => unreachable!(),
        } as u16;

        let instruction =
            (self.n as u16) << 11 | (self.z as u16) << 10 | (self.p as u16) << 9 | value & 0x1FF;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} BR{4}{5}{6} {7}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                if self.n { "n" } else { "" },
                if self.z { "z" } else { "" },
                if self.p { "p" } else { "" },
                match self.operands.first().unwrap() {
                    Token::Immediate(imm) => format!("#{}", imm.value),
                    Token::Label(label) => label.token().to_string(),
                    _ => unreachable!(),
                }
            ),
        )]
    }
}

impl Requirements for Br {
    fn min_operands(&self) -> u64 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Label, Immediate);

        operands_check!(self);

        tokens
    }
}
