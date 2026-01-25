pub mod fees;
pub mod signal;
pub mod spread;

pub use fees::Fees;
pub use signal::{compute_signal, GasDecision, Signal, Thresholds};
pub use spread::SpreadInput;
