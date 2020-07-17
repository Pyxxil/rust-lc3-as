extern crate clap;
extern crate lc3lib;

use clap::{App, Arg};

use lc3lib::assembler::Assembler;
use lc3lib::notifier;

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

    notifier::register(
        String::from("lc3as"),
        if args.is_present("quiet") {
            notifier::Notifier::Standard(notifier::Stdout::Quiet)
        } else {
            notifier::Notifier::Standard(notifier::Stdout::Colour)
        },
    );

    files.into_iter().for_each(move |file| {
        Assembler::from_file(file.to_string())
            .and_then(|assembler| {
                Ok(assembler
                    .assemble(should_print_ast)
                    .and_then(|(assembler, symbols, assembled)| {
                        Some(assembler.write(symbols, assembled))
                    })
                    .and_then(|_| Some(println!("Assembly successful")))
                    .or_else(|| Some(println!("Assembly failed for {}", file))))
            })
            .expect("There was a problem with the file");

        // Clear all notifications
        notifier::clear(None);
    });
}
