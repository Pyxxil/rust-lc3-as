#[derive(Debug, PartialEq, Clone)]
pub struct Character {
    token: String,
    column: u64,
    line: u64,
}

impl Character {
    pub fn new(token: String, column: u64, line: u64) -> Self {
        Self {
            token,
            column,
            line,
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
}
