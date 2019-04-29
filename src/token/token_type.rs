use token::tokens::traits::*;
use token::tokens::{_in, getc, halt, out, puts, putsp};
use token::tokens::{
    add, and, br, jmp, jsr, jsrr, ld, ldi, ldr, lea, not, ret, rti, st, sti, str, trap,
};
use token::tokens::{binary, character, decimal, hexadecimal, label, register, string};
use token::tokens::{blkw, end, fill, include, lshift, neg, orig, set, stringz, sub};

use std::fmt;

#[derive(PartialEq, Clone)]
pub enum TokenType {
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
    In(_in::In),
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

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::Add(ref token) => write!(f, "{:#?}", token),
            TokenType::And(ref token) => write!(f, "{:#?}", token),
            TokenType::Br(ref token) => write!(f, "{:#?}", token),
            TokenType::Jmp(ref token) => write!(f, "{:#?}", token),
            TokenType::Jsr(ref token) => write!(f, "{:#?}", token),
            TokenType::Jsrr(ref token) => write!(f, "{:#?}", token),
            TokenType::Ld(ref token) => write!(f, "{:#?}", token),
            TokenType::Ldi(ref token) => write!(f, "{:#?}", token),
            TokenType::Ldr(ref token) => write!(f, "{:#?}", token),
            TokenType::Lea(ref token) => write!(f, "{:#?}", token),
            TokenType::Not(ref token) => write!(f, "{:#?}", token),
            TokenType::Ret(ref token) => write!(f, "{:#?}", token),
            TokenType::Rti(ref token) => write!(f, "{:#?}", token),
            TokenType::St(ref token) => write!(f, "{:#?}", token),
            TokenType::Sti(ref token) => write!(f, "{:#?}", token),
            TokenType::Str(ref token) => write!(f, "{:#?}", token),
            TokenType::Trap(ref token) => write!(f, "{:#?}", token),
            TokenType::Getc(ref token) => write!(f, "{:#?}", token),
            TokenType::Halt(ref token) => write!(f, "{:#?}", token),
            TokenType::In(ref token) => write!(f, "{:#?}", token),
            TokenType::Out(ref token) => write!(f, "{:#?}", token),
            TokenType::Puts(ref token) => write!(f, "{:#?}", token),
            TokenType::Putsp(ref token) => write!(f, "{:#?}", token),
            TokenType::Binary(ref token) => write!(f, "{:#?}", token),
            TokenType::Character(ref token) => write!(f, "{:#?}", token),
            TokenType::Decimal(ref token) => write!(f, "{:#?}", token),
            TokenType::Hexadecimal(ref token) => write!(f, "{:#?}", token),
            TokenType::Label(ref token) => write!(f, "{:#?}", token),
            TokenType::Register(ref token) => write!(f, "{:#?}", token),
            TokenType::String(ref token) => write!(f, "{:#?}", token),
            TokenType::Blkw(ref token) => write!(f, "{:#?}", token),
            TokenType::End(ref token) => write!(f, "{:#?}", token),
            TokenType::Fill(ref token) => write!(f, "{:#?}", token),
            TokenType::Include(ref token) => write!(f, "{:#?}", token),
            TokenType::Lshift(ref token) => write!(f, "{:#?}", token),
            TokenType::Orig(ref token) => write!(f, "{:#?}", token),
            TokenType::Neg(ref token) => write!(f, "{:#?}", token),
            TokenType::Set(ref token) => write!(f, "{:#?}", token),
            TokenType::Stringz(ref token) => write!(f, "{:#?}", token),
            TokenType::Sub(ref token) => write!(f, "{:#?}", token),
            TokenType::EOL => write!(f, "EOL"),
        }
    }
}

impl Requirements for TokenType {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn is_satisfied(&self) -> bool {
        false
    }

    fn consume(&mut self, tokens: Vec<TokenType>) -> Vec<TokenType> {
        match *self {
            TokenType::Add(ref mut token) => token.consume(tokens),
            TokenType::And(ref mut token) => token.consume(tokens),
            TokenType::Br(ref mut token) => token.consume(tokens),
            TokenType::Jmp(ref mut token) => token.consume(tokens),
            TokenType::Jsr(ref mut token) => token.consume(tokens),
            TokenType::Jsrr(ref mut token) => token.consume(tokens),
            TokenType::Ld(ref mut token) => token.consume(tokens),
            TokenType::Ldi(ref mut token) => token.consume(tokens),
            TokenType::Ldr(ref mut token) => token.consume(tokens),
            TokenType::Lea(ref mut token) => token.consume(tokens),
            TokenType::Not(ref mut token) => token.consume(tokens),
            TokenType::Ret(ref mut token) => token.consume(tokens),
            TokenType::Rti(ref mut token) => token.consume(tokens),
            TokenType::St(ref mut token) => token.consume(tokens),
            TokenType::Sti(ref mut token) => token.consume(tokens),
            TokenType::Str(ref mut token) => token.consume(tokens),
            TokenType::Trap(ref mut token) => token.consume(tokens),
            TokenType::Getc(ref mut token) => token.consume(tokens),
            TokenType::Halt(ref mut token) => token.consume(tokens),
            TokenType::In(ref mut token) => token.consume(tokens),
            TokenType::Out(ref mut token) => token.consume(tokens),
            TokenType::Puts(ref mut token) => token.consume(tokens),
            TokenType::Putsp(ref mut token) => token.consume(tokens),
            TokenType::Binary(ref mut token) => token.consume(tokens),
            TokenType::Character(ref mut token) => token.consume(tokens),
            TokenType::Decimal(ref mut token) => token.consume(tokens),
            TokenType::Hexadecimal(ref mut token) => token.consume(tokens),
            TokenType::Label(ref mut token) => token.consume(tokens),
            TokenType::Register(ref mut token) => token.consume(tokens),
            TokenType::String(ref mut token) => token.consume(tokens),
            TokenType::Blkw(ref mut token) => token.consume(tokens),
            TokenType::End(ref mut token) => token.consume(tokens),
            TokenType::Fill(ref mut token) => token.consume(tokens),
            TokenType::Include(ref mut token) => token.consume(tokens),
            TokenType::Lshift(ref mut token) => token.consume(tokens),
            TokenType::Orig(ref mut token) => token.consume(tokens),
            TokenType::Neg(ref mut token) => token.consume(tokens),
            TokenType::Set(ref mut token) => token.consume(tokens),
            TokenType::Stringz(ref mut token) => token.consume(tokens),
            TokenType::Sub(ref mut token) => token.consume(tokens),
            TokenType::EOL => tokens,
        }
    }
}

