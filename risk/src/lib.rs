use el_core::event::Event;

#[derive(Debug, thiserror::Error)]
pub enum RiskError {
    #[error("position limit exceeded")]
    PositionLimit,
    #[error("notional limit exceeded")]
    NotionalLimit,
}

pub struct RiskEngine;

impl RiskEngine {
    pub fn check(_event: &Event) -> Result<(), RiskError> {
        Ok(())
    }
}
