use contracts_bridge::v1;
use el_core::instrument::InstrumentKey;
use el_core::time::Timestamp;

/// Shadow-contract emission helper.
/// IMPORTANT: no behavior impact; caller may ignore return value.
pub fn shadow_md_event_from_bbo(
    instrument: InstrumentKey,
    ts: Timestamp,
    bid_px: f64,
    bid_qty: f64,
    ask_px: f64,
    ask_qty: f64,
) -> v1::md::MdEvent {
    contracts_bridge::md_bbo(instrument, ts, bid_px, bid_qty, ask_px, ask_qty)
}

/// Shadow-contract emission for strategy decision (gas/no_gas).
/// IMPORTANT: no behavior impact; caller may ignore return value.
pub fn shadow_strategy_decision(
    instrument: InstrumentKey,
    ts: Timestamp,
    decision: v1::strategy::Decision,
    edge_bps: f64,
) -> v1::strategy::StrategyDecision {
    contracts_bridge::strategy_decision(instrument, ts, decision, edge_bps)
}

/// Shadow-contract emission for audit trace.
/// IMPORTANT: no behavior impact; caller may ignore return value.
pub fn shadow_audit_event(
    ts: el_core::time::Timestamp,
    source: &'static str,
    message: &'static str,
) -> contracts_bridge::v1::audit::AuditEvent {
    contracts_bridge::audit_event(ts, source, message)
}
