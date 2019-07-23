use std::collections::{HashMap, VecDeque};

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::{Symbol, Token};

token!(Br, 1, n: bool, z: bool, p: bool);

impl Assemble for Br {
    fn assembled(
        self,
        program_counter: &mut i16,
        symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Vec<(u16, String)> {
        *program_counter += 1;

        let value = match self.operands.first().unwrap() {
            Token::Immediate(imm) => imm.value,
            Token::Label(label) => {
                if let Some(symbol) = symbols.get(label.token()) {
                    symbol.address() as i16 - *program_counter
                } else {
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
    fn require_range(&self) -> (u64, u64) {
        (1, 1)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(
                self,
                tokens,
                token,
                Token::Label,
                "Label",
                Token::Immediate,
                "Immediate"
            );
        }

        operands_check!(self);

        tokens
    }
}
