use notifier;
use notifier::{DiagType, Diagnostic, Highlight};

#[derive(Debug, PartialEq, Clone)]
pub struct Immediate {
    token: String,
    column: u64,
    line: u64,
    file: String,
    pub value: i16,
}

impl Immediate {
    pub fn from_decimal(token: String, file: String, column: u64, line: u64) -> Self {
        let value = token
            .chars()
            .skip(token.chars().position(|c| c.is_digit(10)).unwrap())
            .collect::<String>()
            .parse::<i16>()
            .unwrap_or_else(|_| {
                notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    file.clone(),
                    column,
                    line,
                    token.len(),
                    format!(
                        "Value {} is too large to be represented in signed 16 bits",
                        token
                    ),
                )));
                0
            });

        let value = if token.find('-').is_some() {
            -value
        } else {
            value
        };

        Self {
            token,
            column,
            line,
            file,
            value,
        }
    }

    pub fn from_hexadecimal(token: String, file: String, column: u64, line: u64) -> Self {
        let value = u16::from_str_radix(
            token
                .chars()
                .skip(
                    1 + token
                        .chars()
                        .position(|c| c.to_ascii_uppercase() == 'X')
                        .unwrap(),
                )
                .collect::<String>()
                .as_ref(),
            16,
        )
        .unwrap_or_else(|_| {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                file.clone(),
                column,
                line,
                token.len(),
                format!(
                    "Value {} is too large to be represented in signed 16 bits",
                    token
                ),
            )));
            0
        }) as i16;
        Self {
            token,
            column,
            line,
            file,
            value,
        }
    }

    pub fn from_binary(token: String, file: String, column: u64, line: u64) -> Self {
        let value = u16::from_str_radix(
            token
                .chars()
                .skip(
                    1 + token
                        .chars()
                        .position(|c| c.to_ascii_uppercase() == 'B')
                        .unwrap(),
                )
                .collect::<String>()
                .as_ref(),
            2,
        )
        .unwrap_or_else(|_| {
            notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
                DiagType::Error,
                file.clone(),
                column,
                line,
                token.len(),
                format!(
                    "Value {} is too large to be represented in signed 16 bits",
                    token
                ),
            )));
            0
        }) as i16;

        let value = if token.find('-').is_some() {
            -value
        } else {
            value
        };

        Self {
            token,
            column,
            line,
            file,
            value,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn column(&self) -> u64 {
        self.column
    }

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn file(&self) -> &String {
        &self.file
    }
}
