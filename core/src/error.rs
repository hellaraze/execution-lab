use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid event data: {0}")]
    InvalidEvent(String),

    #[error("time error: {0}")]
    TimeError(String),

    #[error("serialization error: {0}")]
    Serialization(String),
}
