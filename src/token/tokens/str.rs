use token::tokens::traits::*;

use token::tokens::*;

use token::Token;

use std::collections::VecDeque;

token!(Str, 3);

impl Assemble for Str {
    fn assembled(mut self, program_counter: &mut i16) -> Vec<(u16, String)> {
        *program_counter += 1;

        let destination_register = match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let source_one = match self.operands.remove(0) {
            Token::Register(register) => register.register,
            _ => unreachable!(),
        };

        let source_two = match self.operands.remove(0) {
            Token::Immediate(imm) => imm.value & 0x3F,
            _ => unreachable!(),
        } as u16;

        let instruction: u16 = 0x7000 | destination_register << 9 | source_one << 6 | source_two;

        vec![(
            instruction,
            format!(
                "({0:4X}) {1:04X} {1:016b} ({2: >4}) ADD R{3} R{4} #{5}",
                *program_counter - 1,
                instruction,
                self.line,
                destination_register,
                source_one,
                (source_two << 10) as i16 >> 10,
            ),
        )]
    }
}

impl Requirements for Str {
    fn memory_requirement(&self) -> u16 {
        1
    }

    fn require_range(&self) -> (u64, u64) {
        (3, 3)
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Register, "Register");
        }

        if let Some(token) = tokens.front() {
            expect!(self, tokens, token, Token::Immediate, "Immediate");
        }

        operands_check!(self);

        tokens
    }
}
