pub mod state;
pub mod wire;
pub mod decode;
pub mod quality;

use state::ReplayHealth;

pub struct ReplayGuard {
    pub health: ReplayHealth,
}

impl ReplayGuard {
    pub fn new() -> Self {
        Self { health: ReplayHealth::Healthy }
    }

    pub fn on_adapter_signal(&mut self) {
        self.health = ReplayHealth::NeedSnapshot;
    }

    pub fn allow_event(&self) -> bool {
        self.health == ReplayHealth::Healthy
    }

    pub fn on_snapshot(&mut self) {
        self.health = ReplayHealth::Healthy;
    }
}
