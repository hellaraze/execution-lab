pub use el_contracts::v1;

use el_core::instrument::InstrumentKey;
use el_core::time::Timestamp;

pub fn md_bbo(
    instrument: InstrumentKey,
    ts: Timestamp,
    bid_px: f64,
    bid_qty: f64,
    ask_px: f64,
    ask_qty: f64,
) -> v1::md::MdEvent {
    v1::md::MdEvent {
        instrument,
        ts,
        bbo: v1::md::Bbo {
            bid_px,
            bid_qty,
            ask_px,
            ask_qty,
        },
    }
}

pub fn strategy_decision(
    instrument: InstrumentKey,
    ts: Timestamp,
    decision: v1::strategy::Decision,
    edge_bps: f64,
) -> v1::strategy::StrategyDecision {
    v1::strategy::StrategyDecision {
        instrument,
        ts,
        decision,
        edge_bps,
    }
}

pub fn risk_decision(
    instrument: InstrumentKey,
    ts: Timestamp,
    verdict: v1::risk::RiskVerdict,
    reason: &'static str,
) -> v1::risk::RiskDecision {
    v1::risk::RiskDecision {
        instrument,
        ts,
        verdict,
        reason,
    }
}

pub fn audit_event(
    ts: Timestamp,
    source: &'static str,
    message: &'static str,
) -> v1::audit::AuditEvent {
    v1::audit::AuditEvent {
        ts,
        source,
        message,
    }
}
