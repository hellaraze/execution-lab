//! Risk crate
//!
//! Canonical rule: risk engine implements `el_contracts::v1::RiskEngine` and gates `ExCommand`.

use el_contracts::v1::{ExCommand, Rejection, RiskEngine as RiskEngineContract};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    /// Hard cap on order notional (px * qty). None = disabled.
    pub max_order_notional: Option<f64>,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            max_order_notional: None,
        }
    }
}

#[derive(Debug)]
pub struct RiskEngine {
    cfg: RiskConfig,
}

impl RiskEngine {
    pub fn new(cfg: RiskConfig) -> Self {
        Self { cfg }
    }

    fn check_internal(&mut self, cmd: &ExCommand) -> Result<(), String> {
        if let Some(max) = self.cfg.max_order_notional {
            match cmd {
                ExCommand::Place { px, qty, .. } => {
                    let notional = (*px) * (*qty);
                    if notional > max {
                        return Err(format!("max_order_notional exceeded: {notional} > {max}"));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl RiskEngineContract for RiskEngine {
    fn name(&self) -> &'static str {
        "risk::RiskEngine(v1)"
    }

    fn check(&mut self, cmd: &ExCommand) -> Result<(), Rejection> {
        self.check_internal(cmd).map_err(Rejection::Risk)
    }
}
