use el_core::event::Event;

#[derive(Debug, thiserror::Error)]
pub enum RiskError {
    #[error("position limit exceeded")]
    PositionLimit,
    #[error("notional limit exceeded")]
    NotionalLimit,
}

#[derive(Clone, Debug)]
pub struct RiskConfig {
    pub max_pos: f64,
    pub max_notional: f64,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            max_pos: f64::INFINITY,
            max_notional: f64::INFINITY,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RiskEngine {
    cfg: RiskConfig,
}

impl RiskEngine {
    pub fn new(cfg: RiskConfig) -> Self {
        Self { cfg }
    }

    pub fn cfg(&self) -> &RiskConfig {
        &self.cfg
    }

    // Phase5 step 1: contract only. We will fold ExecEvents later.
    pub fn check(&self, _event: &Event) -> Result<(), RiskError> {
        Ok(())
    }
}
