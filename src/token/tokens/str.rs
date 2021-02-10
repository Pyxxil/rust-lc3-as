use std::collections::{HashMap, VecDeque};

use crate::{
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Symbol, Token,
    },
    types::Listings,
};

token!(Str, 3);

impl Assemble for Str {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        _symbols: &HashMap<String, Symbol>,
        symbol: &str,
    ) -> Listings {
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
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} STR R{4} R{5} #{6}",
                *program_counter - 1,
                instruction,
                self.line,
                symbol,
                destination_register,
                source_one,
                (source_two << 10) as i16 >> 10,
            ),
        )]
    }
}

impl Requirements for Str {
    fn require_range(&self) -> (u64, u64) {
        (3, 3)
    }

    fn memory_requirement(&self) -> u16 {
        1
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Token::Register, "Register");

        expect!(self, tokens, Token::Register, "Register");

        expect!(self, tokens, Token::Immediate, "Immediate");

        operands_check!(self);

        tokens
    }
}
