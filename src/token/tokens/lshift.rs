use std::{collections::VecDeque, iter};

use crate::{
    token::{
        tokens::{
            expected, too_few_operands,
            traits::{Assemble, Requirements},
        },
        Token,
    },
    types::{Listings, SymbolTable},
};

token!(Lshift, 2);

impl Assemble for Lshift {
    fn assembled(
        mut self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        let register = if let Token::Register(register) = self.operands.remove(0) {
            register.register
        } else {
            unreachable!()
        };

        let count = if let Token::Immediate(immediate) = self.operands.remove(0) {
            immediate.value as u16
        } else {
            unreachable!()
        };

        let instruction = 0x1000 | register << 9 | register << 6 | register;

        let mut assembled = vec![(
            instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} ADD R{4} R{4} R{4}",
                *program_counter, instruction, self.line, symbol, register,
            ),
        )];

        iter::repeat(instruction)
            .take(count as usize - 1)
            .map(|val| {
                *program_counter += 1;
                (
                    val,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4})                      ADD R{3} R{3} R{3}",
                        *program_counter, val as i16, self.line, register,
                    ),
                )
            })
            .for_each(|line| assembled.push(line));

        *program_counter += 1;

        assembled
    }
}

impl Requirements for Lshift {
    fn min_operands(&self) -> u64 {
        2
    }

    fn memory_requirement(&self) -> u16 {
        match self.operands.last().unwrap() {
            Token::Immediate(imm) => imm.value as u16,
            _ => unreachable!(),
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        expect!(self, tokens, Immediate);

        operands_check!(self);

        tokens
    }
}
