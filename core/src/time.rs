use serde::{Deserialize, Serialize};

/// Unix time в наносекундах
pub type UnixNanos = i64;

/// Источник времени
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeSource {
    Exchange,
    Receive,
    Process,
}

/// Унифицированная временная метка события
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp {
    pub nanos: UnixNanos,
    pub source: TimeSource,
}

impl Timestamp {
    pub fn new(nanos: UnixNanos, source: TimeSource) -> Self {
        Self { nanos, source }
    }
}
