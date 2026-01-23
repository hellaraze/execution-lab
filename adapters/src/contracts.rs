//! Canonical contract implementations live here.
//!
//! Rule: `el_contracts` adapter traits are implemented in `adapters` crate only.

use std::collections::VecDeque;

use el_contracts::v1::{ExCommand, ExEvent, ExecutionAdapter, Health, MarketDataAdapter, MdEvent};
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};

use crate::wire;

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

/// Real v1 adapter: Binance bookTicker -> MdEvent::Bbo.
/// Ingest: push raw strings, Output: poll normalized MdEvent.
pub struct BinanceMdAdapterBbo {
    instrument: InstrumentKey,
    q: VecDeque<MdEvent>,
    last_seq: Option<u64>,
}

impl BinanceMdAdapterBbo {
    pub fn new(instrument: InstrumentKey) -> Self {
        Self {
            instrument,
            q: VecDeque::new(),
            last_seq: None,
        }
    }

    pub fn push_raw(&mut self, raw: &str, seq: u64, ts_exchange_ms: u64) {
        if let Some(last) = self.last_seq {
            if seq <= last {
                // idempotency/dup guard (v1 minimal)
                return;
            }
        }

        if let Some(ev) = wire::binance::map_raw_bbo(raw, seq, ts_exchange_ms) {
            if let wire::WirePayload::Bbo {
                bid_px,
                bid_qty,
                ask_px,
                ask_qty,
            } = ev.payload
            {
                self.q.push_back(MdEvent::Bbo {
                    instrument: self.instrument,
                    ts: (Timestamp::new((ts_exchange_ms as i64) * 1_000_000, TimeSource::Exchange)),
                    bid_px,
                    bid_qty,
                    ask_px,
                    ask_qty,
                });
                self.last_seq = Some(seq);
            }
        }
    }
}

impl MarketDataAdapter for BinanceMdAdapterBbo {
    fn health(&self, _instrument: InstrumentKey) -> Health {
        Health::Healthy
    }

    fn poll(&mut self) -> Vec<MdEvent> {
        self.q.drain(..).collect()
    }
}
