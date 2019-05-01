#[derive(Debug, PartialEq, Clone)]
pub struct Label {
    token: String,
    column: u64,
    line: u64,
}

impl Label {
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
}
