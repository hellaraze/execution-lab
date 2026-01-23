//! Canonical contract implementations live here.
//!
//! Rule: `el_contracts` adapter traits are implemented in `adapters` crate only.

use std::collections::{HashMap, VecDeque};

use el_contracts::v1::{ExCommand, ExEvent, ExecutionAdapter, Health, MarketDataAdapter, MdEvent};
use el_core::event::Exchange;
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

/// Real v1 adapter: Binance bookTicker -> MdEvent::Bbo (single-instrument wrapper).
/// Assumption: caller feeds only that instrument's raws (symbol is ignored).
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
                return;
            }
        }

        if let Some(ev) = wire::binance::map_raw_bbo(raw, seq, ts_exchange_ms) {
            if let wire::WirePayload::Bbo {
                bid_px,
                bid_qty,
                ask_px,
                ask_qty,
                ..
            } = ev.payload
            {
                self.q.push_back(MdEvent::Bbo {
                    instrument: self.instrument.clone(),
                    ts: Timestamp::new((ts_exchange_ms as i64) * 1_000_000, TimeSource::Exchange),
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

/// Multi-instrument mux: derives InstrumentKey from symbol in raw bookTicker.
/// Dedup is per-symbol (per-instrument).
pub struct BinanceMdMuxAdapterBbo {
    q: VecDeque<MdEvent>,
    last_seq: HashMap<String, u64>,
}

impl Default for BinanceMdMuxAdapterBbo {
    fn default() -> Self {
        Self {
            q: VecDeque::new(),
            last_seq: HashMap::new(),
        }
    }
}

impl BinanceMdMuxAdapterBbo {
    pub fn push_raw(&mut self, raw: &str, seq: u64, ts_exchange_ms: u64) {
        let ev = match wire::binance::map_raw_bbo(raw, seq, ts_exchange_ms) {
            Some(v) => v,
            None => return,
        };

        let (symbol, bid_px, bid_qty, ask_px, ask_qty) = match ev.payload {
            wire::WirePayload::Bbo {
                symbol,
                bid_px,
                bid_qty,
                ask_px,
                ask_qty,
            } => (symbol, bid_px, bid_qty, ask_px, ask_qty),
            _ => return,
        };

        if let Some(last) = self.last_seq.get(&symbol) {
            if seq <= *last {
                return;
            }
        }

        let instrument = InstrumentKey::new(Exchange::Binance, symbol.as_str());

        self.q.push_back(MdEvent::Bbo {
            instrument,
            ts: Timestamp::new((ts_exchange_ms as i64) * 1_000_000, TimeSource::Exchange),
            bid_px,
            bid_qty,
            ask_px,
            ask_qty,
        });

        self.last_seq.insert(symbol, seq);
    }
}

impl MarketDataAdapter for BinanceMdMuxAdapterBbo {
    fn health(&self, _instrument: InstrumentKey) -> Health {
        Health::Healthy
    }

    fn poll(&mut self) -> Vec<MdEvent> {
        self.q.drain(..).collect()
    }
}
