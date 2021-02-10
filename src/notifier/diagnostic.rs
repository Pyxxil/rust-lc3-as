extern crate colored;

use assembler::get_line;

use self::colored::Colorize;

pub trait Colour {
    fn colour(&self) -> String;
}

pub trait NoColour {
    fn no_colour(&self) -> String;
}

#[derive(PartialEq, Debug)]
pub enum DiagType {
    Note,
    Warning,
    Error,
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
    fn colour(&self) -> String {
        format!(
            "{}:{}:{}: {}",
            match self.diagnostic_type {
                DiagType::Note => "Note".bright_white(),
                DiagType::Warning => "Warning".yellow(),
                DiagType::Error => "Error".red(),
            },
            self.line,
            self.column,
            self.context
        )
    }
}

impl NoColour for Note {
    fn no_colour(&self) -> String {
        format!(
            "{}:{}:{}: {}",
            match self.diagnostic_type {
                DiagType::Note => "Note",
                DiagType::Warning => "Warning",
                DiagType::Error => "Error",
            },
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
    fn colour(&self) -> String {
        format!(
            "{}:{}:{}: {}: {}\n{}\n{}",
            match self.diagnostic_type {
                DiagType::Note => "Note".bright_white(),
                DiagType::Warning => "Warning".yellow(),
                DiagType::Error => "Error".red(),
            },
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
    fn no_colour(&self) -> String {
        format!(
            "{}:{}:{}: {}: {}\n{}\n{}",
            self.file,
            self.line,
            self.column,
            match self.diagnostic_type {
                DiagType::Note => "Note",
                DiagType::Warning => "Warning",
                DiagType::Error => "Error",
            },
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
    fn colour(&self) -> String {
        format!(
            "{}:{}:{}: {}: {}\n{}\n{}",
            self.file,
            self.line,
            self.column,
            match self.diagnostic_type {
                DiagType::Note => "Note".bright_white(),
                DiagType::Warning => "Warning".yellow(),
                DiagType::Error => "Error".red(),
            },
            self.context,
            get_line(&self.file, self.line),
            " ".repeat(self.column as usize - 1) + &"~".repeat(self.width)
        )
    }
}

impl NoColour for Highlight {
    fn no_colour(&self) -> String {
        format!(
            "{}:{}:{}: {}: {}\n{}\n{}",
            self.file,
            self.line,
            self.column,
            match self.diagnostic_type {
                DiagType::Note => "Note",
                DiagType::Warning => "Warning",
                DiagType::Error => "Error",
            },
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