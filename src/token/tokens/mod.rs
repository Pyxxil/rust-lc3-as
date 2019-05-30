use notifier;
use notifier::{DiagType, Diagnostic, Highlight};
use token::r#type::Token;

#[macro_use]
pub mod macros;
pub mod traits;

pub fn expected(file: &str, expect: &[&str], found: &Token, at: (u64, u64, usize)) {
    notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
        DiagType::Error,
        (*file).to_string(),
        at.0,
        at.1,
        at.2,
        format!(
            "Expected to find argument of type {}, but found\n{:#?}",
            expect.to_vec().join(", "),
            found
        ),
    )));
}

pub fn too_few_operands(file: &str, required: u64, found: u64, token: &str, at: (u64, u64, usize)) {
    notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
        DiagType::Error,
        (*file).to_string(),
        at.0,
        at.1,
        at.2,
        format!(
            "{} expects {} operand{}, but {}{} {} found",
            token,
            required,
            if required == 1 { "" } else { "s" },
            if found == 0 { "" } else { "only " },
            found,
            if found == 1 { "was" } else { "were" }
        ),
    )));
}

// Instructions
pub mod add;
pub mod and;
pub mod br;
pub mod jmp;
pub mod jmpt;
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
