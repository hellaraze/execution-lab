#[derive(Debug, Default, Clone)]
pub struct ReplayGuard;

impl ReplayGuard {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Default)]
pub struct ExecGuard {
    replay: ReplayGuard,
}

impl ExecGuard {
    pub fn new() -> Self {
        Self {
            replay: ReplayGuard::new(),
        }
    }
}
