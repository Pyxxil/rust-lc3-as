use std::collections::HashMap;

use crate::token::Symbol;

pub type SymbolTable = HashMap<String, Symbol>;
pub type Listing = (u16, String);
pub type Listings = Vec<Listing>;
pub type Program = (SymbolTable, Listings);
