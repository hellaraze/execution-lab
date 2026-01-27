pub mod fees;
pub mod multileg;
pub mod replay_ro;
pub mod signal;
pub mod spread;

pub use fees::Fees;
pub use multileg::{compute_multileg, Leg, MultiLegSignal};
pub use signal::{compute_signal, DecisionReason, GasDecision, Signal, Thresholds};
pub use spread::SpreadInput;

// replay(ro)
pub use replay_ro::ro;
pub mod obs;

pub mod contracts_shadow;
