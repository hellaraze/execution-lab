//! Canonical contract implementations live here (PHASE A).
//!
//! Rule: `el_contracts` adapter traits are implemented in `adapters` crate only.

use el_contracts::v1::{ExCommand, ExEvent, ExecutionAdapter, Health, MarketDataAdapter, MdEvent};
use el_core::instrument::InstrumentKey;

#[derive(Default)]
pub struct NoopMdAdapter;

impl MarketDataAdapter for NoopMdAdapter {
    fn health(&self, _instrument: InstrumentKey) -> Health {
        Health::Healthy
    }

    fn poll(&mut self) -> Vec<MdEvent> {
        Vec::new()
    }
}

#[derive(Default)]
pub struct NoopExecAdapter;

impl ExecutionAdapter for NoopExecAdapter {
    fn health(&self, _instrument: InstrumentKey) -> Health {
        Health::Healthy
    }

    fn submit(&mut self, _cmd: ExCommand) {}

    fn poll(&mut self) -> Vec<ExEvent> {
        Vec::new()
    }
}
