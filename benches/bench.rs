#![feature(test)]

#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate lc3lib;
use lc3lib::assembler::Assembler;

const FILES: &[&str] = &[
    "Examples/Fibonacci.asm",
    "Examples/Recursive_Fibonacci.asm",
    "Examples/input.asm",
    "Examples/Compare.asm",
    "Examples/Features.asm",
    "Examples/Multi_Word_Addition.asm",
];

fn lex(file: &str) {
    Assembler::from_file(String::from(file))
        .map(|assembler| assembler.lex())
        .unwrap();
}

fn assemble(file: &str) {
    Assembler::from_file(String::from(file))
        .map(|assembler| assembler.assemble(false))
        .unwrap();
}

fn bench_lexer(c: &mut Criterion) {
    FILES.iter().for_each(|file| {
        c.bench_function(&format!("Lex {}", file), |b| b.iter(|| lex(file)));
    });
}

fn bench_lexer_and_parser(c: &mut Criterion) {
    FILES.iter().for_each(|file| {
        c.bench_function(&format!("Assemble {}", file), |b| b.iter(|| assemble(file)));
    });
}

criterion_group!(benches, bench_lexer, bench_lexer_and_parser);
criterion_main!(benches);
