extern crate colored;
use self::colored::*;

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
            "{}:{}{} {}",
            match self.diagnostic_type {
                DiagType::Note => "Note".bright_white(),
                DiagType::Warning => "Warning".yellow(),
                DiagType::Error => "Error".red(),
            },
            if self.line > 0 {
                format!(" Line {}:", self.line)
            } else {
                String::new()
            },
            if self.column > 0 {
                format!(" Column {}:", self.column)
            } else {
                String::new()
            },
            self.context
        )
    }
}

impl NoColour for Note {
    fn no_colour(&self) -> String {
        format!(
            "{:#?}:{}{} {}",
            self.diagnostic_type,
            if self.line > 0 {
                format!(" Line {}:", self.line)
            } else {
                String::new()
            },
            if self.column > 0 {
                format!(" Column {}:", self.column)
            } else {
                String::new()
            },
            self.context
        )
    }
}

pub struct Pointer {
    diagnostic_type: DiagType,
    column: u64,
    line: u64,
    context: String,
}

impl Pointer {
    pub fn new(diagnostic_type: DiagType, column: u64, line: u64, context: String) -> Self {
        Self {
            diagnostic_type,
            column,
            line,
            context,
        }
    }
}

impl Colour for Pointer {
    fn colour(&self) -> String {
        format!(
            "{}: Line {}: Column {}: {}",
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

impl NoColour for Pointer {
    #[inline]
    fn no_colour(&self) -> String {
        format!(
            "{:#?}: Line {}: Column {}: {}",
            self.diagnostic_type, self.line, self.column, self.context
        )
    }
}

pub struct Highlight {
    diagnostic_type: DiagType,
    column: u64,
    line: u64,
    width: usize,
    context: String,
}

impl Highlight {
    pub fn new(
        diagnostic_type: DiagType,
        column: u64,
        line: u64,
        width: usize,
        context: String,
    ) -> Self {
        Self {
            diagnostic_type,
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
            "{}: Line {}: Column {}: {}",
            match self.diagnostic_type {
                DiagType::Note => "Note".bright_white(),
                DiagType::Warning => "Warning".yellow(),
                DiagType::Error => "Error".red(),
            },
            self.line,
            self.column,
            self.context,
        )
    }
}

impl NoColour for Highlight {
    #[inline]
    fn no_colour(&self) -> String {
        format!(
            "{:#?}: {}: {}: {}",
            self.diagnostic_type, self.line, self.column, self.context
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
