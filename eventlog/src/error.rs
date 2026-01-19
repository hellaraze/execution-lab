use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventLogError {
    #[error("io error: {0}")]
    Io(String),

    #[error("serialization error: {0}")]
    Serde(String),

    #[error("corrupted log entry")]
    CorruptedEntry,
}