impl Assemble for TokenType {
    fn assemble(&mut self) {
        match *self {
            TokenType::Add(ref mut token) => token.assemble(),
            TokenType::And(ref mut token) => token.assemble(),
            TokenType::Br(ref mut token) => token.assemble(),
            TokenType::Jmp(ref mut token) => token.assemble(),
            TokenType::Jsr(ref mut token) => token.assemble(),
            TokenType::Jsrr(ref mut token) => token.assemble(),
            TokenType::Ld(ref mut token) => token.assemble(),
            TokenType::Ldi(ref mut token) => token.assemble(),
            TokenType::Ldr(ref mut token) => token.assemble(),
            TokenType::Lea(ref mut token) => token.assemble(),
            TokenType::Not(ref mut token) => token.assemble(),
            TokenType::Ret(ref mut token) => token.assemble(),
            TokenType::Rti(ref mut token) => token.assemble(),
            TokenType::St(ref mut token) => token.assemble(),
            TokenType::Sti(ref mut token) => token.assemble(),
            TokenType::Str(ref mut token) => token.assemble(),
            TokenType::Trap(ref mut token) => token.assemble(),
            TokenType::Getc(ref mut token) => token.assemble(),
            TokenType::Halt(ref mut token) => token.assemble(),
            TokenType::In(ref mut token) => token.assemble(),
            TokenType::Out(ref mut token) => token.assemble(),
            TokenType::Puts(ref mut token) => token.assemble(),
            TokenType::Putsp(ref mut token) => token.assemble(),
            TokenType::Binary(ref mut token) => token.assemble(),
            TokenType::Character(ref mut token) => token.assemble(),
            TokenType::Decimal(ref mut token) => token.assemble(),
            TokenType::Hexadecimal(ref mut token) => token.assemble(),
            TokenType::Label(ref mut token) => token.assemble(),
            TokenType::Register(ref mut token) => token.assemble(),
            TokenType::String(ref mut token) => token.assemble(),
            TokenType::Blkw(ref mut token) => token.assemble(),
            TokenType::End(ref mut token) => token.assemble(),
            TokenType::Fill(ref mut token) => token.assemble(),
            TokenType::Include(ref mut token) => token.assemble(),
            TokenType::Lshift(ref mut token) => token.assemble(),
            TokenType::Orig(ref mut token) => token.assemble(),
            TokenType::Neg(ref mut token) => token.assemble(),
            TokenType::Set(ref mut token) => token.assemble(),
            TokenType::Stringz(ref mut token) => token.assemble(),
            TokenType::Sub(ref mut token) => token.assemble(),
            TokenType::EOL => {}
        }
    }

    fn assembled(self) -> Vec<(u16, String)> {
        match self {
            TokenType::Add(token) => token.assembled(),
            TokenType::And(token) => token.assembled(),
            TokenType::Br(token) => token.assembled(),
            TokenType::Jmp(token) => token.assembled(),
            TokenType::Jsr(token) => token.assembled(),
            TokenType::Jsrr(token) => token.assembled(),
            TokenType::Ld(token) => token.assembled(),
            TokenType::Ldi(token) => token.assembled(),
            TokenType::Ldr(token) => token.assembled(),
            TokenType::Lea(token) => token.assembled(),
            TokenType::Not(token) => token.assembled(),
            TokenType::Ret(token) => token.assembled(),
            TokenType::Rti(token) => token.assembled(),
            TokenType::St(token) => token.assembled(),
            TokenType::Sti(token) => token.assembled(),
            TokenType::Str(token) => token.assembled(),
            TokenType::Trap(token) => token.assembled(),
            TokenType::Getc(token) => token.assembled(),
            TokenType::Halt(token) => token.assembled(),
            TokenType::In(token) => token.assembled(),
            TokenType::Out(token) => token.assembled(),
            TokenType::Puts(token) => token.assembled(),
            TokenType::Putsp(token) => token.assembled(),
            TokenType::Binary(token) => token.assembled(),
            TokenType::Character(token) => token.assembled(),
            TokenType::Decimal(token) => token.assembled(),
            TokenType::Hexadecimal(token) => token.assembled(),
            TokenType::Label(token) => token.assembled(),
            TokenType::Register(token) => token.assembled(),
            TokenType::String(token) => token.assembled(),
            TokenType::Blkw(token) => token.assembled(),
            TokenType::End(token) => token.assembled(),
            TokenType::Fill(token) => token.assembled(),
            TokenType::Include(token) => token.assembled(),
            TokenType::Lshift(token) => token.assembled(),
            TokenType::Orig(token) => token.assembled(),
            TokenType::Neg(token) => token.assembled(),
            TokenType::Set(token) => token.assembled(),
            TokenType::Stringz(token) => token.assembled(),
            TokenType::Sub(token) => token.assembled(),
            TokenType::EOL => Vec::new(),
        }
    }
}
