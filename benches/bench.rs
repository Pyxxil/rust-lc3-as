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
    // "Examples/Features.asm", // Currently broken
    "Examples/Multi_Word_Addition.asm",
    "Examples/2048.asm",
];

fn assemble(file: &str) {
    Assembler::from_file(String::from(file))
        .map(|assembler| assembler.assemble(false))
        .unwrap();
}

fn bench_assembly(c: &mut Criterion) {
    FILES.iter().for_each(|file| {
        c.bench_function(&format!("Assemble {}", file), |b| b.iter(|| assemble(file)));
    });
}

criterion_group!(benches, bench_assembly);
criterion_main!(benches);
