use crate::hash::stable_hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Snapshot<T> {
    pub state: T,
    pub hash: u64,
}

impl<T: std::hash::Hash> Snapshot<T> {
    pub fn new(state: T) -> Self {
        let hash = stable_hash(&state);
        Self { state, hash }
    }

    pub fn assert_same(&self, other: &Self) {
        if self.hash != other.hash {
            panic!("SNAPSHOT HASH MISMATCH");
        }
    }
}
