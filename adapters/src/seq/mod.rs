#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeqState {
    Healthy,
    Gap { expected: u64, got: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterSignal {
    NeedSnapshot,
}

#[derive(Debug)]
pub struct SeqTracker {
    last: Option<u64>,
}

impl SeqTracker {
    pub fn new() -> Self {
        Self { last: None }
    }

    pub fn observe(&mut self, seq: u64) -> Result<SeqState, AdapterSignal> {
        match self.last {
            None => {
                self.last = Some(seq);
                Ok(SeqState::Healthy)
            }
            Some(prev) if seq == prev + 1 => {
                self.last = Some(seq);
                Ok(SeqState::Healthy)
            }
            Some(_prev) => Err(AdapterSignal::NeedSnapshot),
        }
    }
}

impl SeqTracker {
    pub fn reset(&mut self, last_seq: u64) {
        self.last = Some(last_seq);
    }
}
