use token::tokens::traits::*;
use token::tokens::{
    add, and, br, jmp, jsr, jsrr, ld, ldi, ldr, lea, not, ret, rti, st, sti, str, trap,
};
use token::tokens::{binary, character, decimal, hexadecimal, label, register, string};
use token::tokens::{blkw, end, fill, include, lshift, neg, orig, set, stringz, sub};
use token::tokens::{getc, halt, out, puts, putsp, r#in};

use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Token {
    // Instructions
    Add(add::Add),
    And(and::And),
    Br(br::Br),
    Jmp(jmp::Jmp),
    Jsr(jsr::Jsr),
    Jsrr(jsrr::Jsrr),
    Ld(ld::Ld),
    Ldi(ldi::Ldi),
    Ldr(ldr::Ldr),
    Lea(lea::Lea),
    Not(not::Not),
    Ret(ret::Ret),
    Rti(rti::Rti),
    St(st::St),
    Sti(sti::Sti),
    Str(str::Str),
    Trap(trap::Trap),

    // Traps
    Getc(getc::Getc),
    Halt(halt::Halt),
    In(r#in::In),
    Out(out::Out),
    Puts(puts::Puts),
    Putsp(putsp::Putsp),

    // Types
    Binary(binary::Binary),
    Character(character::Character),
    Decimal(decimal::Decimal),
    Hexadecimal(hexadecimal::Hexadecimal),
    Label(label::Label),
    Register(register::Register),
    String(string::String),

    // Directives
    Blkw(blkw::Blkw),
    End(end::End),
    Fill(fill::Fill),
    Include(include::Include),
    Lshift(lshift::Lshift),
    Orig(orig::Orig),
    Neg(neg::Neg),
    Set(set::Set),
    Stringz(stringz::Stringz),
    Sub(sub::Sub),

    EOL,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Add(ref token) => write!(f, "{:#?}", token),
            Token::And(ref token) => write!(f, "{:#?}", token),
            Token::Br(ref token) => write!(f, "{:#?}", token),
            Token::Jmp(ref token) => write!(f, "{:#?}", token),
            Token::Jsr(ref token) => write!(f, "{:#?}", token),
            Token::Jsrr(ref token) => write!(f, "{:#?}", token),
            Token::Ld(ref token) => write!(f, "{:#?}", token),
            Token::Ldi(ref token) => write!(f, "{:#?}", token),
            Token::Ldr(ref token) => write!(f, "{:#?}", token),
            Token::Lea(ref token) => write!(f, "{:#?}", token),
            Token::Not(ref token) => write!(f, "{:#?}", token),
            Token::Ret(ref token) => write!(f, "{:#?}", token),
            Token::Rti(ref token) => write!(f, "{:#?}", token),
            Token::St(ref token) => write!(f, "{:#?}", token),
            Token::Sti(ref token) => write!(f, "{:#?}", token),
            Token::Str(ref token) => write!(f, "{:#?}", token),
            Token::Trap(ref token) => write!(f, "{:#?}", token),
            Token::Getc(ref token) => write!(f, "{:#?}", token),
            Token::Halt(ref token) => write!(f, "{:#?}", token),
            Token::In(ref token) => write!(f, "{:#?}", token),
            Token::Out(ref token) => write!(f, "{:#?}", token),
            Token::Puts(ref token) => write!(f, "{:#?}", token),
            Token::Putsp(ref token) => write!(f, "{:#?}", token),
            Token::Binary(ref token) => write!(f, "{:#?}", token),
            Token::Character(ref token) => write!(f, "{:#?}", token),
            Token::Decimal(ref token) => write!(f, "{:#?}", token),
            Token::Hexadecimal(ref token) => write!(f, "{:#?}", token),
            Token::Label(ref token) => write!(f, "{:#?}", token),
            Token::Register(ref token) => write!(f, "{:#?}", token),
            Token::String(ref token) => write!(f, "{:#?}", token),
            Token::Blkw(ref token) => write!(f, "{:#?}", token),
            Token::End(ref token) => write!(f, "{:#?}", token),
            Token::Fill(ref token) => write!(f, "{:#?}", token),
            Token::Include(ref token) => write!(f, "{:#?}", token),
            Token::Lshift(ref token) => write!(f, "{:#?}", token),
            Token::Orig(ref token) => write!(f, "{:#?}", token),
            Token::Neg(ref token) => write!(f, "{:#?}", token),
            Token::Set(ref token) => write!(f, "{:#?}", token),
            Token::Stringz(ref token) => write!(f, "{:#?}", token),
            Token::Sub(ref token) => write!(f, "{:#?}", token),
            Token::EOL => write!(f, "EOL"),
        }
    }
}

