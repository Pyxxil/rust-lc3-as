extern crate lc3lib;
use lc3lib::assembler::Assembler;
use lc3lib::notifier;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let args = App::new("LC3AS")
        .arg(Arg::with_name("files").multiple(true).required(true))
        .arg(
            Arg::with_name("print ast")
                .help("Print the parsed tokens")
                .long("print-ast"),
        )
        .arg(
            Arg::with_name("quiet")
                .help("Don't output any errors or warnings")
                .long("quiet")
                .short("q"),
        )
        .get_matches();

    let files: Vec<&str> = args.values_of("files").unwrap().collect();
    let should_print_ast = args.is_present("print-ast");

    notifier::push(if args.is_present("quiet") {
        notifier::Stdout::Quiet
    } else {
        notifier::Stdout::Colour
    });

    files.into_iter().for_each(move |file| {
        Assembler::new(file.to_string()).assemble(should_print_ast);
        notifier::clear();
    });
}
