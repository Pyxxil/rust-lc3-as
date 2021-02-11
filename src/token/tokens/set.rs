use std::collections::VecDeque;

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

        if immediate >= -16 && immediate <= 15 {
            *program_counter += 2;
            let clear_instruction = 0x5020 | register << 9 | register << 6;
            let set_instruction =
                0x1020 | register << 9 | register << 6 | (immediate as u16 & 0x1F);

            vec![
                (
                    clear_instruction,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} AND R{4} R{4} #0",
                        *program_counter - 2,
                        clear_instruction,
                        self.line,
                        symbol,
                        register
                    ),
                ),
                (
                    set_instruction,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4})                      ADD R{3} R{3} #{4}",
                        *program_counter - 1,
                        set_instruction,
                        self.line,
                        register,
                        immediate
                    ),
                ),
            ]
        } else {
            *program_counter += 3;
            vec![
                (
                    0x0E01,
                    format!(
                        "({0:04X}) 0E01 0000111000000001 ({1: >4}) {2: <20} BRnzp #1",
                        *program_counter - 3,
                        self.line,
                        symbol
                    ),
                ),
                (
                    immediate as u16,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4})                      .FILL #{1}",
                        *program_counter - 2,
                        immediate as i16,
                        self.line
                    ),
                ),
                (
                    0x21FE,
                    format!(
                        "({0:04X}) {1:04X} {1:016b} ({2: >4})                      LD R{3} #-2",
                        *program_counter - 1,
                        0x21FE | register << 9,
                        self.line,
                        register,
                    ),
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
