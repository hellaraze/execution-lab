use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct WireEvent {
    pub event_type: String,
    pub payload: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct BookLevels {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}
