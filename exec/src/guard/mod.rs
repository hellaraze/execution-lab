#[derive(Debug, Default, Clone)]
pub struct ReplayGuard;

impl ReplayGuard {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Default)]
pub struct Guard {
    replay: ReplayGuard,
}

impl Guard {
    pub fn new() -> Self {
        Self {
            replay: ReplayGuard::new(),
        }
    }
}
