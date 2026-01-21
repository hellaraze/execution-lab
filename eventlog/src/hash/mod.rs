use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn stable_hash<T: Hash>(value: &T) -> u64 {
    let mut h = DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}
