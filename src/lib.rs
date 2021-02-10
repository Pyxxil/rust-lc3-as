#![feature(test)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]

#[macro_use]
extern crate lazy_static;

pub mod assembler;
pub mod lexer;
pub mod notifier;
pub mod parser;
pub mod types;
pub mod writer;

#[macro_use]
pub mod token;
