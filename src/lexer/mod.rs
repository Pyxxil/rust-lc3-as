pub mod tokenizer;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind;

use token::TokenType;

use lexer::tokenizer::Tokenizer;

use notifier;
use notifier::{Diagnostic, DiagnosticType, NoteDiagnostic};

pub struct Lexer<'a> {
    file: &'a str,
    tokens: Vec<TokenType>,
    okay: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a str) -> Lexer<'a> {
        Lexer {
            file,
            tokens: Vec::new(),
            okay: true,
        }
    }

    pub fn lex(&mut self) {
        match File::open(self.file) {
            Ok(file) => {
                BufReader::new(file)
                    .lines()
                    .enumerate()
                    .for_each(move |(line_number, line)| {
                        Tokenizer::new(&line.unwrap(), (line_number + 1) as u64)
                            .for_each(|token| self.tokens.push(token));
                    });
            }
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                notifier::add_diagnostic(Diagnostic::Note(NoteDiagnostic::new(
                    DiagnosticType::Error,
                    0,
                    0,
                    format!("File '{}' doesn't exist", self.file).as_ref(),
                )));
            }
            Err(_) => {
                notifier::add_diagnostic(Diagnostic::Note(NoteDiagnostic::new(
                    DiagnosticType::Error,
                    0,
                    0,
                    format!("Unable to open file '{}'", self.file).as_ref(),
                )));
            }
        }
    }

    #[inline]
    pub fn tokens(self) -> Vec<TokenType> {
        self.tokens
    }

    #[inline]
    pub fn is_okay(&self) -> bool {
        self.okay && notifier::error_count() == 0
    }
}

#[cfg(test)]
mod benchmarking {
    use super::*;
    extern crate test;
    use test::Bencher;

    use token::tokens::*;
    use token::TokenType;

