use std::collections::{HashMap, VecDeque};

use token::tokens::traits::*;
use token::tokens::{expected, too_few_operands};
use token::{Symbol, Token};

token!(Jsr, 1);

impl Assemble for Jsr {
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

        let instruction = 0x4800 | value & 0x7FF;

        vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} JSR {4}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                match self.operands.first().unwrap() {
                    Token::Immediate(imm) => format!("#{}", imm.value),
                    Token::Label(label) => label.token().to_string(),
                    _ => unreachable!(),
                }
            ),
        )]
    }
}

impl Requirements for Jsr {
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
