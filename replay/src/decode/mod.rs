pub mod from_wire;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("missing field: {0}")]
    Missing(&'static str),

    #[error("invalid field: {0}")]
    Invalid(&'static str),

    #[error("unsupported event_type: {0}")]
    Unsupported(String),

    #[error("payload key mismatch for event_type={event_type}: expected key={expected}")]
    PayloadKeyMismatch {
        event_type: String,
        expected: &'static str,
    },

    #[error("sequence regression: prev={prev} curr={curr}")]
    SeqRegression { prev: u64, curr: u64 },
}