impl Requirements for Token {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, tokens: Vec<Token>) -> Vec<Token> {
        match *self {
            Token::Add(ref mut token) => token.consume(tokens),
            Token::And(ref mut token) => token.consume(tokens),
            Token::Br(ref mut token) => token.consume(tokens),
            Token::Jmp(ref mut token) => token.consume(tokens),
            Token::Jsr(ref mut token) => token.consume(tokens),
            Token::Jsrr(ref mut token) => token.consume(tokens),
            Token::Ld(ref mut token) => token.consume(tokens),
            Token::Ldi(ref mut token) => token.consume(tokens),
            Token::Ldr(ref mut token) => token.consume(tokens),
            Token::Lea(ref mut token) => token.consume(tokens),
            Token::Not(ref mut token) => token.consume(tokens),
            Token::Ret(ref mut token) => token.consume(tokens),
            Token::Rti(ref mut token) => token.consume(tokens),
            Token::St(ref mut token) => token.consume(tokens),
            Token::Sti(ref mut token) => token.consume(tokens),
            Token::Str(ref mut token) => token.consume(tokens),
            Token::Trap(ref mut token) => token.consume(tokens),
            Token::Getc(ref mut token) => token.consume(tokens),
            Token::Halt(ref mut token) => token.consume(tokens),
            Token::In(ref mut token) => token.consume(tokens),
            Token::Out(ref mut token) => token.consume(tokens),
            Token::Puts(ref mut token) => token.consume(tokens),
            Token::Putsp(ref mut token) => token.consume(tokens),
            Token::Binary(ref mut token) => token.consume(tokens),
            Token::Character(ref mut token) => token.consume(tokens),
            Token::Decimal(ref mut token) => token.consume(tokens),
            Token::Hexadecimal(ref mut token) => token.consume(tokens),
            Token::Label(ref mut token) => token.consume(tokens),
            Token::Register(ref mut token) => token.consume(tokens),
            Token::String(ref mut token) => token.consume(tokens),
            Token::Blkw(ref mut token) => token.consume(tokens),
            Token::End(ref mut token) => token.consume(tokens),
            Token::Fill(ref mut token) => token.consume(tokens),
            Token::Include(ref mut token) => token.consume(tokens),
            Token::Lshift(ref mut token) => token.consume(tokens),
            Token::Orig(ref mut token) => token.consume(tokens),
            Token::Neg(ref mut token) => token.consume(tokens),
            Token::Set(ref mut token) => token.consume(tokens),
            Token::Stringz(ref mut token) => token.consume(tokens),
            Token::Sub(ref mut token) => token.consume(tokens),
            Token::EOL => tokens,
        }
    }
}

