#[cfg(test)]
mod tests {
    use super::super::seq::*;
    use el_core::event::{Event, EventPayload, EventType, Exchange};
    use el_core::time::{TimeSource, Timestamp};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn mk(seq: u64) -> Event {
        Event {
            id: Uuid::new_v4(),
            event_type: EventType::BookDelta,
            instrument: el_core::instrument::InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
            exchange: Exchange::Binance,
            symbol: "BTCUSDT".to_string(),
            ts_exchange: None,
            ts_recv: Timestamp::new(0, TimeSource::Receive),
            ts_proc: Timestamp::new(0, TimeSource::Process),
            seq: Some(seq),
            schema_version: 1,
            integrity_flags: vec![],
            payload: EventPayload::BookDelta {
                bids: vec![],
                asks: vec![],
            },
            meta: HashMap::new(),
        }
    }

    #[test]
    fn detects_gap() {
        let mut t = SeqTracker::new();
        assert_eq!(t.observe(&mk(10)).unwrap(), None);
        let g = t.observe(&mk(15)).unwrap().unwrap();
        assert_eq!(g.from, 11);
        assert_eq!(g.to, 14);
    }

    #[test]
    fn detects_regression() {
        let mut t = SeqTracker::new();
        t.observe(&mk(10)).unwrap();
        let e = t.observe(&mk(9)).err().unwrap();
        let msg = format!("{e}");
        assert!(msg.contains("regression"));
    }

    #[test]
    fn ignores_events_without_seq() {
        let mut t = SeqTracker::new();
        let mut e = mk(10);
        e.seq = None;
        assert_eq!(t.observe(&e).unwrap(), None);
    }

    #[test]
    fn duplicate_seq_is_error() {
        let mut t = SeqTracker::new();
        t.observe(&mk(10)).unwrap();
        let e = t.observe(&mk(10)).err().unwrap();
        let msg = format!("{e}");
        assert!(msg.contains("regression"));
    }
}
