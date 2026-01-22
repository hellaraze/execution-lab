use exec_bridge::adapter::adapt;

use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::instrument::InstrumentKey as CoreIK;
use el_core::time::{TimeSource, Timestamp};

fn mk_event(event_type: EventType, payload: EventPayload) -> Event {
    let ts = Timestamp::new(0, TimeSource::Exchange);
    Event {
        id: uuid::Uuid::new_v4(),
        event_type,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        instrument: CoreIK::new(Exchange::Binance, "BTCUSDT"),
        ts_exchange: None,
        ts_recv: ts,
        ts_proc: ts,
        seq: None,
        schema_version: 1,
        integrity_flags: vec![],
        payload,
        meta: Default::default(),
    }
}

#[test]
fn maps_order_ack_to_order_acked() {
    let e = mk_event(
        EventType::OrderAck,
        EventPayload::OrderAck {
            order_id: "42".to_string(),
        },
    );

    let out = adapt(&e).expect("must map");
    match out {
        exec::events::ExecEvent::OrderAcked { instrument, id } => {
            assert_eq!(instrument.exchange, "Binance");
            assert_eq!(instrument.symbol, "BTCUSDT");
            assert_eq!(id.0, 42);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn maps_fill_to_order_fill() {
    let e = mk_event(
        EventType::Fill,
        EventPayload::Fill {
            order_id: "7".to_string(),
            fill_id: "f1".to_string(),
            price: 123.0,
            qty: 0.5,
        },
    );

    let out = adapt(&e).expect("must map");
    match out {
        exec::events::ExecEvent::OrderFill {
            instrument,
            id,
            filled_qty,
            avg_px,
        } => {
            assert_eq!(instrument.exchange, "Binance");
            assert_eq!(instrument.symbol, "BTCUSDT");
            assert_eq!(id.0, 7);
            assert_eq!(filled_qty, 0.5);
            assert_eq!(avg_px, 123.0);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn rejects_non_u64_order_id() {
    let e = mk_event(
        EventType::OrderAck,
        EventPayload::OrderAck {
            order_id: "not-a-number".to_string(),
        },
    );

    assert!(adapt(&e).is_none());
}

#[test]
fn maps_submit_to_order_created() {
    let e = mk_event(
        EventType::OrderSubmit,
        EventPayload::OrderSubmit {
            order_id: "1".to_string(),
            side: "BUY".to_string(),
            price: 1.0,
            qty: 1.0,
        },
    );

    let out = adapt(&e).expect("must map");
    match out {
        exec::events::ExecEvent::OrderCreated { instrument, id } => {
            assert_eq!(instrument.exchange, "Binance");
            assert_eq!(instrument.symbol, "BTCUSDT");
            assert_eq!(id.0, 1);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn maps_cancel_request_to_order_cancel_requested() {
    let e = mk_event(
        EventType::CancelRequest,
        EventPayload::CancelRequest {
            order_id: "2".to_string(),
        },
    );

    let out = adapt(&e).expect("must map");
    match out {
        exec::events::ExecEvent::OrderCancelRequested { instrument, id } => {
            assert_eq!(instrument.exchange, "Binance");
            assert_eq!(instrument.symbol, "BTCUSDT");
            assert_eq!(id.0, 2);
        }
        other => panic!("unexpected: {:?}", other),
    }
}
