use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hash32(pub [u8; 32]);

impl Hash32 {
    pub const ZERO: Hash32 = Hash32([0u8; 32]);
}

pub fn hash_bytes(data: &[u8]) -> Hash32 {
    let mut h = Sha256::new();
    h.update(data);
    Hash32(h.finalize().into())
}

/// Hash-chain: H_i = sha256(H_{i-1} || payload)
pub fn chain_step(prev: Hash32, payload: &[u8]) -> Hash32 {
    let mut h = Sha256::new();
    h.update(prev.0);
    h.update(payload);
    Hash32(h.finalize().into())
}
