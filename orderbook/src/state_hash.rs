use crate::OrderBook;
use blake3::Hasher;

impl OrderBook {
    /// Deterministic hash of the current book state.
    /// NOTE: This assumes invariants have already guaranteed finite values.
    pub fn state_hash64(&self) -> u64 {
        let mut h = Hasher::new();

        // Domain separator
        h.update(b"orderbook:v1|bids|");

        for (p, q) in self.bids.iter() {
            h.update(&p.0.to_le_bytes());
            h.update(&q.to_le_bytes());
        }

        h.update(b"|asks|");

        for (p, q) in self.asks.iter() {
            h.update(&p.0.to_le_bytes());
            h.update(&q.to_le_bytes());
        }

        let out = h.finalize();
        let bytes = out.as_bytes();
        u64::from_le_bytes(bytes[0..8].try_into().unwrap())
    }
}
