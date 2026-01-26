#[cfg(feature = "replay-ro")]
use d2::ro::extract_last_bbo;

#[cfg(feature = "replay-ro")]
use el_core::event::{Event, EventPayload, EventType, Exchange};

#[cfg(feature = "replay-ro")]
use el_core::instrument::InstrumentKey;

#[cfg(feature = "replay-ro")]
use el_core::time::{TimeSource, Timestamp};

#[cfg(feature = "replay-ro")]
use std::collections::HashMap;

#[cfg(feature = "replay-ro")]
use uuid::Uuid;

#[cfg(feature = "replay-ro")]
fn mk_event(
    id: Uuid,
    exchange: Exchange,
    symbol: &str,
    instrument: InstrumentKey,
    payload: EventPayload,
) -> Event {
    let t0 = Timestamp::new(0, TimeSource::Receive);

    Event {
        id,
        event_type: EventType::TickerBbo,
        exchange,
        symbol: symbol.to_string(),
        instrument,
        ts_exchange: None,
        ts_recv: t0,
        ts_proc: t0,
        seq: None,
        schema_version: 1,
        integrity_flags: vec![],
        payload,
        meta: HashMap::new(),
    }
}

#[cfg(feature = "replay-ro")]
#[test]
fn extracts_last_ticker_bbo() {
    let ex = Exchange::Binance;
    let id1 = Uuid::from_u128(1);
    let id2 = Uuid::from_u128(2);

    let i = InstrumentKey::new(ex.clone(), "BTCUSDT");

    let e1 = mk_event(
        id1,
        ex.clone(),
        "BTCUSDT",
        i.clone(),
        EventPayload::TickerBbo { bid: 1.0, ask: 2.0 },
    );

    let e2 = mk_event(
        id2,
        ex,
        "BTCUSDT",
        i,
        EventPayload::TickerBbo { bid: 3.0, ask: 4.0 },
    );

    let last = extract_last_bbo(&[e1, e2]).expect("bbo");
    assert_eq!(last.bid, 3.0);
    assert_eq!(last.ask, 4.0);
}
