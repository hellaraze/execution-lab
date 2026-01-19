use std::collections::HashMap;

use el_core::event::{Event, Exchange};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StreamKey {
    pub exchange: Exchange,
    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gap {
    pub from: u64,
    pub to: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum SeqError {
    #[error("sequence regression for {exchange:?} {symbol}: prev={prev} curr={curr}")]
    Regression {
        exchange: Exchange,
        symbol: String,
        prev: u64,
        curr: u64,
    },
}

#[derive(Debug, Default)]
pub struct SeqTracker {
    last: HashMap<StreamKey, u64>,
}

impl SeqTracker {
    pub fn new() -> Self {
        Self { last: HashMap::new() }
    }

    /// Update tracker with an event.
    /// Returns:
    /// - Ok(Some(Gap)) if a gap is detected (prev+1..curr-1)
    /// - Ok(None) if ok or event has no seq
    /// - Err(SeqError) if seq regressed
    pub fn observe(&mut self, ev: &Event) -> Result<Option<Gap>, SeqError> {
        let curr = match ev.seq {
            Some(s) => s,
            None => return Ok(None),
        };

        let key = StreamKey {
            exchange: ev.exchange.clone(),
            symbol: ev.symbol.clone(),
        };

        match self.last.get(&key).copied() {
            None => {
                self.last.insert(key, curr);
                Ok(None)
            }
            Some(prev) => {
                if curr <= prev {
                    return Err(SeqError::Regression {
                        exchange: ev.exchange.clone(),
                        symbol: ev.symbol.clone(),
                        prev,
                        curr,
                    });
                }

                self.last.insert(key, curr);

                if curr > prev + 1 {
                    Ok(Some(Gap { from: prev + 1, to: curr - 1 }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub fn last_seq(&self, exchange: &Exchange, symbol: &str) -> Option<u64> {
        self.last
            .get(&StreamKey {
                exchange: exchange.clone(),
                symbol: symbol.to_string(),
            })
            .copied()
    }
}
