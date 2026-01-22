#[cfg(test)]
use replay::ReplayGuard;

pub struct ExecGuard {
    replay: ReplayGuard,
}

impl ExecGuard {
    pub fn new() -> Self {
        Self {
            replay: ReplayGuard::new(),
        }
    }

    pub fn allow_exec(&self) -> bool {
        self.replay.allow_event()
    }

    pub fn on_need_snapshot(&mut self) {
        self.replay.on_adapter_signal();
    }

    pub fn on_snapshot(&mut self) {
        self.replay.on_snapshot();
    }
}
