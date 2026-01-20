use el_core::event::{EventPayload, EventType};

#[derive(Debug, Clone)]
pub struct OrderEvent {
    pub event_type: EventType,
    pub payload: EventPayload,
}

impl OrderEvent {
    pub fn new(event_type: EventType, payload: EventPayload) -> Self {
        Self { event_type, payload }
    }
}
