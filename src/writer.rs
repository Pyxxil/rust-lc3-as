use std::{
    fs::{File, OpenOptions},
    io::{Error, Write},
};

use crate::{token::Symbol, types::Listing, types::Program};

#[derive(Default)]
pub struct Writer<W: Write> {
    outputs: Vec<(Format, W)>,
}

pub enum Format {
    Binary,
    Hex,
    Listing,
    Object,
    SymbolTable,
}

impl Format {
    /// Write the symbol table header to the writer
    fn write_header<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        if let Format::SymbolTable = self {
            writeln!(
                out,
                "{: <20} Assembler\n-------------------- -------",
                "Symbol"
            )?
        };

        Ok(())
    }

    /// Write a symbol to the symbol table if that's the specified format
    fn write_symbol<W: Write>(&self, out: &mut W, symbol: &Symbol) -> Result<(), Error> {
        if let Format::SymbolTable = self {
            writeln!(out, "{: <20} {:04X}", symbol.symbol(), symbol.address())?
        };

        Ok(())
    }

    /// Write the listing to the writer in the specified format
    fn write_listing<W: Write>(&self, out: &mut W, listing: &Listing) -> Result<(), Error> {
        let (binary, listing) = listing;

        match self {
            Format::Binary => writeln!(out, "{:016b}", binary)?,
            Format::Hex => writeln!(out, "{:04X}", binary)?,
            Format::Listing => writeln!(out, "{}", listing)?,
            Format::Object => {
                out.write_all(&[(binary >> 8 & 0xFF) as u8, (binary & 0xFF) as u8])?
            }
            _ => (),
        };

        Ok(())
    }
}

impl<W: Write> Writer<W> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
        }
    }

    /// Register a writer for a format, e.g. provide a file that will
    /// have the binary representation of the program written to it
    pub fn register(mut self, ty: Format, writer: W) -> Self {
        self.outputs.push((ty, writer));
        self
    }

    /// Write the program to each of the registered writers in their respective
    /// formats
    ///
    /// # Errors
    ///   Fails if there is an error writing to the writer
    pub fn write(mut self, program: Program) -> Result<(), Error> {
        let (symbols, listings) = program;

        self.outputs
            .iter_mut()
            .try_for_each(|(format, writer)| format.write_header(writer))?;

        symbols.into_iter().try_for_each(|(_, symbol)| {
            self.outputs
                .iter_mut()
                .try_for_each(|(format, writer)| format.write_symbol(writer, &symbol))
        })?;

        listings.into_iter().try_for_each(|listing| {
            self.outputs
                .iter_mut()
                .try_for_each(|(format, writer)| format.write_listing(writer, &listing))
        })?;

        Ok(())
    }
}

impl Writer<File> {
    /// Register all of the formats with a specific base file
    /// This is currently only used by the assembler
    #[must_use]
    pub fn register_all(mut self, file: &str) -> Self {
        let base_file_name: String = file
            .chars()
            .take(file.rfind(|c| c == '.').unwrap())
            .collect();

        let bin_file = base_file_name.clone() + ".bin";
        let hex_file = base_file_name.clone() + ".hex";
        let lst_file = base_file_name.clone() + ".lst";
        let obj_file = base_file_name.clone() + ".obj";
        let sym_file = base_file_name + ".sym";

        let sym_f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(sym_file)
            .expect("Can't create symbol file");

        let bin_f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(bin_file)
            .expect("Can't create bin file");

        let hex_f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(hex_file)
            .expect("Can't create hex file");

        let lst_f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(lst_file)
            .expect("Can't create listing file");

        let obj_f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(obj_file)
            .expect("Can't create object file");

        self.outputs.push((Format::Binary, bin_f));
        self.outputs.push((Format::Hex, hex_f));
        self.outputs.push((Format::Listing, lst_f));
        self.outputs.push((Format::Object, obj_f));
        self.outputs.push((Format::SymbolTable, sym_f));

        self
    }
}
