pub mod ro {
    #[derive(Debug, Clone, Copy)]
    pub struct Bbo {
        pub bid: f64,
        pub ask: f64,
    }

    #[cfg(feature = "replay-ro")]
    fn valid(bid: f64, ask: f64) -> bool {
        bid.is_finite() && ask.is_finite() && bid > 0.0 && ask > 0.0 && bid <= ask
    }

    #[cfg(feature = "replay-ro")]
    pub fn extract_last_bbo_for(
        events: &[el_core::event::Event],
        instrument: el_core::instrument::InstrumentKey,
    ) -> Option<Bbo> {
        use el_core::event::{EventPayload, EventType};

        let mut last: Option<Bbo> = None;

        for e in events {
            if e.instrument != instrument {
                continue;
            }

            match (&e.event_type, &e.payload) {
                (EventType::TickerBbo, EventPayload::TickerBbo { bid, ask }) => {
                    if valid(*bid, *ask) {
                        last = Some(Bbo { bid: *bid, ask: *ask });
                    }
                }
                (EventType::BookSnapshot, EventPayload::BookSnapshot { bids, asks }) => {
                    if let (Some((bid, _)), Some((ask, _))) = (bids.first(), asks.first()) {
                        if valid(*bid, *ask) {
                            last = Some(Bbo { bid: *bid, ask: *ask });
                        }
                    }
                }
                _ => {}
            }
        }

        last
    }

    #[cfg(feature = "replay-ro")]
    pub fn extract_last_bbo(events: &[el_core::event::Event]) -> Option<Bbo> {
        // ULTRA-JB: refuse ambiguous multi-instrument logs.
        let mut inst: Option<el_core::instrument::InstrumentKey> = None;
        for e in events {
            match &inst {
                None => inst = Some(e.instrument.clone()),
                Some(x) => {
                    if e.instrument != *x {
                        return None;
                    }
                }
            }
        }
        let inst = inst?;
        extract_last_bbo_for(events, inst)
    }

    #[cfg(not(feature = "replay-ro"))]
    pub fn extract_last_bbo<T>(_events: &[T]) -> Option<Bbo> {
        None
    }

    #[cfg(not(feature = "replay-ro"))]
    pub fn extract_last_bbo_for<T>(_events: &[T], _instrument: ()) -> Option<Bbo> {
        None
    }
}
