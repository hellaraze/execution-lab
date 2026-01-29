pub mod binance_live;
pub mod normalized;
pub mod seq;
pub mod wire;
use normalized::{NormalizedEvent, NormalizedKind};
use wire::WireEvent;

pub use seq::{AdapterSignal, SeqState, SeqTracker};

pub fn adapt(event: WireEvent) -> NormalizedEvent {
    NormalizedEvent {
        seq: event.seq,
        ts: event.ts_exchange,
        kind: match event.payload {
            wire::WirePayload::Depth => NormalizedKind::Depth,
            wire::WirePayload::Trade => NormalizedKind::Trade,
            wire::WirePayload::Bbo => NormalizedKind::Bbo,
        },
    }
}
