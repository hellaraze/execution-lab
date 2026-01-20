#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    New,
    Placed,
    Accepted,
    PartiallyFilled,
    Filled,
    Canceled,
}
