use std::{collections::VecDeque, iter};

use crate::{
    listing,
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
        let reg = format!("R{}", register);

        let mut assembled = vec![listing!(
            instruction,
            *program_counter,
            self.line,
            symbol,
            "ADD",
            reg,
            reg,
            reg
        )];

        iter::repeat(instruction)
            .take(count as usize - 1)
            .map(|val| {
                *program_counter += 1;
                listing!(val, *program_counter, self.line, "", "ADD", reg, reg, reg)
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
        if let Token::Immediate(immediate) = self.operands.last().unwrap() {
            immediate.value as u16
        } else {
            unreachable!()
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        expect!(self, tokens, Immediate);

        operands_check!(self);

        tokens
    }
}
