use crate::{
    err,
    notifier::{self, DiagType, Diagnostic, Highlight},
    token::tokens::Token,
};

token!(Immediate, value: i16);

fn too_large(file: String, column: u64, line: u64, length: usize, value: &str) {
    err!(
        Highlight,
        file,
        column,
        line,
        length,
        format!(
            "Value {} is too large to be represented in signed 16 bits",
            value
        )
    );
}

impl Immediate {
    #[must_use]
    pub fn from_decimal(token: String, file: String, column: u64, line: u64) -> Self {
        let value = token
            .chars()
            .skip(token.chars().position(|c| c == '#').map_or(0, |p| p + 1))
            .collect::<String>()
            .parse::<i16>()
            .unwrap_or_else(|_| {
                too_large(file.clone(), column, line, token.len(), &token);
                0
            });

        Self::new(token, file, column, line, value)
    }

    #[must_use]
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
            too_large(file.clone(), column, line, token.len(), &token);
            0
        }) as i16;

        Self::new(token, file, column, line, value)
    }

    #[must_use]
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
            too_large(file.clone(), column, line, token.len(), &token);
            0
        }) as i16;

        let value = if token.contains('-') { -value } else { value };

        Self::new(token, file, column, line, value)
    }
}
