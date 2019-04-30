#[derive(Debug)]
pub struct Symbol {
    address: u16,
    symbol: String,
}

impl Symbol {
    pub fn new(address: u16, symbol: String) -> Self {
        Self { address, symbol }
    }

    pub fn address(&self) -> u16 {
        self.address
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }
}
