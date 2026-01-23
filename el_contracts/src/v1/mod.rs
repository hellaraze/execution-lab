//! PHASE A: Frozen contracts (ABI) for execution-lab.
//!
//! Rule: editing these is a BREAKING change. Extend via versioning.

use el_core::instrument::InstrumentKey;
use el_core::time::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Health {
    Healthy,
    Degraded,
    NeedSnapshot,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

/// Normalized market data events (minimal v1).
#[derive(Debug, Clone)]
pub enum MdEvent {
    Bbo {
        instrument: InstrumentKey,
        ts: Timestamp,
        bid_px: f64,
        bid_qty: f64,
        ask_px: f64,
        ask_qty: f64,
    },
    L2Delta {
        instrument: InstrumentKey,
        ts: Timestamp,
        seq: u64,
        side: Side,
        px: f64,
        qty: f64,
    },
    Trade {
        instrument: InstrumentKey,
        ts: Timestamp,
        px: f64,
        qty: f64,
        is_maker: bool,
    },
}

/// Strategy -> execution intent (minimal v1).
#[derive(Debug, Clone)]
pub enum ExCommand {
    Place {
        instrument: InstrumentKey,
        client_order_id: u64,
        side: Side,
        px: f64,
        qty: f64,
    },
    Cancel {
        instrument: InstrumentKey,
        client_order_id: u64,
    },
    CancelAll {
        instrument: InstrumentKey,
    },
}

#[derive(Debug, Clone)]
pub enum Rejection {
    Risk(String),
    Exchange(String),
    Invalid(String),
}

/// Execution facts (adapter -> system).
#[derive(Debug, Clone)]
pub enum ExEvent {
    Accepted {
        instrument: InstrumentKey,
        client_order_id: u64,
        ts: Timestamp,
    },
    Rejected {
        instrument: InstrumentKey,
        client_order_id: u64,
        ts: Timestamp,
        reason: Rejection,
    },
    Fill {
        instrument: InstrumentKey,
        client_order_id: u64,
        ts: Timestamp,
        px: f64,
        qty: f64,
    },
    Canceled {
        instrument: InstrumentKey,
        client_order_id: u64,
        ts: Timestamp,
    },
}

/// Clock/TimeSource contract.
pub trait Clock: Send + Sync {
    fn now(&self) -> Timestamp;
}

/// MarketDataAdapter contract (ingest -> normalized MdEvent stream).
pub trait MarketDataAdapter: Send + Sync {
    fn health(&self, instrument: InstrumentKey) -> Health;
    fn poll(&mut self) -> Vec<MdEvent>;
}

/// ExecutionAdapter contract (commands -> exchange, events <- exchange).
pub trait ExecutionAdapter: Send + Sync {
    fn health(&self, instrument: InstrumentKey) -> Health;
    fn submit(&mut self, cmd: ExCommand);
    fn poll(&mut self) -> Vec<ExEvent>;
}

/// Strategy contract (pure logic).
pub trait Strategy: Send {
    fn name(&self) -> &'static str;
    fn on_md(&mut self, ev: &MdEvent) -> Vec<ExCommand>;
    fn on_ex(&mut self, ev: &ExEvent) -> Vec<ExCommand>;
    fn on_timer(&mut self, _ts: Timestamp) -> Vec<ExCommand> {
        Vec::new()
    }
}

/// RiskEngine contract (gate commands).
pub trait RiskEngine: Send {
    fn name(&self) -> &'static str;
    fn check(&mut self, cmd: &ExCommand) -> Result<(), Rejection>;
}

/// AuditSink contract (append-only trace).
pub trait AuditSink: Send {
    fn name(&self) -> &'static str;
    fn record_md(&mut self, _ev: &MdEvent) {}
    fn record_ex_cmd(&mut self, _cmd: &ExCommand) {}
    fn record_ex_ev(&mut self, _ev: &ExEvent) {}
}
