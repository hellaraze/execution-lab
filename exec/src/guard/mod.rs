#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GuardState {
    #[default]
    Healthy,
    NeedSnapshot,
}

#[derive(Debug, Default)]
pub struct ExecGuard {
    state: GuardState,
}

impl ExecGuard {
    pub fn new() -> Self {
        Self { state: GuardState::Healthy }
    }

    pub fn allow_exec(&self) -> bool {
        matches!(self.state, GuardState::Healthy)
    }

    pub fn on_need_snapshot(&mut self) {
        self.state = GuardState::NeedSnapshot;
    }

    pub fn on_snapshot(&mut self) {
        self.state = GuardState::Healthy;
    }
}
