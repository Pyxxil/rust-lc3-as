// Instructions
pub mod add;
pub mod and;
pub mod br;
pub mod jmp;
pub mod jsr;
pub mod jsrr;
pub mod ld;
pub mod ldi;
pub mod ldr;
pub mod lea;
pub mod not;
pub mod ret;
pub mod rti;
pub mod st;
pub mod sti;
pub mod str;
pub mod trap;

// Traps
pub mod getc;
pub mod halt;
pub mod r#in;
pub mod out;
pub mod puts;
pub mod putsp;

// Types
pub mod character;
pub mod immediate;
pub mod label;
pub mod register;
pub mod string;

// Directives
pub mod blkw;
pub mod end;
pub mod fill;
pub mod include;
pub mod lshift;
pub mod neg;
pub mod orig;
pub mod set;
pub mod stringz;
pub mod sub;

pub mod traits;
