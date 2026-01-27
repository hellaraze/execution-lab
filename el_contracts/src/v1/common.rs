use std::fmt::Debug;

pub use el_core::instrument::InstrumentKey;
pub use el_core::time::Timestamp;

pub trait ContractEvent: Debug + Send + Sync + 'static {}
