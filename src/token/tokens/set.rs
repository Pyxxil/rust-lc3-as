use std::collections::VecDeque;

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

token!(Set, 2);

impl Assemble for Set {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        let immediate = if let Token::Immediate(immediate) = self.operands.last().unwrap() {
            immediate.value
        } else {
            unreachable!()
        };

        let register = if let Token::Register(register) = self.operands.first().unwrap() {
            register.register
        } else {
            unreachable!()
        };

        let reg = format!("R{}", register);

        if immediate >= -16 && immediate <= 15 {
            *program_counter += 2;
            let clear_instruction = 0x5020 | register << 9 | register << 6;
            let set_instruction =
                0x1020 | register << 9 | register << 6 | (immediate as u16 & 0x1F);

            vec![
                listing!(
                    clear_instruction,
                    *program_counter - 2,
                    self.line,
                    symbol,
                    "AND",
                    reg,
                    reg,
                    reg
                ),
                listing!(
                    set_instruction,
                    *program_counter - 1,
                    self.line,
                    "",
                    "ADD",
                    register,
                    register,
                    format!("#{}", immediate)
                ),
            ]
        } else {
            *program_counter += 3;
            vec![
                listing!(
                    0x0E01,
                    *program_counter - 3,
                    self.line,
                    symbol,
                    "BRnzp",
                    "#1"
                ),
                listing!(
                    immediate as u16,
                    *program_counter - 2,
                    self.line,
                    "",
                    ".FILL",
                    format!("#{}", immediate)
                ),
                listing!(
                    0x21FE | register << 9,
                    *program_counter - 1,
                    self.line,
                    "",
                    "LD",
                    register,
                    "#-2"
                ),
            ]
        }
    }
}

impl Requirements for Set {
    fn min_operands(&self) -> u64 {
        2
    }

    fn memory_requirement(&self) -> u16 {
        if let Token::Immediate(immediate) = self.operands.last().unwrap() {
            if immediate.value > 15 || immediate.value < -16 {
                3
            } else {
                2
            }
        } else {
            unreachable!()
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        expect!(self, tokens, Register, Immediate, Label);

        operands_check!(self);

        tokens
    }
}
