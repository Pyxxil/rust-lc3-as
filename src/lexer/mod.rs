pub mod tokenizer;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind;

use token::Token;

use lexer::tokenizer::Tokenizer;

use notifier;
use notifier::{DiagType, Diagnostic, Note};

pub struct Lexer<'a> {
    file: &'a str,
    tokens: Vec<Token>,
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
                notifier::add_diagnostic(Diagnostic::Note(Note::new(
                    DiagType::Error,
                    0,
                    0,
                    format!("File '{}' doesn't exist", self.file),
                )));
            }
            Err(_) => {
                notifier::add_diagnostic(Diagnostic::Note(Note::new(
                    DiagType::Error,
                    0,
                    0,
                    format!("Unable to open file '{}'", self.file),
                )));
            }
        }
    }

    #[inline]
    pub fn tokens(self) -> Vec<Token> {
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
    use token::Token;

    #[test]
    fn test_lexer() {
        let tokens = &[
            Token::Orig(orig::Orig::new(String::from(".ORIG"), 3, 27)),
            Token::Binary(binary::Binary::new(
                String::from("0b0011000000000000"),
                9,
                27,
            )),
            Token::Label(label::Label::new(String::from("OUT_PROMPT"), 3, 30)),
            Token::Lea(lea::Lea::new(String::from("LEA"), 3, 31)),
            Token::Register(register::Register::new(String::from("R0"), 7, 31)),
            Token::Label(label::Label::new(String::from("PROMPT"), 11, 31)),
            Token::Puts(puts::Puts::new(String::from("PUTS"), 3, 32)),
            Token::And(and::And::new(String::from("AND"), 3, 35)),
            Token::Register(register::Register::new(String::from("R5"), 7, 35)),
            Token::Register(register::Register::new(String::from("R5"), 11, 35)),
            Token::Decimal(decimal::Decimal::new(String::from("#0"), 15, 35)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 36)),
            Token::Register(register::Register::new(String::from("R5"), 7, 36)),
            Token::Register(register::Register::new(String::from("R5"), 11, 36)),
            Token::Decimal(decimal::Decimal::new(String::from("10"), 15, 36)),
            Token::Ld(ld::Ld::new(String::from("LD"), 3, 37)),
            Token::Register(register::Register::new(String::from("R1"), 6, 37)),
            Token::Label(label::Label::new(String::from("NUMBER"), 10, 37)),
            Token::Jsr(jsr::Jsr::new(String::from("JSR"), 3, 38)),
            Token::Label(label::Label::new(String::from("CLEAR_FLAG"), 7, 38)),
            Token::Label(label::Label::new(String::from("GET_INPUT"), 3, 43)),
            Token::Getc(getc::Getc::new(String::from("GETC"), 3, 44)),
            Token::Out(out::Out::new(String::from("OUT"), 3, 45)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 46)),
            Token::Register(register::Register::new(String::from("R3"), 7, 46)),
            Token::Register(register::Register::new(String::from("R0"), 11, 46)),
            Token::Decimal(decimal::Decimal::new(String::from("#-10"), 15, 46)),
            Token::Br(br::Br::new(String::from("BRz"), 3, 47, false, true, false)),
            Token::Label(label::Label::new(String::from("CHECK_INPUT"), 7, 47)),
            Token::Ld(ld::Ld::new(String::from("LD"), 3, 49)),
            Token::Register(register::Register::new(String::from("R3"), 6, 49)),
            Token::Label(label::Label::new(String::from("ASCII_NINE"), 10, 49)),
            Token::Not(not::Not::new(String::from("NOT"), 3, 50)),
            Token::Register(register::Register::new(String::from("R3"), 7, 50)),
            Token::Register(register::Register::new(String::from("R3"), 11, 50)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 51)),
            Token::Register(register::Register::new(String::from("R3"), 7, 51)),
            Token::Register(register::Register::new(String::from("R3"), 11, 51)),
            Token::Decimal(decimal::Decimal::new(String::from("#1"), 15, 51)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 52)),
            Token::Register(register::Register::new(String::from("R3"), 7, 52)),
            Token::Register(register::Register::new(String::from("R0"), 11, 52)),
            Token::Register(register::Register::new(String::from("R3"), 15, 52)),
            Token::Br(br::Br::new(String::from("BRp"), 3, 53, false, false, true)),
            Token::Label(label::Label::new(String::from("FLAG_THAT"), 7, 53)),
            Token::Ld(ld::Ld::new(String::from("LD"), 3, 55)),
            Token::Register(register::Register::new(String::from("R3"), 6, 55)),
            Token::Label(label::Label::new(String::from("ASCII_ZERO"), 10, 55)),
            Token::Not(not::Not::new(String::from("NOT"), 3, 56)),
            Token::Register(register::Register::new(String::from("R3"), 7, 56)),
            Token::Register(register::Register::new(String::from("R3"), 11, 56)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 57)),
            Token::Register(register::Register::new(String::from("R3"), 7, 57)),
            Token::Register(register::Register::new(String::from("R3"), 11, 57)),
            Token::Decimal(decimal::Decimal::new(String::from("#1"), 15, 57)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 58)),
            Token::Register(register::Register::new(String::from("R3"), 7, 58)),
            Token::Register(register::Register::new(String::from("R3"), 11, 58)),
            Token::Register(register::Register::new(String::from("R0"), 15, 58)),
            Token::Br(br::Br::new(String::from("BRn"), 3, 59, true, false, false)),
            Token::Label(label::Label::new(String::from("FLAG_THAT"), 7, 59)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 62)),
            Token::Register(register::Register::new(String::from("R6"), 7, 62)),
            Token::Register(register::Register::new(String::from("R5"), 11, 62)),
            Token::Decimal(decimal::Decimal::new(String::from("#-10"), 15, 62)),
            Token::Br(br::Br::new(String::from("BRn"), 3, 63, true, false, false)),
            Token::Label(label::Label::new(String::from("CHECK_ZERO"), 7, 63)),
            Token::Br(br::Br::new(String::from("BRz"), 3, 64, false, true, false)),
            Token::Label(label::Label::new(String::from("SET_TO"), 7, 64)),
            Token::Br(br::Br::new(String::from("BRnzp"), 3, 65, true, true, true)),
            Token::Label(label::Label::new(String::from("MULTIPLY_BY_TEN"), 9, 65)),
            Token::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                3,
                69,
            )),
            Token::Add(add::Add::new(String::from("ADD"), 3, 70)),
            Token::Register(register::Register::new(String::from("R5"), 7, 70)),
            Token::Register(register::Register::new(String::from("R5"), 11, 70)),
            Token::Decimal(decimal::Decimal::new(String::from("#-1"), 15, 70)),
            Token::Br(br::Br::new(String::from("BRz"), 3, 71, false, true, false)),
            Token::Label(label::Label::new(String::from("OUT_PROMPT"), 7, 71)),
            Token::Br(br::Br::new(String::from("BRnzp"), 3, 72, true, true, true)),
            Token::Label(label::Label::new(String::from("GET_INPUT"), 9, 72)),
            Token::Label(label::Label::new(String::from("SET_TO"), 3, 76)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 77)),
            Token::Register(register::Register::new(String::from("R1"), 7, 77)),
            Token::Register(register::Register::new(String::from("R3"), 11, 77)),
            Token::Decimal(decimal::Decimal::new(String::from("#0"), 15, 77)),
            Token::Br(br::Br::new(String::from("BRnzp"), 3, 78, true, true, true)),
            Token::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                9,
                78,
            )),
            Token::Label(label::Label::new(String::from("MULTIPLY_BY_TEN"), 3, 82)),
            Token::Jsr(jsr::Jsr::new(String::from("JSR"), 3, 83)),
            Token::Label(label::Label::new(String::from("CHECK_FLAG"), 7, 83)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 85)),
            Token::Register(register::Register::new(String::from("R4"), 7, 85)),
            Token::Register(register::Register::new(String::from("R1"), 11, 85)),
            Token::Register(register::Register::new(String::from("R1"), 15, 85)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 86)),
            Token::Register(register::Register::new(String::from("R6"), 7, 86)),
            Token::Register(register::Register::new(String::from("R4"), 11, 86)),
            Token::Register(register::Register::new(String::from("R4"), 15, 86)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 87)),
            Token::Register(register::Register::new(String::from("R6"), 7, 87)),
            Token::Register(register::Register::new(String::from("R6"), 11, 87)),
            Token::Register(register::Register::new(String::from("R6"), 15, 87)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 88)),
            Token::Register(register::Register::new(String::from("R1"), 7, 88)),
            Token::Register(register::Register::new(String::from("R6"), 11, 88)),
            Token::Register(register::Register::new(String::from("R4"), 15, 88)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 89)),
            Token::Register(register::Register::new(String::from("R1"), 7, 89)),
            Token::Register(register::Register::new(String::from("R1"), 11, 89)),
            Token::Register(register::Register::new(String::from("R3"), 15, 89)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 90)),
            Token::Register(register::Register::new(String::from("R4"), 7, 90)),
            Token::Register(register::Register::new(String::from("R1"), 11, 90)),
            Token::Decimal(decimal::Decimal::new(String::from("#-16"), 15, 90)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 91)),
            Token::Register(register::Register::new(String::from("R4"), 7, 91)),
            Token::Register(register::Register::new(String::from("R4"), 11, 91)),
            Token::Decimal(decimal::Decimal::new(String::from("#-7"), 15, 91)),
            Token::Br(br::Br::new(String::from("BRp"), 3, 92, false, false, true)),
            Token::Label(label::Label::new(String::from("FLAG_THAT"), 7, 92)),
            Token::Br(br::Br::new(String::from("BRnzp"), 3, 93, true, true, true)),
            Token::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                9,
                93,
            )),
            Token::Label(label::Label::new(String::from("CHECK_FLAG"), 3, 97)),
            Token::Ld(ld::Ld::new(String::from("LD"), 3, 98)),
            Token::Register(register::Register::new(String::from("R4"), 6, 98)),
            Token::Label(label::Label::new(String::from("FLAG"), 10, 98)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 99)),
            Token::Register(register::Register::new(String::from("R4"), 7, 99)),
            Token::Register(register::Register::new(String::from("R4"), 11, 99)),
            Token::Decimal(decimal::Decimal::new(String::from("#-1"), 15, 99)),
            Token::Br(br::Br::new(String::from("BRz"), 3, 100, false, true, false)),
            Token::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                7,
                100,
            )),
            Token::Ret(ret::Ret::new(String::from("RET"), 3, 101)),
            Token::Label(label::Label::new(String::from("CHECK_ZERO"), 3, 106)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 107)),
            Token::Register(register::Register::new(String::from("R6"), 7, 107)),
            Token::Register(register::Register::new(String::from("R1"), 11, 107)),
            Token::Decimal(decimal::Decimal::new(String::from("#-1"), 15, 107)),
            Token::Br(br::Br::new(String::from("BRn"), 3, 108, true, false, false)),
            Token::Label(label::Label::new(String::from("FLAG_THAT"), 7, 108)),
            Token::Br(br::Br::new(String::from("BRnzp"), 3, 109, true, true, true)),
            Token::Label(label::Label::new(String::from("MULTIPLY_BY_TEN"), 9, 109)),
            Token::Label(label::Label::new(String::from("FLAG_THAT"), 3, 113)),
            Token::Lea(lea::Lea::new(String::from("LEA"), 3, 114)),
            Token::Register(register::Register::new(String::from("R4"), 7, 114)),
            Token::Label(label::Label::new(String::from("FLAG"), 11, 114)),
            Token::And(and::And::new(String::from("AND"), 3, 115)),
            Token::Register(register::Register::new(String::from("R6"), 7, 115)),
            Token::Register(register::Register::new(String::from("R6"), 11, 115)),
            Token::Decimal(decimal::Decimal::new(String::from("#0"), 15, 115)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 116)),
            Token::Register(register::Register::new(String::from("R6"), 7, 116)),
            Token::Register(register::Register::new(String::from("R6"), 11, 116)),
            Token::Decimal(decimal::Decimal::new(String::from("#1"), 15, 116)),
            Token::Str(str::Str::new(String::from("STR"), 3, 117)),
            Token::Register(register::Register::new(String::from("R6"), 7, 117)),
            Token::Register(register::Register::new(String::from("R4"), 11, 117)),
            Token::Decimal(decimal::Decimal::new(String::from("#0"), 15, 117)),
            Token::Br(br::Br::new(String::from("BRnzp"), 3, 118, true, true, true)),
            Token::Label(label::Label::new(
                String::from("DECREMENT_INPUT_COUNTER"),
                9,
                118,
            )),
            Token::Label(label::Label::new(String::from("CLEAR_FLAG"), 3, 122)),
            Token::Lea(lea::Lea::new(String::from("LEA"), 3, 123)),
            Token::Register(register::Register::new(String::from("R4"), 7, 123)),
            Token::Label(label::Label::new(String::from("FLAG"), 11, 123)),
            Token::And(and::And::new(String::from("AND"), 3, 124)),
            Token::Register(register::Register::new(String::from("R6"), 7, 124)),
            Token::Register(register::Register::new(String::from("R6"), 11, 124)),
            Token::Decimal(decimal::Decimal::new(String::from("#0"), 15, 124)),
            Token::Str(str::Str::new(String::from("STR"), 3, 125)),
            Token::Register(register::Register::new(String::from("R6"), 7, 125)),
            Token::Register(register::Register::new(String::from("R4"), 11, 125)),
            Token::Decimal(decimal::Decimal::new(String::from("#0"), 15, 125)),
            Token::Ret(ret::Ret::new(String::from("RET"), 3, 126)),
            Token::Label(label::Label::new(String::from("CHECK_INPUT"), 3, 130)),
            Token::Ld(ld::Ld::new(String::from("LD"), 3, 131)),
            Token::Register(register::Register::new(String::from("R3"), 6, 131)),
            Token::Label(label::Label::new(String::from("FLAG"), 10, 131)),
            Token::Add(add::Add::new(String::from("ADD"), 3, 132)),
            Token::Register(register::Register::new(String::from("R3"), 7, 132)),
            Token::Register(register::Register::new(String::from("R3"), 11, 132)),
            Token::Decimal(decimal::Decimal::new(String::from("#-1"), 15, 132)),
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
