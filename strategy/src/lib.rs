//! Strategy crate
//!
//! Canonical rule: strategies must implement `el_contracts::v1::Strategy`
//! and speak only in `MdEvent/ExEvent/ExCommand`.

pub use el_contracts::v1::{ExCommand, ExEvent, MdEvent, Rejection, RiskEngine, Side, Strategy};

/// Minimal no-op strategy example (kept tiny, serves as compile guard).
pub struct NoopStrategy;

impl Strategy for NoopStrategy {
    fn name(&self) -> &'static str {
        "noop"
    }

    fn on_md(&mut self, _ev: &MdEvent) -> Vec<ExCommand> {
        Vec::new()
    }

    fn on_ex(&mut self, _ev: &ExEvent) -> Vec<ExCommand> {
        Vec::new()
    }
}
