pub mod binance;

#[derive(Debug, Clone)]
pub struct WireEvent {
    pub source: &'static str,
    pub seq: u64,
    pub ts_exchange: u64,
    pub payload: WirePayload,
}

#[derive(Debug, Clone)]
pub enum WirePayload {
    Depth,
    Trade,
    Bbo,
}
