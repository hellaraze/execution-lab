pub mod ro {
    #[derive(Debug, Clone, Copy)]
    pub struct Bbo {
        pub bid: f64,
        pub ask: f64,
    }

    #[cfg(feature = "replay-ro")]
    pub fn extract_last_bbo(events: &[el_core::event::Event]) -> Option<Bbo> {
        use el_core::event::{EventPayload, EventType};

        let mut last: Option<Bbo> = None;

        for e in events {
            match (&e.event_type, &e.payload) {
                (EventType::TickerBbo, EventPayload::TickerBbo { bid, ask }) => {
                    last = Some(Bbo {
                        bid: *bid,
                        ask: *ask,
                    });
                }
                (EventType::BookSnapshot, EventPayload::BookSnapshot { bids, asks }) => {
                    if let (Some((bid, _)), Some((ask, _))) = (bids.first(), asks.first()) {
                        last = Some(Bbo {
                            bid: *bid,
                            ask: *ask,
                        });
                    }
                }
                _ => {}
            }
        }

        last
    }

    #[cfg(not(feature = "replay-ro"))]
    pub fn extract_last_bbo<T>(_events: &[T]) -> Option<Bbo> {
        None
    }
}
