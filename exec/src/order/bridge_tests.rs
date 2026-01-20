#[cfg(test)]
mod tests {
    use crate::order::{build_snapshot, to_exec_event};
    use el_core::event::{Event, EventPayload, EventType, Exchange};
    use el_core::time::{Timestamp, TimeSource};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn mk(ev_type: EventType, payload: EventPayload) -> Event {
        Event {
            id: Uuid::new_v4(),
            event_type: ev_type,
            exchange: Exchange::Binance,
            symbol: "BTCUSDT".to_string(),
            ts_exchange: None,
            ts_recv: Timestamp::new(0, TimeSource::Receive),
            ts_proc: Timestamp::new(0, TimeSource::Process),
            seq: None,
            schema_version: 1,
            integrity_flags: vec![],
            payload,
            meta: HashMap::new(),
        }
    }

    #[test]
    fn bridge_snapshot_hash_is_deterministic() {
        let order_id = "order-abc";

        let core_events = vec![
            mk(EventType::OrderSubmit, EventPayload::OrderSubmit {
                order_id: order_id.into(),
                side: "Buy".into(),
                price: 100.0,
                qty: 3.0,
            }),
            mk(EventType::OrderAck, EventPayload::OrderAck { order_id: order_id.into() }),
            mk(EventType::Fill, EventPayload::Fill {
                order_id: order_id.into(),
                fill_id: "f1".into(),
                price: 100.0,
                qty: 1.0,
            }),
            mk(EventType::Fill, EventPayload::Fill {
                order_id: order_id.into(),
                fill_id: "f2".into(),
                price: 110.0,
                qty: 2.0,
            }),
            mk(EventType::CancelRequest, EventPayload::CancelRequest { order_id: order_id.into() }),
            mk(EventType::CancelAck, EventPayload::CancelAck { order_id: order_id.into() }),
        ];

        let mut exec_events = Vec::new();
        for e in &core_events {
            if let Some(x) = to_exec_event(e).unwrap() {
                exec_events.push(x);
            }
        }

        let (_s1, h1) = build_snapshot(&exec_events).unwrap();
        let (_s2, h2) = build_snapshot(&exec_events).unwrap();
        assert_eq!(h1, h2);
    }
}
