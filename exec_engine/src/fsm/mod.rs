#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    New,
    Open,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderEvent {
    Create,
    Accept,
    Fill { qty: f64 },
    Cancel,
    Reject,
}

pub fn apply(state: OrderState, ev: &OrderEvent) -> OrderState {
    match (state, ev) {
        (OrderState::New, OrderEvent::Accept) => OrderState::Open,
        (OrderState::Open, OrderEvent::Fill { .. }) => OrderState::PartiallyFilled,
        (OrderState::PartiallyFilled, OrderEvent::Fill { .. }) => OrderState::Filled,
        (_, OrderEvent::Cancel) => OrderState::Canceled,
        (_, OrderEvent::Reject) => OrderState::Rejected,
        (s, _) => s,
    }
}
