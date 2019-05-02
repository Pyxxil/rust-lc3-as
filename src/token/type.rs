use token::tokens::traits::*;
use token::tokens::*;

use std::fmt;

pub use std::collections::VecDeque;

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
    Immediate(immediate::Immediate),
    Character(character::Character),
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

macro_rules! fmt {
    ( $self:expr, $f:expr, $( $token_type:path ),* ) => {
        match $self {
            $( $token_type(ref token) => write!($f, "{:#?}", token), )+
            _ => Ok(())
        }
    };
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt!(
            *self,
            f,
            Token::Add,
            Token::And,
            Token::Br,
            Token::Jmp,
            Token::Jsr,
            Token::Jsrr,
            Token::Ld,
            Token::Ldi,
            Token::Ldr,
            Token::Lea,
            Token::Not,
            Token::Ret,
            Token::Rti,
            Token::St,
            Token::Sti,
            Token::Str,
            Token::Trap,
            Token::Getc,
            Token::Halt,
            Token::In,
            Token::Out,
            Token::Puts,
            Token::Putsp,
            Token::Blkw,
            Token::Fill,
            Token::Include,
            Token::Lshift,
            Token::Orig,
            Token::Neg,
            Token::Set,
            Token::Stringz,
            Token::Sub,
            Token::Immediate,
            Token::Label,
            Token::Character,
            Token::String,
            Token::End,
            Token::Register
        )
    }
}

macro_rules! memory{
        ( $self:expr, $( $token:path ),* ) => {
            match *$self {
                $( $token(ref token) => token.memory_requirement(), )+
                _ => 0,
            }
        }
    }

macro_rules! consume {
        ( $self:expr, $tokens:expr, $( $token_type:path ),*, 0, $( $fail_token:path ),* ) => {
            match $self {
                $( $token_type(ref mut token) => token.consume($tokens), )+
                $( $fail_token(ref token) => {
                    expected(
                        &["Instruction", "Directive", "Label"],
                        $self,
                        (token.column(), token.line(), token.token().len()),
                    );
                    $tokens
                }
                )+
                _ => $tokens,
            }
        };
    }

impl Requirements for Token {
    fn require_range(&self) -> (u64, u64) {
        (0, 0)
    }

    fn memory_requirement(&self) -> u16 {
        0
        //memory!(
        //    self,
        //    Token::Add,
        //    Token::And,
        //    Token::Br,
        //    Token::Jmp,
        //    Token::Jsr,
        //    Token::Jsrr,
        //    Token::Ld,
        //    Token::Ldi,
        //    Token::Ldr,
        //    Token::Lea,
        //    Token::Not,
        //    Token::Ret,
        //    Token::Rti,
        //    Token::St,
        //    Token::Sti,
        //    Token::Str,
        //    Token::Trap,
        //    Token::Getc,
        //    Token::Halt,
        //    Token::In,
        //    Token::Out,
        //    Token::Puts,
        //    Token::Putsp,
        //    Token::Blkw,
        //    Token::Fill,
        //    Token::Include,
        //    Token::Lshift,
        //    Token::Orig,
        //    Token::Neg,
        //    Token::Set,
        //    Token::Stringz,
        //    Token::Sub,
        //)
    }

    fn consume(&mut self, tokens: VecDeque<Token>) -> VecDeque<Token> {
        consume!(
            self,
            tokens,
            Token::Add,
            Token::And,
            Token::Br,
            Token::Jmp,
            Token::Jsr,
            Token::Jsrr,
            Token::Ld,
            Token::Ldi,
            Token::Ldr,
            Token::Lea,
            Token::Not,
            Token::Ret,
            Token::Rti,
            Token::St,
            Token::Sti,
            Token::Str,
            Token::Trap,
            Token::Getc,
            Token::Halt,
            Token::In,
            Token::Out,
            Token::Puts,
            Token::Putsp,
            Token::Blkw,
            Token::Fill,
            Token::Include,
            Token::Lshift,
            Token::Orig,
            Token::Neg,
            Token::Set,
            Token::Stringz,
            Token::Sub,
            0, // Just a way of delimiting between the two types of tokens (consumable, and not)
            Token::Immediate,
            Token::Character,
            Token::String,
            Token::Register
        )
    }
}

macro_rules! assembled {
    ( $self:expr, $program_counter:expr, $( $token_type:path ),* ) => {
        match $self {
            $( $token_type(token) => token.assembled($program_counter), )+
            _ => Vec::new(),
        }
    };
}

impl Assemble for Token {
    fn assembled(self, program_counter: &mut i16) -> Vec<(u16, String)> {
        assembled!(
            self,
            program_counter,
            Token::Add,
            Token::And,
            Token::Br,
            Token::Jmp,
            Token::Jsr,
            Token::Jsrr,
            Token::Ld,
            Token::Ldi,
            Token::Ldr,
            Token::Lea,
            Token::Not,
            Token::Ret,
            Token::Rti,
            Token::St,
            Token::Sti,
            Token::Str,
            Token::Trap,
            Token::Getc,
            Token::Halt,
            Token::In,
            Token::Out,
            Token::Puts,
            Token::Putsp,
            Token::Blkw,
            Token::Fill,
            Token::Include,
            Token::Lshift,
            Token::Orig,
            Token::Neg,
            Token::Set,
            Token::Stringz,
            Token::Sub
        )
    }
}
