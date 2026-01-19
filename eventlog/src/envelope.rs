use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub seq: u64,
    pub ts_ns: u64,
    pub stream: String,
    pub kind: String,
    /// base64 bytes in json
    pub payload_b64: String,
    pub checksum: u32,
}
