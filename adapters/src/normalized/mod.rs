#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedEvent {
    pub seq: u64,
    pub ts: u64,
    pub kind: NormalizedKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizedKind {
    Depth,
    Trade,
    Bbo,
}
