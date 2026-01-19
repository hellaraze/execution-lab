use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub seq: u64,
    pub ts_ns: u64,
    pub stream: String,
    pub kind: String,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
    pub checksum: u32,
}
