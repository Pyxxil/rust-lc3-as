extern crate colored;
use self::colored::Colorize;

use assembler::get_line;

pub trait Colour {
    fn coloured(&self) -> String;
}

pub trait NoColour {
    fn uncoloured(&self) -> String;
}

#[derive(PartialEq)]
pub enum DiagType {
    Note,
    Warning,
    Error,
}

impl ToString for DiagType {
    fn to_string(&self) -> String {
        match self {
            Self::Note => String::from("Note"),
            Self::Warning => String::from("Warning"),
            Self::Error => String::from("Error"),
        }
    }
}

impl Colour for DiagType {
    fn coloured(&self) -> String {
        (match self {
            DiagType::Note => self.to_string().bright_white(),
            DiagType::Warning => self.to_string().yellow(),
            DiagType::Error => self.to_string().red(),
        })
        .to_string()
    }
}

pub trait Type {
    fn diagnostic_type(&self) -> &DiagType;
}

pub struct Note {
    diagnostic_type: DiagType,
    column: u64,
    line: u64,
    context: String,
}

impl Note {
    #[must_use]
    pub fn new(diagnostic_type: DiagType, column: u64, line: u64, context: String) -> Self {
        Self {
            diagnostic_type,
            column,
            line,
            context,
        }
    }
}

impl Colour for Note {
    fn coloured(&self) -> String {
        format!(
            "{}:{}:{}: {}",
            self.diagnostic_type.coloured(),
            self.line,
            self.column,
            self.context
        )
    }
}

impl NoColour for Note {
    fn uncoloured(&self) -> String {
        format!(
            "{}:{}:{}: {}",
            self.diagnostic_type.to_string(),
            self.line,
            self.column,
            self.context
        )
    }
}

pub struct Pointer {
    diagnostic_type: DiagType,
    file: String,
    column: u64,
    line: u64,
    context: String,
}

impl Pointer {
    #[must_use]
    pub fn new(
        diagnostic_type: DiagType,
        file: String,
        column: u64,
        line: u64,
        context: String,
    ) -> Self {
        Self {
            diagnostic_type,
            file,
            column,
            line,
            context,
        }
    }
}

impl Colour for Pointer {
    fn coloured(&self) -> String {
        format!(
            "{}:{}:{}: {}: {}\n{}\n{}",
            self.diagnostic_type.coloured(),
            self.file,
            self.line,
            self.column,
            self.context,
            get_line(&self.file, self.line),
            " ".repeat(self.column as usize - 1) + "^"
        )
    }
}

impl NoColour for Pointer {
    fn uncoloured(&self) -> String {
        format!(
            "{}:{}:{}: {}: {}\n{}\n{}",
            self.file,
            self.line,
            self.column,
            self.diagnostic_type.to_string(),
            self.context,
            get_line(&self.file, self.line),
            " ".repeat(self.column as usize - 1) + "^"
        )
    }
}

pub struct Highlight {
    diagnostic_type: DiagType,
    file: String,
    column: u64,
    line: u64,
    width: usize,
    context: String,
}

impl Highlight {
    #[must_use]
    pub fn new(
        diagnostic_type: DiagType,
        file: String,
        column: u64,
        line: u64,
        width: usize,
        context: String,
    ) -> Self {
        Self {
            diagnostic_type,
            file,
            column,
            line,
            width,
            context,
        }
    }
}

impl Colour for Highlight {
    fn coloured(&self) -> String {
        format!(
            "{}:{}:{}: {}: {}\n{}\n{}",
            self.file,
            self.line,
            self.column,
            self.diagnostic_type.coloured(),
            self.context,
            get_line(&self.file, self.line),
            " ".repeat(self.column as usize - 1) + &"~".repeat(self.width)
        )
    }
}

impl NoColour for Highlight {
    fn uncoloured(&self) -> String {
        format!(
            "{}:{}:{}: {}: {}\n{}\n{}",
            self.file,
            self.line,
            self.column,
            self.diagnostic_type.to_string(),
            self.context,
            get_line(&self.file, self.line),
            " ".repeat(self.column as usize - 1) + &"~".repeat(self.width)
        )
    }
}

pub enum Diagnostic {
    Note(Note),
    Pointer(Pointer),
    Highlight(Highlight),
}

impl Type for Diagnostic {
    fn diagnostic_type(&self) -> &DiagType {
        match *self {
            Diagnostic::Note(ref diagnostic) => &diagnostic.diagnostic_type,
            Diagnostic::Pointer(ref diagnostic) => &diagnostic.diagnostic_type,
            Diagnostic::Highlight(ref diagnostic) => &diagnostic.diagnostic_type,
        }
    }
}
