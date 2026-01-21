#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayHealth {
    Healthy,
    NeedSnapshot,
}
