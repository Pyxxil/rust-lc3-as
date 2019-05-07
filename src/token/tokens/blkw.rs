use token::tokens::traits::*;

use token::tokens::{expected, too_few_operands};

use token::Token;

use std::collections::VecDeque;

use std::iter;

token!(Blkw, 2);

impl Assemble for Blkw {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        let value = match self.operands.last().unwrap() {
            Token::Immediate(imm) => imm.value,
            Token::Label(_) => 0,
            _ => unreachable!(),
        } as u16;
        iter::repeat(value)
            .take(match self.operands.first().unwrap() {
                Token::Immediate(imm) => imm.value,
                _ => unreachable!(),
            } as usize)
            .map(|val| {
                *program_counter += 1;
                (
                    val,
                    format!(
                        "({0:4X}) {1:04X} {1:016b} ({2: >4}) .FILL #{1}",
                        *program_counter - 1,
                        val as i16,
                        self.line,
                    ),
                )
            })
            .collect()
    }
}

impl Requirements for Blkw {
    fn memory_requirement(&self) -> u16 {
        match self.operands.first().unwrap() {
            Token::Immediate(imm) => imm.value as u16,
            _ => unreachable!(),
        }
    }

    fn require_range(&self) -> (u64, u64) {
        (1, 2)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Immediate, "Immediate");
        }

        if let Some(token) = tokens.front() {
            maybe_expect!(self, tokens, token, Token::Immediate, Token::Character, Token::Label);
        }

        operands_check!(self);

        tokens
    }
}
