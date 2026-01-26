#[cfg(feature = "replay-ro")]
pub mod ro {
    use el_core::event::{Event, EventPayload, EventType};

    #[derive(Debug, Clone, Copy)]
    pub struct Bbo {
        pub bid: f64,
        pub ask: f64,
    }

    pub fn extract_last_bbo(events: &[Event]) -> Option<Bbo> {
        let mut last: Option<Bbo> = None;

        for e in events {
            if e.event_type == EventType::TickerBbo {
                if let EventPayload::TickerBbo { bid, ask } = &e.payload {
                    last = Some(Bbo {
                        bid: *bid,
                        ask: *ask,
                    });
                }
            }
        }

        last
    }
}

#[cfg(not(feature = "replay-ro"))]
pub mod ro {
    #[derive(Debug, Clone, Copy)]
    pub struct Bbo {
        pub bid: f64,
        pub ask: f64,
    }

    pub fn extract_last_bbo_from_lines<I: IntoIterator<Item = String>>(_lines: I) -> Option<Bbo> {
        None
    }
}
