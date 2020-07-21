#[derive(Debug)]
pub struct Symbol {
    address: u16,
    symbol: String,
}

impl Symbol {
    #[must_use]
    pub fn new(address: u16, symbol: String) -> Self {
        Self { address, symbol }
    }

    #[must_use]
    pub fn address(&self) -> u16 {
        self.address
    }

    #[must_use]
    pub fn symbol(&self) -> &String {
        &self.symbol
    }
}
