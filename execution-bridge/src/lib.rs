use anyhow::Result;
use el_core::event::{ExecEvent, EventId};
use eventlog::EventLogWriter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(pub EventId);

pub trait ExecOutbox {
    fn publish_once(&mut self, ev: ExecEvent) -> Result<()>;
}

pub struct Bridge {
    writer: EventLogWriter,
}

impl Bridge {
    pub fn new(writer: EventLogWriter) -> Self {
        Self { writer }
    }
}

impl ExecOutbox for Bridge {
    fn publish_once(&mut self, ev: ExecEvent) -> Result<()> {
        // ЖБ-контракт:
        // - writer.append MUST be idempotent by EventId
        // - duplicate EventId -> NO-OP
        self.writer.append(ev)?;
        Ok(())
    }
}
