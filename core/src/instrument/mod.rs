use serde::{Deserialize, Serialize};
use std::fmt;

use crate::event::Exchange;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstrumentKey {
    pub exchange: Exchange,
    pub symbol: Symbol,
}

impl InstrumentKey {
    pub fn new(exchange: Exchange, symbol: impl Into<String>) -> Self {
        Self {
            exchange,
            symbol: Symbol(symbol.into()),
        }
    }
}

impl fmt::Display for InstrumentKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.exchange, self.symbol.0)
    }
}
