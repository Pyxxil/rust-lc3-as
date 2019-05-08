#![feature(const_fn)]
#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;

pub mod assembler;
pub mod lexer;
pub mod notifier;
pub mod parser;
#[macro_use]
pub mod token;