impl Assemble for Token {
    fn assemble(&mut self) {
        match *self {
            Token::Add(ref mut token) => token.assemble(),
            Token::And(ref mut token) => token.assemble(),
            Token::Br(ref mut token) => token.assemble(),
            Token::Jmp(ref mut token) => token.assemble(),
            Token::Jsr(ref mut token) => token.assemble(),
            Token::Jsrr(ref mut token) => token.assemble(),
            Token::Ld(ref mut token) => token.assemble(),
            Token::Ldi(ref mut token) => token.assemble(),
            Token::Ldr(ref mut token) => token.assemble(),
            Token::Lea(ref mut token) => token.assemble(),
            Token::Not(ref mut token) => token.assemble(),
            Token::Ret(ref mut token) => token.assemble(),
            Token::Rti(ref mut token) => token.assemble(),
            Token::St(ref mut token) => token.assemble(),
            Token::Sti(ref mut token) => token.assemble(),
            Token::Str(ref mut token) => token.assemble(),
            Token::Trap(ref mut token) => token.assemble(),
            Token::Getc(ref mut token) => token.assemble(),
            Token::Halt(ref mut token) => token.assemble(),
            Token::In(ref mut token) => token.assemble(),
            Token::Out(ref mut token) => token.assemble(),
            Token::Puts(ref mut token) => token.assemble(),
            Token::Putsp(ref mut token) => token.assemble(),
            Token::Binary(ref mut token) => token.assemble(),
            Token::Character(ref mut token) => token.assemble(),
            Token::Decimal(ref mut token) => token.assemble(),
            Token::Hexadecimal(ref mut token) => token.assemble(),
            Token::Label(ref mut token) => token.assemble(),
            Token::Register(ref mut token) => token.assemble(),
            Token::String(ref mut token) => token.assemble(),
            Token::Blkw(ref mut token) => token.assemble(),
            Token::End(ref mut token) => token.assemble(),
            Token::Fill(ref mut token) => token.assemble(),
            Token::Include(ref mut token) => token.assemble(),
            Token::Lshift(ref mut token) => token.assemble(),
            Token::Orig(ref mut token) => token.assemble(),
            Token::Neg(ref mut token) => token.assemble(),
            Token::Set(ref mut token) => token.assemble(),
            Token::Stringz(ref mut token) => token.assemble(),
            Token::Sub(ref mut token) => token.assemble(),
            Token::EOL => {}
        }
    }

    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        match self {
            Token::Add(token) => token.assembled(program_counter),
            Token::And(token) => token.assembled(program_counter),
            Token::Br(token) => token.assembled(program_counter),
            Token::Jmp(token) => token.assembled(program_counter),
            Token::Jsr(token) => token.assembled(program_counter),
            Token::Jsrr(token) => token.assembled(program_counter),
            Token::Ld(token) => token.assembled(program_counter),
            Token::Ldi(token) => token.assembled(program_counter),
            Token::Ldr(token) => token.assembled(program_counter),
            Token::Lea(token) => token.assembled(program_counter),
            Token::Not(token) => token.assembled(program_counter),
            Token::Ret(token) => token.assembled(program_counter),
            Token::Rti(token) => token.assembled(program_counter),
            Token::St(token) => token.assembled(program_counter),
            Token::Sti(token) => token.assembled(program_counter),
            Token::Str(token) => token.assembled(program_counter),
            Token::Trap(token) => token.assembled(program_counter),
            Token::Getc(token) => token.assembled(program_counter),
            Token::Halt(token) => token.assembled(program_counter),
            Token::In(token) => token.assembled(program_counter),
            Token::Out(token) => token.assembled(program_counter),
            Token::Puts(token) => token.assembled(program_counter),
            Token::Putsp(token) => token.assembled(program_counter),
            Token::Binary(token) => token.assembled(program_counter),
            Token::Character(token) => token.assembled(program_counter),
            Token::Decimal(token) => token.assembled(program_counter),
            Token::Hexadecimal(token) => token.assembled(program_counter),
            Token::Label(token) => token.assembled(program_counter),
            Token::Register(token) => token.assembled(program_counter),
            Token::String(token) => token.assembled(program_counter),
            Token::Blkw(token) => token.assembled(program_counter),
            Token::End(token) => token.assembled(program_counter),
            Token::Fill(token) => token.assembled(program_counter),
            Token::Include(token) => token.assembled(program_counter),
            Token::Lshift(token) => token.assembled(program_counter),
            Token::Orig(token) => token.assembled(program_counter),
            Token::Neg(token) => token.assembled(program_counter),
            Token::Set(token) => token.assembled(program_counter),
            Token::Stringz(token) => token.assembled(program_counter),
            Token::Sub(token) => token.assembled(program_counter),
            Token::EOL => Vec::new(),
        }
    }
}
