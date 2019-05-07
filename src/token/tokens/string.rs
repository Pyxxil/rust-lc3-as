use std::string;

#[derive(Debug, PartialEq, Clone)]
pub struct String {
    token: string::String,
    column: u64,
    line: u64,
    file: string::String,
}

impl String {
    pub fn new(token: string::String, file: string::String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
            file,
        }
    }

    pub fn token(&self) -> &string::String {
        &self.token
    }

    pub fn column(&self) -> u64 {
        self.column
    }

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn file(&self) -> &string::String {
        &self.file
    }
}
