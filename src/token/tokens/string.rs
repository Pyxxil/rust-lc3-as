use std::string;

#[derive(Debug, PartialEq, Clone)]
pub struct String {
    token: string::String,
    column: u64,
    line: u64,
    file: string::String,
}

impl String {
    #[must_use]
    pub fn new(token: string::String, file: string::String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
            file,
        }
    }

    #[must_use]
    pub fn token(&self) -> &string::String {
        &self.token
    }

    #[must_use]
    pub fn column(&self) -> u64 {
        self.column
    }

    #[must_use]
    pub fn line(&self) -> u64 {
        self.line
    }

    #[must_use]
    pub fn file(&self) -> &string::String {
        &self.file
    }
}
