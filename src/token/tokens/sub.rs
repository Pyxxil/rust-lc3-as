use std::collections::VecDeque;

use crate::{
    listing,
    token::tokens::{
        expected, too_few_operands,
        traits::{Assemble, Requirements},
        Token,
    },
    types::{Listings, SymbolTable},
};

token!(Sub, 3);

impl Assemble for Sub {
    fn assembled(
        self,
        program_counter: &mut i16,
        _symbols: &SymbolTable,
        symbol: &str,
    ) -> Listings {
        *program_counter += 1;

        let destination_register = if let Token::Register(register) = self.operands.first().unwrap()
        {
            register.register
        } else {
            unreachable!()
        };

        let source_register_one = if self.operands.len() > 2 {
            if let Token::Register(register) = &self.operands[1] {
                register.register
            } else {
                unreachable!()
            }
        } else {
            destination_register
        };

        let source_register_two = if let Token::Register(register) = self.operands.last().unwrap() {
            register.register
        } else {
            unreachable!()
        };

        if source_register_one == source_register_two {
            let instruction = 0x5000 | destination_register << 9 | source_register_one << 6 | 0x20;
            vec![listing!(
                instruction,
                *program_counter - 1,
                self.line,
                symbol,
                "AND",
                format!("R{}", destination_register),
                format!("R{}", source_register_one),
                "#0"
            )]
        } else {
            *program_counter += 2;
            let not_instruction = 0x903F | source_register_two << 9 | source_register_two << 6;
            let add_instruction = 0x1021 | source_register_two << 9 | source_register_two << 6;
            let subtract_instruction =
                0x1000 | destination_register << 9 | source_register_one << 6 | source_register_two;

            let source_two = format!("R{}", source_register_two);

            let mut assembled = vec![
                listing!(
                    not_instruction,
                    *program_counter - 3,
                    self.line,
                    symbol,
                    "NOT",
                    source_two,
                    source_two
                ),
                listing!(
                    add_instruction,
                    *program_counter - 2,
                    self.line,
                    "",
                    "ADD",
                    source_two,
                    source_two,
                    "#1"
                ),
                listing!(
                    subtract_instruction,
                    *program_counter - 1,
                    self.line,
                    "",
                    "ADD",
                    format!("R{}", destination_register),
                    format!("R{}", source_register_one),
                    source_two
                ),
            ];

            if destination_register != source_register_two {
                *program_counter += 2;
                assembled.push(listing!(
                    not_instruction,
                    *program_counter - 2,
                    self.line,
                    "",
                    "NOT",
                    source_two,
                    source_two
                ));
                assembled.push(listing!(
                    add_instruction,
                    *program_counter - 1,
                    self.line,
                    "",
                    "ADD",
                    source_two,
                    source_two,
                    "#1"
                ));
            }

            assembled
        }
    }
}

impl Requirements for Sub {
    fn min_operands(&self) -> u64 {
        2
    }

    fn memory_requirement(&self) -> u16 {
        let destination_register = if let Token::Register(register) = self.operands.first().unwrap()
        {
            register.register
        } else {
            unreachable!()
        };

        let source_register_one = if self.operands.len() > 2 {
            if let Token::Register(ref register) = &self.operands[1] {
                register.register
            } else {
                unreachable!()
            }
        } else {
            destination_register
        };

        let source_register_two = if let Token::Register(register) = self.operands.last().unwrap() {
            register.register
        } else {
            unreachable!()
        };

        if source_register_one == source_register_two {
            1
        } else if destination_register == source_register_two {
            3
        } else {
            5
        }
    }

    fn consume(&mut self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
        expect!(self, tokens, Register);

        expect!(self, tokens, Register);

        maybe_expect!(self, tokens, Register);

        operands_check!(self);

        tokens
    }
}
