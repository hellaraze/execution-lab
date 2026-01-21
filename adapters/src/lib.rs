pub mod wire;
pub mod normalized;
pub mod seq;

use wire::WireEvent;
use normalized::{NormalizedEvent, NormalizedKind};

pub use seq::{SeqTracker, SeqState, AdapterSignal};

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