    #[test]
    fn test_lexer() {
        let tokens = &[
            TokenType::Orig(orig::Orig::new(String::from(".ORIG"), 3, 27)),
            TokenType::Binary(binary::Binary::new(
                String::from("0b0011000000000000"),
                9,
                27,
            )),
            TokenType::Label(label::Label::new(String::from("OUT_PROMPT"), 3, 30)),
            TokenType::Lea(lea::Lea::new(String::from("LEA"), 3, 31)),
            TokenType::Register(register::Register::new(String::from("R0"), 7, 31)),
            TokenType::Label(label::Label::new(String::from("PROMPT"), 11, 31)),
            TokenType::Puts(puts::Puts::new(String::from("PUTS"), 3, 32)),
            TokenType::And(and::And::new(String::from("AND"), 3, 35)),
            TokenType::Register(register::Register::new(String::from("R5"), 7, 35)),
            TokenType::Register(register::Register::new(String::from("R5"), 11, 35)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#0"), 15, 35)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 36)),
            TokenType::Register(register::Register::new(String::from("R5"), 7, 36)),
            TokenType::Register(register::Register::new(String::from("R5"), 11, 36)),
            TokenType::Decimal(decimal::Decimal::new(String::from("10"), 15, 36)),
            TokenType::Ld(ld::Ld::new(String::from("LD"), 3, 37)),
            TokenType::Register(register::Register::new(String::from("R1"), 6, 37)),
            TokenType::Label(label::Label::new(String::from("NUMBER"), 10, 37)),
            TokenType::Jsr(jsr::Jsr::new(String::from("JSR"), 3, 38)),
            TokenType::Label(label::Label::new(String::from("CLEAR_FLAG"), 7, 38)),
            TokenType::Label(label::Label::new(String::from("GET_INPUT"), 3, 43)),
            TokenType::Getc(getc::Getc::new(String::from("GETC"), 3, 44)),
            TokenType::Out(out::Out::new(String::from("OUT"), 3, 45)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 46)),
            TokenType::Register(register::Register::new(String::from("R3"), 7, 46)),
            TokenType::Register(register::Register::new(String::from("R0"), 11, 46)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#-10"), 15, 46)),
            TokenType::Br(br::Br::new(String::from("BRz"), 3, 47, false, true, false)),
            TokenType::Label(label::Label::new(String::from("CHECK_INPUT"), 7, 47)),
            TokenType::Ld(ld::Ld::new(String::from("LD"), 3, 49)),
            TokenType::Register(register::Register::new(String::from("R3"), 6, 49)),
            TokenType::Label(label::Label::new(String::from("ASCII_NINE"), 10, 49)),
            TokenType::Not(not::Not::new(String::from("NOT"), 3, 50)),
            TokenType::Register(register::Register::new(String::from("R3"), 7, 50)),
            TokenType::Register(register::Register::new(String::from("R3"), 11, 50)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 51)),
            TokenType::Register(register::Register::new(String::from("R3"), 7, 51)),
            TokenType::Register(register::Register::new(String::from("R3"), 11, 51)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#1"), 15, 51)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 52)),
            TokenType::Register(register::Register::new(String::from("R3"), 7, 52)),
            TokenType::Register(register::Register::new(String::from("R0"), 11, 52)),
            TokenType::Register(register::Register::new(String::from("R3"), 15, 52)),
            TokenType::Br(br::Br::new(String::from("BRp"), 3, 53, false, false, true)),
            TokenType::Label(label::Label::new(String::from("FLAG_THAT"), 7, 53)),
            TokenType::Ld(ld::Ld::new(String::from("LD"), 3, 55)),
            TokenType::Register(register::Register::new(String::from("R3"), 6, 55)),
            TokenType::Label(label::Label::new(String::from("ASCII_ZERO"), 10, 55)),
            TokenType::Not(not::Not::new(String::from("NOT"), 3, 56)),
            TokenType::Register(register::Register::new(String::from("R3"), 7, 56)),
            TokenType::Register(register::Register::new(String::from("R3"), 11, 56)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 57)),
            TokenType::Register(register::Register::new(String::from("R3"), 7, 57)),
            TokenType::Register(register::Register::new(String::from("R3"), 11, 57)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#1"), 15, 57)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 58)),
            TokenType::Register(register::Register::new(String::from("R3"), 7, 58)),
            TokenType::Register(register::Register::new(String::from("R3"), 11, 58)),
            TokenType::Register(register::Register::new(String::from("R0"), 15, 58)),
            TokenType::Br(br::Br::new(String::from("BRn"), 3, 59, true, false, false)),
            TokenType::Label(label::Label::new(String::from("FLAG_THAT"), 7, 59)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 62)),
            TokenType::Register(register::Register::new(String::from("R6"), 7, 62)),
            TokenType::Register(register::Register::new(String::from("R5"), 11, 62)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#-10"), 15, 62)),
            TokenType::Br(br::Br::new(String::from("BRn"), 3, 63, true, false, false)),
            TokenType::Label(label::Label::new(String::from("CHECK_ZERO"), 7, 63)),
            TokenType::Br(br::Br::new(String::from("BRz"), 3, 64, false, true, false)),
            TokenType::Label(label::Label::new(String::from("SET_TO"), 7, 64)),
            TokenType::Br(br::Br::new(String::from("BRnzp"), 3, 65, true, true, true)),
            TokenType::Label(label::Label::new(String::from("MULTIPLY_BY_TEN"), 9, 65)),
            TokenType::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                3,
                69,
            )),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 70)),
            TokenType::Register(register::Register::new(String::from("R5"), 7, 70)),
            TokenType::Register(register::Register::new(String::from("R5"), 11, 70)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#-1"), 15, 70)),
            TokenType::Br(br::Br::new(String::from("BRz"), 3, 71, false, true, false)),
            TokenType::Label(label::Label::new(String::from("OUT_PROMPT"), 7, 71)),
            TokenType::Br(br::Br::new(String::from("BRnzp"), 3, 72, true, true, true)),
            TokenType::Label(label::Label::new(String::from("GET_INPUT"), 9, 72)),
            TokenType::Label(label::Label::new(String::from("SET_TO"), 3, 76)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 77)),
            TokenType::Register(register::Register::new(String::from("R1"), 7, 77)),
            TokenType::Register(register::Register::new(String::from("R3"), 11, 77)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#0"), 15, 77)),
            TokenType::Br(br::Br::new(String::from("BRnzp"), 3, 78, true, true, true)),
            TokenType::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                9,
                78,
            )),
            TokenType::Label(label::Label::new(String::from("MULTIPLY_BY_TEN"), 3, 82)),
            TokenType::Jsr(jsr::Jsr::new(String::from("JSR"), 3, 83)),
            TokenType::Label(label::Label::new(String::from("CHECK_FLAG"), 7, 83)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 85)),
            TokenType::Register(register::Register::new(String::from("R4"), 7, 85)),
            TokenType::Register(register::Register::new(String::from("R1"), 11, 85)),
            TokenType::Register(register::Register::new(String::from("R1"), 15, 85)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 86)),
            TokenType::Register(register::Register::new(String::from("R6"), 7, 86)),
            TokenType::Register(register::Register::new(String::from("R4"), 11, 86)),
            TokenType::Register(register::Register::new(String::from("R4"), 15, 86)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 87)),
            TokenType::Register(register::Register::new(String::from("R6"), 7, 87)),
            TokenType::Register(register::Register::new(String::from("R6"), 11, 87)),
            TokenType::Register(register::Register::new(String::from("R6"), 15, 87)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 88)),
            TokenType::Register(register::Register::new(String::from("R1"), 7, 88)),
            TokenType::Register(register::Register::new(String::from("R6"), 11, 88)),
            TokenType::Register(register::Register::new(String::from("R4"), 15, 88)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 89)),
            TokenType::Register(register::Register::new(String::from("R1"), 7, 89)),
            TokenType::Register(register::Register::new(String::from("R1"), 11, 89)),
            TokenType::Register(register::Register::new(String::from("R3"), 15, 89)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 90)),
            TokenType::Register(register::Register::new(String::from("R4"), 7, 90)),
            TokenType::Register(register::Register::new(String::from("R1"), 11, 90)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#-16"), 15, 90)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 91)),
            TokenType::Register(register::Register::new(String::from("R4"), 7, 91)),
            TokenType::Register(register::Register::new(String::from("R4"), 11, 91)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#-7"), 15, 91)),
            TokenType::Br(br::Br::new(String::from("BRp"), 3, 92, false, false, true)),
            TokenType::Label(label::Label::new(String::from("FLAG_THAT"), 7, 92)),
            TokenType::Br(br::Br::new(String::from("BRnzp"), 3, 93, true, true, true)),
            TokenType::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                9,
                93,
            )),
            TokenType::Label(label::Label::new(String::from("CHECK_FLAG"), 3, 97)),
            TokenType::Ld(ld::Ld::new(String::from("LD"), 3, 98)),
            TokenType::Register(register::Register::new(String::from("R4"), 6, 98)),
            TokenType::Label(label::Label::new(String::from("FLAG"), 10, 98)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 99)),
            TokenType::Register(register::Register::new(String::from("R4"), 7, 99)),
            TokenType::Register(register::Register::new(String::from("R4"), 11, 99)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#-1"), 15, 99)),
            TokenType::Br(br::Br::new(String::from("BRz"), 3, 100, false, true, false)),
            TokenType::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                7,
                100,
            )),
            TokenType::Ret(ret::Ret::new(String::from("RET"), 3, 101)),
            TokenType::Label(label::Label::new(String::from("CHECK_ZERO"), 3, 106)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 107)),
            TokenType::Register(register::Register::new(String::from("R6"), 7, 107)),
            TokenType::Register(register::Register::new(String::from("R1"), 11, 107)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#-1"), 15, 107)),
            TokenType::Br(br::Br::new(String::from("BRn"), 3, 108, true, false, false)),
            TokenType::Label(label::Label::new(String::from("FLAG_THAT"), 7, 108)),
            TokenType::Br(br::Br::new(String::from("BRnzp"), 3, 109, true, true, true)),
            TokenType::Label(label::Label::new(String::from("MULTIPLY_BY_TEN"), 9, 109)),
            TokenType::Label(label::Label::new(String::from("FLAG_THAT"), 3, 113)),
            TokenType::Lea(lea::Lea::new(String::from("LEA"), 3, 114)),
            TokenType::Register(register::Register::new(String::from("R4"), 7, 114)),
            TokenType::Label(label::Label::new(String::from("FLAG"), 11, 114)),
            TokenType::And(and::And::new(String::from("AND"), 3, 115)),
            TokenType::Register(register::Register::new(String::from("R6"), 7, 115)),
            TokenType::Register(register::Register::new(String::from("R6"), 11, 115)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#0"), 15, 115)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 116)),
            TokenType::Register(register::Register::new(String::from("R6"), 7, 116)),
            TokenType::Register(register::Register::new(String::from("R6"), 11, 116)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#1"), 15, 116)),
            TokenType::Str(str::Str::new(String::from("STR"), 3, 117)),
            TokenType::Register(register::Register::new(String::from("R6"), 7, 117)),
            TokenType::Register(register::Register::new(String::from("R4"), 11, 117)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#0"), 15, 117)),
            TokenType::Br(br::Br::new(String::from("BRnzp"), 3, 118, true, true, true)),
            TokenType::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                9,
                118,
            )),
            TokenType::Label(label::Label::new(String::from("CLEAR_FLAG"), 3, 122)),
            TokenType::Lea(lea::Lea::new(String::from("LEA"), 3, 123)),
            TokenType::Register(register::Register::new(String::from("R4"), 7, 123)),
            TokenType::Label(label::Label::new(String::from("FLAG"), 11, 123)),
            TokenType::And(and::And::new(String::from("AND"), 3, 124)),
            TokenType::Register(register::Register::new(String::from("R6"), 7, 124)),
            TokenType::Register(register::Register::new(String::from("R6"), 11, 124)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#0"), 15, 124)),
            TokenType::Str(str::Str::new(String::from("STR"), 3, 125)),
            TokenType::Register(register::Register::new(String::from("R6"), 7, 125)),
            TokenType::Register(register::Register::new(String::from("R4"), 11, 125)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#0"), 15, 125)),
            TokenType::Ret(ret::Ret::new(String::from("RET"), 3, 126)),
            TokenType::Label(label::Label::new(String::from("CHECK_INPUT"), 3, 130)),
            TokenType::Ld(ld::Ld::new(String::from("LD"), 3, 131)),
            TokenType::Register(register::Register::new(String::from("R3"), 6, 131)),
            TokenType::Label(label::Label::new(String::from("FLAG"), 10, 131)),
            TokenType::Add(add::Add::new(String::from("ADD"), 3, 132)),
            TokenType::Register(register::Register::new(String::from("R3"), 7, 132)),
            TokenType::Register(register::Register::new(String::from("R3"), 11, 132)),
            TokenType::Decimal(decimal::Decimal::new(String::from("#-1"), 15, 132)),
        ];

        #[cfg(target_os = "linux")]
        let mut lexer = Lexer::new("/home/pyxxil/Sync/Projects/LC3/Examples/Fibonacci.asm");
        #[cfg(not(target_os = "linux"))]
        let mut lexer = Lexer::new("/Users/pyxxil/Sync/Projects/LC3/Examples/Fibonacci.asm");

        lexer.lex();

        //assert!(lexer.is_okay());

        let lexed = lexer.tokens();
        //assert_eq!(lexed.len(), tokens.len());

        for i in 0..tokens.len() {
            assert_eq!(lexed[i], tokens[i]);
        }
    }

    #[bench]
    fn bench_fibonacci(bench: &mut Bencher) {
        bench.iter(|| {
            #[cfg(target_os = "linux")]
            let mut lexer = Lexer::new("/home/pyxxil/Sync/Projects/LC3/Examples/Fibonacci.asm");
            #[cfg(not(target_os = "linux"))]
            let mut lexer = Lexer::new("/Users/pyxxil/Sync/Projects/LC3/Examples/Fibonacci.asm");

            lexer.lex();
            lexer.lex();

            if lexer.is_okay() {
                println!("Assembled successfully!");
            } else {
                println!("Failed to assemble");
            }
        });
    }

    #[bench]
    fn bench_features(bench: &mut Bencher) {
        bench.iter(|| {
            #[cfg(target_os = "linux")]
            let mut lexer = Lexer::new("/home/pyxxil/Sync/Projects/LC3/Examples/Features.asm");
            #[cfg(not(target_os = "linux"))]
            let mut lexer = Lexer::new("/Users/pyxxil/Sync/Projects/LC3/Examples/Features.asm");

            lexer.lex();
            lexer.lex();

            if lexer.is_okay() {
                println!("Assembled successfully!");
            } else {
                println!("Failed to assemble");
            }
        });
    }

    #[bench]
    fn bench_recursive_fibonacci(bench: &mut Bencher) {
        bench.iter(|| {
            #[cfg(target_os = "linux")]
            let mut lexer =
                Lexer::new("/home/pyxxil/Sync/Projects/LC3/Examples/Recursive_Fibonacci.asm");
            #[cfg(not(target_os = "linux"))]
            let mut lexer =
                Lexer::new("/Users/pyxxil/Sync/Projects/LC3/Examples/Recursive_Fibonacci.asm");

            lexer.lex();
            lexer.lex();

            if lexer.is_okay() {
                println!("Assembled successfully!");
            } else {
                println!("Failed to assemble");
            }
        });
    }

    #[bench]
    fn bench_compare(bench: &mut Bencher) {
        bench.iter(|| {
            #[cfg(target_os = "linux")]
            let mut lexer = Lexer::new("/home/pyxxil/Sync/Projects/LC3/Examples/Compare.asm");
            #[cfg(not(target_os = "linux"))]
            let mut lexer = Lexer::new("/Users/pyxxil/Sync/Projects/LC3/Examples/Compare.asm");

            lexer.lex();
            lexer.lex();

            if lexer.is_okay() {
                println!("Assembled successfully!");
            } else {
                println!("Failed to assemble");
            }
        });
    }
}
