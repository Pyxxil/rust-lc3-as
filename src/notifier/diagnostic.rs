extern crate colored;
use self::colored::*;

pub trait Colour {
    fn colour(&self) -> String;
}

pub trait NoColour {
    fn no_colour(&self) -> String;
}

#[derive(PartialEq, Debug)]
pub enum DiagnosticType {
    Note,
    Warning,
    Error,
}

pub trait Type {
    fn diagnostic_type(&self) -> &DiagnosticType;
}

pub struct NoteDiagnostic {
    diagnostic_type: DiagnosticType,
    column: usize,
    line: usize,
    context: String,
}

impl NoteDiagnostic {
    pub fn new(
        diagnostic_type: DiagnosticType,
        column: usize,
        line: usize,
        context: &str,
    ) -> NoteDiagnostic {
        NoteDiagnostic {
            diagnostic_type,
            column,
            line,
            context: context.to_string(),
        }
    }
}

impl Colour for NoteDiagnostic {
    fn colour(&self) -> String {
        format!(
            "{}:{}{} {}",
            match self.diagnostic_type {
                DiagnosticType::Note => "Note".bright_white(),
                DiagnosticType::Warning => "Warning".yellow(),
                DiagnosticType::Error => "Error".red(),
            },
            if self.line > 0 {
                format!(" Line {}:", self.line)
            } else {
                "".to_owned()
            },
            if self.column > 0 {
                format!(" Column {}:", self.column)
            } else {
                "".to_owned()
            },
            self.context
        )
    }
}

impl NoColour for NoteDiagnostic {
    fn no_colour(&self) -> String {
        format!(
            "{:#?}:{}{} {}",
            self.diagnostic_type,
            if self.line > 0 {
                format!(" Line {}:", self.line)
            } else {
                "".to_owned()
            },
            if self.column > 0 {
                format!(" Column {}:", self.column)
            } else {
                "".to_owned()
            },
            self.context
        )
    }
}

pub struct PointerDiagnostic {
    diagnostic_type: DiagnosticType,
    column: usize,
    line: usize,
    context: String,
}

impl PointerDiagnostic {
    pub fn new(
        diagnostic_type: DiagnosticType,
        column: usize,
        line: usize,
        context: String,
    ) -> PointerDiagnostic {
        PointerDiagnostic {
            diagnostic_type,
            column,
            line,
            context,
        }
    }
}

impl Colour for PointerDiagnostic {
    fn colour(&self) -> String {
        format!(
            "{}: Line {}: Column {}: {}",
            match self.diagnostic_type {
                DiagnosticType::Note => "Note".bright_white(),
                DiagnosticType::Warning => "Warning".yellow(),
                DiagnosticType::Error => "Error".red(),
            },
            self.line,
            self.column,
            self.context
        )
    }
}

impl NoColour for PointerDiagnostic {
    #[inline]
    fn no_colour(&self) -> String {
        format!(
            "{:#?}: Line {}: Column {}: {}",
            self.diagnostic_type, self.line, self.column, self.context
        )
    }
}

pub struct HighlightDiagnostic {
    diagnostic_type: DiagnosticType,
    column: usize,
    line: usize,
    width: usize,
    context: String,
}

impl HighlightDiagnostic {
    pub fn new(
        diagnostic_type: DiagnosticType,
        column: usize,
        line: usize,
        width: usize,
        context: String,
    ) -> HighlightDiagnostic {
        HighlightDiagnostic {
            diagnostic_type,
            column,
            line,
            width,
            context: context.to_string(),
        }
    }
}

impl Colour for HighlightDiagnostic {
    fn colour(&self) -> String {
        format!(
            "{}: Line {}: Column {}: {}  (Width: {})",
            match self.diagnostic_type {
                DiagnosticType::Note => "Note".bright_white(),
                DiagnosticType::Warning => "Warning".yellow(),
                DiagnosticType::Error => "Error".red(),
            },
            self.line,
            self.column,
            self.context,
            self.width
        )
    }
}

impl NoColour for HighlightDiagnostic {
    #[inline]
    fn no_colour(&self) -> String {
        format!(
            "{:#?}: {}: {}: {}",
            self.diagnostic_type, self.line, self.column, self.context
        )
    }
}

pub enum Diagnostic {
    Note(NoteDiagnostic),
    Pointer(PointerDiagnostic),
    Highlight(HighlightDiagnostic),
}

impl Type for Diagnostic {
    fn diagnostic_type(&self) -> &DiagnosticType {
        match *self {
            Diagnostic::Note(ref diagnostic) => &diagnostic.diagnostic_type,
            Diagnostic::Pointer(ref diagnostic) => &diagnostic.diagnostic_type,
            Diagnostic::Highlight(ref diagnostic) => &diagnostic.diagnostic_type,
        }
    }
}
